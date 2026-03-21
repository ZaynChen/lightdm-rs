// SPDX-FileCopyrightText: 2026 ZaynChen <zaynchen@qq.com>
//
// SPDX-License-Identifier: MIT

use gio::prelude::*;
use glib::{
    self, Properties, SourceId,
    subclass::{Signal, prelude::*},
};

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    env,
    io::{PipeReader, PipeWriter, Write},
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd},
        unix::net::UnixStream,
    },
    rc::Rc,
    sync::OnceLock,
};

use super::message::{GreeterMessage, GreeterMessageType, ServerMessage, ServerMessageType};
use super::{GreeterError, Request};

const API_VERSION: u32 = 1;

pub const LIGHTDM_PROMPT_TYPE_VISIBLE: u32 = 0;
pub const LIGHTDM_PROMPT_TYPE_SECRET: u32 = 1;
pub const LIGHTDM_MESSAGE_TYPE_INFO: u32 = 0;
pub const LIGHTDM_MESSAGE_TYPE_ERROR: u32 = 1;

#[derive(Default, Properties)]
#[properties(wrapper_type = super::Greeter)]
pub struct Greeter {
    /// API version the daemon is using
    api_version: RefCell<u32>,
    /// true if the daemon can reuse this greeter
    #[property(get, set = Self::set_resettable)]
    pub resettable: RefCell<bool>,
    /// Socket connection to daemon
    // socket: RefCell<Option<gio::Socket>>,
    socket: RefCell<Option<UnixStream>>,
    /// Channel to write to daemon
    to_server_channel: RefCell<Option<PipeWriter>>,
    /// Channel to read from daemon
    from_server_channel: RefCell<Option<PipeReader>>,
    from_server_watch: RefCell<Option<glib::SourceId>>,

    pub(super) n_responses_waiting: RefCell<usize>,
    pub(super) responses_received: RefCell<Vec<String>>,

    /// true if have got a connect response
    pub(super) connected: RefCell<bool>,

    /// Pending connect requests
    pub(super) connect_requests: RefCell<VecDeque<Rc<Request<()>>>>,

    /// Pending start session requests
    pub(super) start_session_requests: RefCell<VecDeque<Rc<Request<()>>>>,

    /// Pending ensure shared data dir requests
    pub(super) ensure_shared_data_dir_requests: RefCell<VecDeque<Rc<Request<String>>>>,

    /// Hints provided by the daemon
    pub(super) hints: RefCell<HashMap<String, String>>,

    /// Timeout source to notify greeter to autologin
    pub(super) autologin_timeout: RefCell<Option<SourceId>>,

    #[property(get)]
    pub authentication_user: RefCell<Option<String>>,
    #[property(get)]
    pub in_authentication: RefCell<bool>,
    #[property(get)]
    pub is_authenticated: RefCell<bool>,
    pub(super) authenticate_sequence_number: RefCell<u32>,
    pub(super) cancelling_authentication: RefCell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Greeter {
    const NAME: &'static str = "LightDMGreeter";
    type Type = super::Greeter;
    type ParentType = glib::Object;
}

#[glib::derived_properties]
impl ObjectImpl for Greeter {
    fn signals() -> &'static [Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![
                Signal::builder("authentication-complete").build(),
                Signal::builder("autologin-timer-expired").build(),
                Signal::builder("idle").build(),
                Signal::builder("reset").build(),
                Signal::builder("show-message")
                    .param_types([str::static_type(), u32::static_type()])
                    .build(),
                Signal::builder("show-prompt")
                    .param_types([str::static_type(), u32::static_type()])
                    .build(),
            ]
        })
    }
}

impl Greeter {
    ///
    /// @resettable: Whether the greeter wants to be reset instead of killed after the user logs in
    ///
    /// Set whether the greeter will be reset instead of killed after the user lgs in,
    /// This must be called before lightdm_greeter_connect is called.
    fn set_resettable(&self, resettable: bool) {
        if *self.connected.borrow() {
            self.resettable.replace(resettable);
        }
    }

