// SPDX-FileCopyrightText: 2026 ZaynChen <zaynchen@qq.com>
//
// SPDX-License-Identifier: MIT
#![cfg_attr(docsrs, feature(doc_cfg))]

mod imp;
mod message;

use gio::{self, Cancellable, prelude::CancellableExt};
use glib::{self, MainContext, object::ObjectExt, subclass::prelude::*};

use std::{
    cell::{Cell, RefCell},
    pin::Pin,
    rc::Rc,
};

use message::{GreeterMessage, GreeterMessageType};

pub mod prelude {
    pub use glib::object::ObjectExt;
}

glib::wrapper! {
    /// see [lightdm::Greeter](https://zaynchen.github.io/lightdm-rs/stable/latest/docs/lightdm/struct.Greeter.html)
    pub struct Greeter(ObjectSubclass<imp::Greeter>);
}

impl Greeter {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn connect_authentication_complete<F: Fn(&Self) + 'static>(&self, callback: F) {
        self.connect_closure(
            "authentication-complete",
            true,
            glib::closure_local!(move |this| callback(this)),
        );
    }

    pub fn connect_autologin_timer_expired<F: Fn(&Self) + 'static>(&self, callback: F) {
        self.connect_closure(
            "autologin-timer-expired",
            true,
            glib::closure_local!(move |this| callback(this)),
        );
    }

    pub fn connect_idle<F: Fn(&Self) + 'static>(&self, callback: F) {
        self.connect_closure(
            "idle",
            true,
            glib::closure_local!(move |this| callback(this)),
        );
    }

    pub fn connect_reset<F: Fn(&Self) + 'static>(&self, callback: F) {
        self.connect_closure(
            "reset",
            true,
            glib::closure_local!(move |this| callback(this)),
        );
    }

    pub fn connect_show_prompt<F: Fn(&Self, &str, u32) + 'static>(&self, callback: F) {
        self.connect_closure(
            "show-prompt",
            true,
            glib::closure_local!(move |this, text, type_| callback(this, text, type_)),
        );
    }

    pub fn connect_show_message<F: Fn(&Self, &str, u32) + 'static>(&self, callback: F) {
        self.connect_closure(
            "show-message",
            true,
            glib::closure_local!(move |this, text, type_| callback(this, text, type_)),
        );
    }

    pub fn hint(&self, name: &str) -> Option<String> {
        self.imp().hints.borrow().get(name).cloned()
    }

    pub fn default_session_hint(&self) -> Option<String> {
        self.hint("default-session")
    }

    fn bool_hint(&self, name: &str) -> bool {
        self.hint(name).is_some_and(|v| v == "true")
    }

    pub fn hide_users_hint(&self) -> bool {
        self.bool_hint("hide-users")
    }

    pub fn show_manual_login_hint(&self) -> bool {
        self.bool_hint("show-manual-login")
    }

    pub fn show_remote_login_hint(&self) -> bool {
        self.bool_hint("show-remote-login")
    }

    pub fn lock_hint(&self) -> bool {
        self.bool_hint("lock-screen")
    }

    pub fn has_guest_account_hint(&self) -> bool {
        self.bool_hint("has-guest-account")
    }

    pub fn select_user_hint(&self) -> Option<String> {
        self.hint("select-user")
    }

    pub fn select_guest_hint(&self) -> bool {
        self.bool_hint("select-guest")
    }

    pub fn autologin_user_hint(&self) -> Option<String> {
        self.hint("autologin-user")
    }

    pub fn autologin_session_hint(&self) -> Option<String> {
        self.hint("autologin-sessin")
    }

    pub fn autologin_guest_hint(&self) -> bool {
        self.bool_hint("autologin-guest")
    }

    pub fn autologin_timeout_hint(&self) -> u32 {
        self.hint("autologin-timeout")
            .map(|t| t.parse::<u32>().unwrap_or(0))
            .unwrap_or(0)
    }

    pub fn cancel_autologin(&self) {
        if let Some(timer) = self.imp().autologin_timeout.borrow_mut().take() {
            timer.remove();
        }
    }

    pub fn authenticate(&self, username: Option<String>) -> Result<(), glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }

        imp.cancelling_authentication.replace(false);
        imp.authenticate_sequence_number.replace_with(|n| *n + 1);
        imp.in_authentication.replace(true);
        imp.is_authenticated.replace(false);

        log::debug!(
            "Starting authentication for user {}...",
            username.as_ref().unwrap_or(&"".to_string())
        );
        imp.authentication_user.replace(username.clone());

        let message = GreeterMessage::builder(GreeterMessageType::Authenticate)
            .arg_u32(*self.imp().authenticate_sequence_number.borrow())
            .arg_string(username.unwrap_or_default())
            .build();
        imp.send_message(message)
    }

    pub fn authenticate_as_guest(&self) -> Result<(), glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }

        imp.cancelling_authentication.replace(false);
        imp.authenticate_sequence_number.replace_with(|n| *n + 1);
        imp.in_authentication.replace(true);
        imp.is_authenticated.replace(false);
        imp.authentication_user.borrow_mut().take();

        log::debug!("Starting authentication for guest account...");
        let message = GreeterMessage::builder(GreeterMessageType::AuthenticateAsGuest)
            .arg_u32(*self.imp().authenticate_sequence_number.borrow())
            .build();
        imp.send_message(message)
    }

    pub fn authenticate_autologin(&self) -> Result<(), glib::Error> {
        if self.autologin_guest_hint() {
            self.authenticate_as_guest()
        } else if let Some(username) = self.autologin_user_hint() {
            self.authenticate(Some(username))
        } else {
            Err(glib::Error::new(
                GreeterError::NoAutologin,
                "Can't authenticate autologin; autologin not configured",
            ))
        }
    }

    pub fn authenticate_remote(
        &self,
        session: String,
        username: Option<String>,
    ) -> Result<(), glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }

        imp.cancelling_authentication.replace(false);
        imp.authenticate_sequence_number.replace_with(|n| *n + 1);
        imp.in_authentication.replace(true);
        imp.is_authenticated.replace(false);
        imp.authentication_user.borrow_mut().take();

        if let Some(username) = &username {
            log::debug!(
                "Starting authentication for remote session {session} as user {username}..."
            );
        } else {
            log::debug!("Starting authentication for remote session {session}...",);
        }

        let message = GreeterMessage::builder(GreeterMessageType::AuthenticateRemote)
            .arg_u32(*imp.authenticate_sequence_number.borrow())
            .arg_string(session)
            .arg_string(username.unwrap_or_default())
            .build();
        imp.send_message(message)
    }

    pub fn respond(&self, response: String) -> Result<(), glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }

        if *imp.n_responses_waiting.borrow() == 0 {
            return Err(glib::Error::new(
                GreeterError::Session,
                "session has no waiting responses",
            ));
        }
        imp.n_responses_waiting.replace_with(|n| *n - 1);
        imp.responses_received.borrow_mut().push(response);
        if *imp.n_responses_waiting.borrow() == 0 {
            log::debug!("Providing response to display manager");

            let responses = imp.responses_received.take();
            let mut message = GreeterMessage::builder(GreeterMessageType::ContinueAuthentication)
                .arg_u32(responses.len() as u32);
            for response in responses.into_iter() {
                message = message.arg_string(response.clone());
            }
            return imp.send_message(message.build());
        }
        Ok(())
    }

    pub fn cancel_authentication(&self) -> Result<(), glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }
        imp.cancelling_authentication.replace(true);
        log::debug!("Cancelling Authentication...");
        imp.send_message(GreeterMessage::builder(GreeterMessageType::CancelAuthentication).build())
    }

    pub fn set_language(&self, language: String) -> Result<(), glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }
        let message = GreeterMessage::builder(GreeterMessageType::SetLanguage)
            .arg_string(language)
            .build();
        imp.send_message(message)
    }

    pub fn connect_to_daemon<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Cancellable,
        callback: P,
    ) {
        let main_context = MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let request = Rc::new(
            Request::builder()
                .cancellable(cancellable)
                .callback(callback)
                .build(),
        );
        let imp = self.imp();
        if let Err(e) = imp.send_connect(self.resettable()) {
            request.finish(Err(e));
        } else {
            imp.connect_requests.borrow_mut().push_back(request);
        }
    }

    pub fn connect_to_daemon_future(
        &self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.connect_to_daemon(cancellable.clone(), move |res| {
                send.resolve(res);
            });
        }))
    }

    pub fn connect_to_daemon_sync(&self) -> Result<(), glib::Error> {
        let imp = self.imp();
        imp.send_connect(self.resettable())?;

        let request = Rc::new(Request::default());
        imp.connect_requests.borrow_mut().push_back(request.clone());
        while !request.finished.get() {
            imp.handle_message(imp.recv_message()?);
        }
        Ok(())
    }

    pub fn start_session<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        session: Option<String>,
        cancellable: Cancellable,
        callback: P,
    ) {
        let main_context = MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let request = Rc::new(
            Request::builder()
                .cancellable(cancellable)
                .callback(callback)
                .build(),
        );
        let imp = self.imp();
        if let Err(e) = imp.send_start_session(session) {
            request.finish(Err(e))
        } else {
            imp.start_session_requests.borrow_mut().push_back(request);
        }
    }

    pub fn start_session_future(
        &self,
        session: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), glib::Error>> + 'static>> {
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.start_session(session, cancellable.clone(), move |res| {
                send.resolve(res);
            });
        }))
    }

    pub fn start_session_sync(&self, session: Option<String>) -> Result<(), glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }
        if !self.is_authenticated() {
            return Err(glib::Error::new(GreeterError::Session, ""));
        }

        imp.send_start_session(session)?;
        let request = Rc::new(Request::default());
        imp.start_session_requests
            .borrow_mut()
            .push_back(request.clone());
        while !request.finished.get() {
            imp.handle_message(imp.recv_message()?)
        }
        Ok(())
    }

    pub fn ensure_shared_data_dir<P: FnOnce(Result<String, glib::Error>) + 'static>(
        &self,
        username: String,
        cancellable: Cancellable,
        callback: P,
    ) {
        let main_context = MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let request = Rc::new(
            Request::builder()
                .cancellable(cancellable)
                .callback(callback)
                .build(),
        );
        let imp = self.imp();
        if let Err(e) = imp.send_ensure_shared_data_dir(username) {
            request.finish(Err(e))
        } else {
            imp.ensure_shared_data_dir_requests
                .borrow_mut()
                .push_back(request);
        }
    }

    pub fn ensure_shared_data_dir_future(
        &self,
        username: &str,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<String, glib::Error>> + 'static>> {
        let username = String::from(username);
        Box::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.ensure_shared_data_dir(username, cancellable.clone(), move |res| {
                send.resolve(res);
            });
        }))
    }

    pub fn ensure_shared_data_dir_sync(
        &self,
        username: String,
    ) -> Result<Option<String>, glib::Error> {
        let imp = self.imp();
        if !*imp.connected.borrow() {
            return Err(glib::Error::new(GreeterError::Connection, ""));
        }

        imp.send_ensure_shared_data_dir(username)?;
        let request = Rc::new(Request::default());
        imp.ensure_shared_data_dir_requests
            .borrow_mut()
            .push_back(request.clone());
        while !request.finished.get() {
            imp.handle_message(imp.recv_message()?)
        }
        Ok(request.dir.take())
    }
}

