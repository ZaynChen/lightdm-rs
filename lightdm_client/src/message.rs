// SPDX-FileCopyrightText: 2026 ZaynChen <zaynchen@qq.com>
//
// SPDX-License-Identifier: MIT

use std::io::{PipeReader, Read};

use super::GreeterError;

#[derive(Debug, Clone, Copy)]
pub enum ServerMessageType {
    Connected = 0,
    PromptAuthentication = 1,
    EndAuthentication = 2,
    SessionResult = 3,
    SharedDirResult = 4,
    Idle = 5,
    Reset = 6,
    ConnectedV2 = 7,
}

impl TryFrom<u32> for ServerMessageType {
    type Error = glib::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Connected),
            1 => Ok(Self::PromptAuthentication),
            2 => Ok(Self::EndAuthentication),
            3 => Ok(Self::SessionResult),
            4 => Ok(Self::SharedDirResult),
            5 => Ok(Self::Idle),
            6 => Ok(Self::Reset),
            7 => Ok(Self::ConnectedV2),
            _ => Err(glib::Error::new(
                GreeterError::Communication,
                "invalid server message type",
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GreeterMessageType {
    Connect = 0,
    Authenticate = 1,
    AuthenticateAsGuest = 2,
    ContinueAuthentication = 3,
    StartSession = 4,
    CancelAuthentication = 5,
    SetLanguage = 6,
    AuthenticateRemote = 7,
    EnsureSharedDir = 8,
}

pub(super) struct GreeterMessage {
    buf: Vec<u8>,
}

impl GreeterMessage {
    pub(super) fn builder(id: GreeterMessageType) -> GreeterMessageBuilder {
        GreeterMessageBuilder::new(id)
    }

    pub(super) fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

pub(super) struct GreeterMessageBuilder {
    id: GreeterMessageType,
    buf: Vec<u8>,
}

impl GreeterMessageBuilder {
    fn new(id: GreeterMessageType) -> Self {
        Self {
            id,
            buf: Vec::new(),
        }
    }

    pub(super) fn arg_u32(mut self, value: u32) -> Self {
        self.buf.extend(value.to_be_bytes());
        self
    }

    pub(super) fn arg_string(mut self, value: String) -> Self {
        self.buf.extend((value.len() as u32).to_be_bytes());
        self.buf.extend(value.into_bytes());
        self
    }

    pub(super) fn build(self) -> GreeterMessage {
        let buf = [
            &(self.id as u32).to_be_bytes(),
            &(self.buf.len() as u32).to_be_bytes(),
            self.buf.as_slice(),
        ]
        .concat();
        GreeterMessage { buf }
    }
}

#[derive(Debug)]
pub(super) struct ServerMessage {
    pub(super) id: ServerMessageType,
    buf: Vec<u8>,
}

impl ServerMessage {
    pub(super) fn from_reader(reader: &mut PipeReader) -> Result<Self, glib::Error> {
        let id = {
            let mut buf = [0; 4];
            reader
                .read_exact(&mut buf)
                .map_err(|e| glib::Error::new(GreeterError::Communication, &e.to_string()))?;
            ServerMessageType::try_from(u32::from_be_bytes(buf))?
        };
        let length = {
            let mut buf = [0; 4];
            reader
                .read_exact(&mut buf)
                .map_err(|e| glib::Error::new(GreeterError::Communication, &e.to_string()))?;
            u32::from_be_bytes(buf) as usize
        };
        let mut buf = vec![0; length];
        reader
            .read_exact(&mut buf)
            .map_err(|e| glib::Error::new(GreeterError::Communication, &e.to_string()))?;
        glib::g_debug!("", "Read {} bytes from daemon", buf.len() + 8);
        Ok(Self { id, buf })
    }

    pub(super) fn reader(&self) -> ServerMessageReader {
        ServerMessageReader::new(self.buf.clone())
    }
}

pub(super) struct ServerMessageReader {
    data: Vec<u8>,
}

impl ServerMessageReader {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub(super) fn read_u32(&mut self) -> u32 {
        if self.data.len() < 4 {
            glib::g_warning!(
                "",
                "Not enough space for u32, need 4, got {}",
                self.data.len()
            );
            0
        } else {
            u32::from_be_bytes(self.data.drain(0..4).as_slice().try_into().unwrap())
        }
    }

    pub(super) fn read_string(&mut self) -> Option<String> {
        let length = self.read_u32() as usize;
        if self.data.len() < length {
            glib::g_warning!(
                "",
                "ServerMessageReader: Not enough space for string ,need {length}, got {}",
                self.data.len()
            );
            None
        } else {
            String::from_utf8(self.data.drain(0..length).collect()).ok()
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}