    fn connect_to_daemon(&self) -> Result<(), glib::Error> {
        if self.to_server_channel.borrow().is_some() && self.from_server_channel.borrow().is_some()
        {
            return Ok(());
        }

        let (to_fd, from_fd) = {
            // Use private connection if one exists
            let to_server_fd = env::var("LIGHTDM_TO_SERVER_FD");
            let from_server_fd = env::var("LIGHTDM_FROM_SERVER_FD");
            let pipe_path = env::var("LIGHTDM_GREETER_PIPE");

            if let Ok(to_server_fd) = to_server_fd
                && let Ok(from_server_fd) = from_server_fd
            {
                (
                    to_server_fd.parse::<i32>().map_err(|_| {
                        glib::Error::new(
                            GreeterError::Connection,
                            "Invalid file descriptor: LIGHTDM_TO_SERVER_FD={to_server_fd}",
                        )
                    })?,
                    from_server_fd.parse::<i32>().map_err(|_| {
                        glib::Error::new(
                            GreeterError::Connection,
                            "Invalid file descriptor: LIGHTDM_FROM_SERVER_FD={from_server_fd}",
                        )
                    })?,
                )
            } else if let Ok(pipe_path) = pipe_path {
                let socket = UnixStream::connect(pipe_path)
                    .map_err(|e| glib::Error::new(GreeterError::Connection, &e.to_string()))?;
                let fd = socket.as_raw_fd();
                self.socket.replace(Some(socket));
                (fd, fd)
            } else {
                return Err(glib::Error::new(
                    GreeterError::Connection,
                    "Unable to dertermine socket to daemon",
                ));
            }
        };

        let to_server_writer =
            PipeWriter::from(unsafe { OwnedFd::from_raw_fd(to_fd).try_clone().unwrap() });
        self.to_server_channel.replace(Some(to_server_writer));
        let from_server_reader =
            PipeReader::from(unsafe { OwnedFd::from_raw_fd(from_fd).try_clone().unwrap() });
        let from_server_reader_fd = from_server_reader.as_raw_fd();
        self.from_server_channel.replace(Some(from_server_reader));

        self.from_server_watch
            .replace(Some(glib_unix::unix_fd_add_local(
                from_server_reader_fd,
                glib::IOCondition::IN,
                glib::clone!(
                    #[strong(rename_to = this)]
                    self.obj(),
                    move |_, _| {
                        let imp = this.imp();
                        match imp.recv_message() {
                            Ok(message) => {
                                imp.handle_message(message);
                                glib::ControlFlow::Continue
                            }
                            Err(e) => {
                                log::warn!("Failed to read from daemon: {e}");
                                glib::ControlFlow::Break
                            }
                        }
                    }
                ),
            )));

        Ok(())
    }

    pub(super) fn send_message(&self, message: GreeterMessage) -> Result<(), glib::Error> {
        self.connect_to_daemon()?;

        let mut to_server_channel = self.to_server_channel.borrow_mut();
        let buf = message.into_bytes();
        let writer = to_server_channel.as_mut().unwrap();
        writer.write_all(&buf).map_err(|e| {
            glib::Error::new(
                GreeterError::Communication,
                &format!("Failed to write to daemon: {e}"),
            )
        })?;

        log::debug!("Wrote {} bytes to daemon", buf.len());
        writer.flush().map_err(|e| {
            glib::Error::new(
                GreeterError::Communication,
                &format!("Failed to write to daemon: {e}"),
            )
        })
    }

    pub(super) fn send_connect(&self, resettable: bool) -> Result<(), glib::Error> {
        log::debug!("Connecting to display manager...");
        let mut message = GreeterMessage::builder(GreeterMessageType::Connect);
        if resettable {
            message = message.arg_u32(1);
        } else {
            message = message.arg_u32(0)
        };
        message = message.arg_u32(API_VERSION);
        self.send_message(message.build())
    }