impl Default for Greeter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone, glib::ErrorDomain)]
#[error_domain(name = "lightdm_client")]
pub enum GreeterError {
    Communication,
    Connection,
    Session,
    NoAutologin,
    InvalidUser,
    __Unknown,
}

type RequestCallback<T> = Box<dyn FnOnce(Result<T, glib::Error>)>;

#[derive(Default)]
struct Request<T> {
    cancellable: Option<Cancellable>,
    callback: RefCell<Option<RequestCallback<T>>>,
    finished: Cell<bool>,
    dir: Cell<Option<String>>,
}

#[derive(Default)]
struct RequestBuilder<T> {
    cancellable: Option<Cancellable>,
    callback: RefCell<Option<RequestCallback<T>>>,
}

impl<T> RequestBuilder<T> {
    fn new() -> Self {
        Self {
            cancellable: None,
            callback: RefCell::new(None),
        }
    }

    fn cancellable(mut self, cancellable: Cancellable) -> Self {
        self.cancellable = Some(cancellable);
        self
    }

    fn callback<P: FnOnce(Result<T, glib::Error>) + 'static>(mut self, callback: P) -> Self {
        self.callback = RefCell::new(Some(Box::new(callback)));
        self
    }

    fn build(self) -> Request<T> {
        Request {
            cancellable: self.cancellable,
            callback: self.callback,
            finished: Cell::new(false),
            dir: Cell::new(None),
        }
    }
}

impl<T> Request<T> {
    fn builder() -> RequestBuilder<T> {
        RequestBuilder::new()
    }
}

impl Request<()> {
    fn finish(&self, res: Result<(), glib::Error>) {
        self.finished.set(true);
        if let Some(callback) = self.callback.take()
            && self.cancellable.as_ref().is_none_or(|c| !c.is_cancelled())
        {
            glib::idle_add_local_once(move || callback(res));
        }
    }
}

impl Request<String> {
    fn finish(&self, res: Result<String, glib::Error>) {
        self.finished.set(true);
        if let Some(callback) = self.callback.take()
            && self.cancellable.as_ref().is_none_or(|c| !c.is_cancelled())
        {
            glib::idle_add_local_once(move || callback(res));
        }
    }
}