    pub(crate) fn send_start_session(&self, session: Option<String>) -> Result<(), glib::Error> {
        let mut message = GreeterMessage::builder(GreeterMessageType::StartSession);
        match session {
            Some(session) => {
                log::debug!("Starting session {session}");
                message = message.arg_string(session);
            }
            None => log::debug!("Starting default session"),
        }
        self.send_message(message.build())
    }

    pub(crate) fn send_ensure_shared_data_dir(&self, username: String) -> Result<(), glib::Error> {
        log::debug!("Ensuring data directory for user {username}");
        let message = GreeterMessage::builder(GreeterMessageType::EnsureSharedDir)
            .arg_string(username)
            .build();
        self.send_message(message)
    }

    pub(super) fn recv_message(&self) -> Result<ServerMessage, glib::Error> {
        self.connect_to_daemon()?;

        let mut from_server_channel = self.from_server_channel.borrow_mut();
        let reader = from_server_channel.as_mut().unwrap();
        ServerMessage::from_reader(reader)
    }

    pub(super) fn handle_message(&self, message: ServerMessage) {
        match message.id {
            ServerMessageType::Connected => self.handle_connected(false, message),
            ServerMessageType::PromptAuthentication => self.handle_prompt_authentication(message),
            ServerMessageType::EndAuthentication => self.handle_end_authentication(message),
            ServerMessageType::SessionResult => self.handle_sessoin_result(message),
            ServerMessageType::SharedDirResult => self.handle_shared_dir_result(message),
            ServerMessageType::Idle => self.handle_idle(),
            ServerMessageType::Reset => self.handle_reset(message),
            ServerMessageType::ConnectedV2 => self.handle_connected(true, message),
        }
    }

    fn handle_connected(&self, v2: bool, message: ServerMessage) {
        let mut debug_string = String::from("Connected");
        let mut reader = message.reader();
        if v2 {
            let api_version = reader.read_u32();
            self.api_version.replace(api_version);
            debug_string.push_str(&format!(" api={}", api_version));
            let version = reader.read_string().unwrap();
            debug_string.push_str(&format!(" version={}", version));
            let n_env = reader.read_u32();
            for _ in 0..n_env {
                let name = reader.read_string().unwrap();
                let value = reader.read_string().unwrap();
                debug_string.push_str(&format!(" {}={}", name, value));
                self.hints.borrow_mut().insert(name, value);
            }
        } else {
            self.api_version.replace(0);
            let version = reader.read_string().unwrap();
            debug_string.push_str(&format!(" version={}", version));
            while reader.is_empty() {
                let name = reader.read_string().unwrap();
                let value = reader.read_string().unwrap();
                debug_string.push_str(&format!(" {}={}", name, value));
                self.hints.borrow_mut().insert(name, value);
            }
        }

        self.connected.replace(true);
        log::debug!("{debug_string}");
        let obj = self.obj();
        let timeout = obj.autologin_timeout_hint();
        if timeout > 0 {
            log::debug!("Setting autologin timer for {timeout} seconds");
            let source_id = glib::source::timeout_add_seconds_local_once(
                timeout,
                glib::clone!(
                    #[strong(rename_to = this)]
                    obj,
                    move || {
                        this.imp().autologin_timeout.take();
                        this.imp().emit_autologin_timer_expired();
                    },
                ),
            );
            self.autologin_timeout.replace(Some(source_id));
        }

        if let Some(request) = self.connect_requests.borrow_mut().pop_front() {
            request.finish(Ok(()));
        }
    }

    fn handle_prompt_authentication(&self, message: ServerMessage) {
        let mut reader = message.reader();

        let sequence_number = reader.read_u32();
        if sequence_number != *self.authenticate_sequence_number.borrow() {
            log::debug!(
                "Ignoring prompt authentication with invalid sequence number {sequence_number}"
            );
            return;
        }

        if *self.cancelling_authentication.borrow() {
            log::debug!("Ignoring prompt authentication as waiting for it to cancel");
            return;
        }

        let username = reader.read_string();
        self.authentication_user.replace(username);
        self.responses_received.borrow_mut().clear();

        let n_messages = reader.read_u32();
        log::debug!("Prompt user with {n_messages} message(s)");

        for _ in 0..n_messages {
            let style = reader.read_u32();
            let text = reader.read_string();

            match style {
                1 => {
                    self.n_responses_waiting.replace_with(|n| *n + 1);
                    self.emit_show_prompt(text.unwrap(), LIGHTDM_PROMPT_TYPE_SECRET);
                }
                2 => {
                    self.n_responses_waiting.replace_with(|n| *n + 1);
                    self.emit_show_prompt(text.unwrap(), LIGHTDM_PROMPT_TYPE_VISIBLE);
                }
                3 => self.emit_show_message(text.unwrap(), LIGHTDM_MESSAGE_TYPE_ERROR),
                4 => self.emit_show_message(text.unwrap(), LIGHTDM_MESSAGE_TYPE_INFO),
                _ => {}
            }
        }
    }

    fn handle_end_authentication(&self, message: ServerMessage) {
        let mut reader = message.reader();

        let sequence_number = reader.read_u32();
        if sequence_number != *self.authenticate_sequence_number.borrow() {
            log::debug!(
                "Ignoring prompt authentication with invalid sequence number {sequence_number}"
            );
            return;
        }

        let username = reader.read_string();
        let return_code = reader.read_u32();
        log::debug!(
            "Authentication complete for user {} with return code {return_code}",
            username.as_ref().unwrap_or(&"".to_string())
        );

        self.authentication_user.replace(username);
        self.cancelling_authentication.replace(false);
        self.is_authenticated.replace(return_code == 0);
        self.in_authentication.replace(false);
        self.emit_authentication_complete();
    }

    fn handle_sessoin_result(&self, message: ServerMessage) {
        let mut reader = message.reader();
        if let Some(request) = self.start_session_requests.borrow_mut().pop_front() {
            let return_code = reader.read_u32();
            let res = if return_code == 0 {
                Ok(())
            } else {
                Err(glib::Error::new(
                    GreeterError::Session,
                    &format!("Session returned error code {return_code}"),
                ))
            };
            request.finish(res);
        }
    }

    fn handle_shared_dir_result(&self, message: ServerMessage) {
        let mut reader = message.reader();
        if let Some(request) = self
            .ensure_shared_data_dir_requests
            .borrow_mut()
            .pop_front()
        {
            let res = match reader.read_string() {
                Some(dir) => Ok(dir),
                None => Err(glib::Error::new(GreeterError::InvalidUser, "No such user")),
            };
            request.finish(res);
        }
    }

    fn handle_idle(&self) {
        self.emit_idle()
    }

    fn handle_reset(&self, message: ServerMessage) {
        self.hints.borrow_mut().clear();

        let mut reader = message.reader();
        let mut hint = String::new();
        let mut hints = self.hints.borrow_mut();
        while !reader.is_empty() {
            let name = reader.read_string().unwrap();
            let value = reader.read_string().unwrap();
            hints.insert(name, value);
            hint.push_str(" {name}={value}");
        }

        log::debug!("Reset{hint}");
        self.emit_reset()
    }

    fn emit_authentication_complete(&self) {
        self.obj()
            .emit_by_name::<()>("authentication-complete", &[]);
    }

    fn emit_autologin_timer_expired(&self) {
        self.obj()
            .emit_by_name::<()>("autologin-timer-expired", &[]);
    }

    fn emit_show_prompt(&self, text: String, type_: u32) {
        self.obj()
            .emit_by_name::<()>("show-prompt", &[&text.to_value(), &type_.to_value()]);
    }

    fn emit_show_message(&self, text: String, type_: u32) {
        self.obj()
            .emit_by_name::<()>("show-message", &[&text.to_value(), &type_.to_value()]);
    }

    fn emit_idle(&self) {
        self.obj().emit_by_name::<()>("idle", &[]);
    }

    fn emit_reset(&self) {
        self.obj().emit_by_name::<()>("reset", &[]);
    }
}
