use glib::{object::IsA, translate::*};

use std::{boxed::Box as Box_, pin::Pin};

use super::{Greeter, GreeterError};

pub trait GreeterExtManual: 'static {
    /// Starts the authentication procedure for a user.
    /// ## `username`
    /// A username or [`None`] to prompt for a username.
    ///
    /// # Returns
    ///
    /// [`true`] if authentication request sent.
    fn authenticate(&self, username: Option<&str>) -> Result<(), glib::Error>;
    /// Starts the authentication procedure for the guest user.
    ///
    /// # Returns
    ///
    /// [`true`] if authentication request sent.
    fn authenticate_as_guest(&self) -> Result<(), glib::Error>;
    /// Starts the authentication procedure for the automatic login user.
    ///
    /// # Returns
    ///
    /// [`true`] if authentication request sent.
    fn authenticate_autologin(&self) -> Result<(), glib::Error>;
    /// Start authentication for a remote session type.
    /// ## `session`
    /// The name of a remote session
    /// ## `username`
    /// A username of [`None`] to prompt for a username.
    ///
    /// # Returns
    ///
    /// [`true`] if authentication request sent.
    fn authenticate_remote(&self, session: &str, username: Option<&str>)
    -> Result<(), glib::Error>;
    /// Cancel the current user authentication.
    ///
    /// # Returns
    ///
    /// [`true`] if cancel request sent.
    fn cancel_authentication(&self) -> Result<(), glib::Error>;
    /// Connects the greeter to the display manager. Will block until connected.
    ///
    /// # Deprecated since 1.11.1
    ///
    /// Use [`connect_to_daemon_sync()`][Self::connect_to_daemon_sync()] instead
    ///
    /// # Returns
    ///
    /// [`true`] if successfully connected
    fn connect_sync(&self) -> Result<(), glib::Error>;
    /// Asynchronously connects the greeter to the display manager.
    ///
    /// When the operation is finished, `callback` will be invoked. You can then call `lightdm_greeter_connect_to_daemon_finish()` to get the result of the operation.
    ///
    /// See [`connect_to_daemon_sync()`][Self::connect_to_daemon_sync()] for the synchronous version.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    /// ## `callback`
    /// A `GAsyncReadyCallback` to call when completed or [`None`].
    fn connect_to_daemon<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );
    fn connect_to_daemon_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;
    /// Connects the greeter to the display manager. Will block until connected.
    ///
    /// # Returns
    ///
    /// [`true`] if successfully connected
    fn connect_to_daemon_sync(&self) -> Result<(), glib::Error>;
    /// Ensure that a shared data dir for the given user is available. Both the
    /// greeter user and `username` will have write access to that folder. The
    /// intention is that larger pieces of shared data would be stored there (files
    /// that the greeter creates but wants to give to a user -- like camera
    /// photos -- or files that the user creates but wants the greeter to
    /// see -- like contact avatars).
    ///
    /// LightDM will automatically create these if the user actually logs in, so
    /// greeters only need to call this method if they want to store something in
    /// the directory themselves.
    /// ## `username`
    /// A username
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    /// ## `callback`
    /// A `GAsyncReadyCallback` to call when completed or [`None`].
    fn ensure_shared_data_dir<P: FnOnce(Result<glib::GString, glib::Error>) + 'static>(
        &self,
        username: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );
    fn ensure_shared_data_dir_future(
        &self,
        username: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::GString, glib::Error>> + 'static>>;
    /// Ensure that a shared data dir for the given user is available. Both the
    /// greeter user and `username` will have write access to that folder. The
    /// intention is that larger pieces of shared data would be stored there (files
    /// that the greeter creates but wants to give to a user -- like camera
    /// photos -- or files that the user creates but wants the greeter to
    /// see -- like contact avatars).
    ///
    /// LightDM will automatically create these if the user actually logs in, so
    /// greeters only need to call this method if they want to store something in
    /// the directory themselves.
    /// ## `username`
    /// A username
    ///
    /// # Returns
    ///
    /// The path to the shared directory, free with g_free.
    fn ensure_shared_data_dir_sync(&self, username: &str) -> Result<glib::GString, glib::Error>;
    /// Provide response to a prompt. May be one in a series.
    /// ## `response`
    /// Response to a prompt
    ///
    /// # Returns
    ///
    /// [`true`] if response sent.
    fn respond(&self, response: &str) -> Result<(), glib::Error>;
    /// Set the language for the currently authenticated user.
    /// ## `language`
    /// The language to use for this user in the form of a locale specification (e.g. "de_DE.UTF-8").
    ///
    /// # Returns
    ///
    /// [`true`] if set language request sent.
    fn set_language(&self, language: &str) -> Result<(), glib::Error>;
    /// Asynchronously start a session for the authenticated user.
    ///
    /// When the operation is finished, `callback` will be invoked. You can then call `lightdm_greeter_start_session_finish()` to get the result of the operation.
    ///
    /// See [`start_session_sync()`][Self::start_session_sync()] for the synchronous version.
    /// ## `session`
    /// The session to log into or [`None`] to use the default.
    /// ## `cancellable`
    /// A [`gio::Cancellable`][crate::gio::Cancellable] or [`None`].
    /// ## `callback`
    /// A `GAsyncReadyCallback` to call when completed or [`None`].
    fn start_session<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        session: Option<&str>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    );
    fn start_session_future(
        &self,
        session: Option<&str>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;
    /// Start a session for the authenticated user.
    /// ## `session`
    /// The session to log into or [`None`] to use the default.
    ///
    /// # Returns
    ///
    /// TRUE if the session was started.
    fn start_session_sync(&self, session: Option<&str>) -> Result<(), glib::Error>;
}

impl<O: IsA<Greeter>> GreeterExtManual for O {
    #[doc(alias = "lightdm_greeter_authenticate")]
    fn authenticate(&self, username: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_authenticate(
                self.as_ref().to_glib_none().0,
                username.to_glib_none().0,
                &mut error,
            );

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(5),
                    "lightdm_greeter_authenticate() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_authenticate_as_guest")]
    fn authenticate_as_guest(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_authenticate_as_guest(
                self.as_ref().to_glib_none().0,
                &mut error,
            );

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(6),
                    "lightdm_greeter_authenticate_as_guest() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_authenticate_autologin")]
    fn authenticate_autologin(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_authenticate_autologin(
                self.as_ref().to_glib_none().0,
                &mut error,
            );

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(7),
                    "lightdm_greeter_authenticate_autologin() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_authenticate_remote")]
    fn authenticate_remote(
        &self,
        session: &str,
        username: Option<&str>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_authenticate_remote(
                self.as_ref().to_glib_none().0,
                session.to_glib_none().0,
                username.to_glib_none().0,
                &mut error,
            );

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(8),
                    "lightdm_greeter_authenticate_remote() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_cancel_authentication")]
    fn cancel_authentication(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_cancel_authentication(
                self.as_ref().to_glib_none().0,
                &mut error,
            );

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(9),
                    "lightdm_greeter_cancel_authentication() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_connect_sync")]
    fn connect_sync(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::lightdm_greeter_connect_sync(self.as_ref().to_glib_none().0, &mut error);

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(10),
                    "lightdm_greeter_connect_sync() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_connect_to_daemon")]
    fn connect_to_daemon<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn connect_to_daemon_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let is_ok = unsafe {
                ffi::lightdm_greeter_connect_to_daemon_finish(
                    _source_object as *mut _,
                    res,
                    &mut error,
                )
            };

            let result = if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(11),
                    "lightdm_greeter_connect_to_daemon_trampoline() failed",
                ))
            } else {
                Err(unsafe { from_glib_full(error) })
            };

            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                unsafe { Box_::from_raw(user_data as *mut _) };
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = connect_to_daemon_trampoline::<P>;
        unsafe {
            ffi::lightdm_greeter_connect_to_daemon(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_to_daemon_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.connect_to_daemon(Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "lightdm_greeter_connect_to_daemon_sync")]
    fn connect_to_daemon_sync(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_connect_to_daemon_sync(
                self.as_ref().to_glib_none().0,
                &mut error,
            );

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(12),
                    "lightdm_greeter_connect_to_daemon_sync() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
    #[doc(alias = "lightdm_greeter_ensure_shared_data_dir")]
    fn ensure_shared_data_dir<P: FnOnce(Result<glib::GString, glib::Error>) + 'static>(
        &self,
        username: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn ensure_shared_data_dir_trampoline<
            P: FnOnce(Result<glib::GString, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = unsafe {
                ffi::lightdm_greeter_ensure_shared_data_dir_finish(
                    _source_object as *mut _,
                    res,
                    &mut error,
                )
            };
            let result = unsafe {
                if error.is_null() && !ret.is_null() {
                    Ok(from_glib_full(ret))
                } else if ret.is_null() {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(13),
                        "lightdm_greeter_ensure_shared_data_dir_finish() failed",
                    ))
                } else {
                    Err(from_glib_full(error))
                }
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                unsafe { Box_::from_raw(user_data as *mut _) };
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = ensure_shared_data_dir_trampoline::<P>;
        unsafe {
            ffi::lightdm_greeter_ensure_shared_data_dir(
                self.as_ref().to_glib_none().0,
                username.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn ensure_shared_data_dir_future(
        &self,
        username: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::GString, glib::Error>> + 'static>>
    {
        let username = String::from(username);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.ensure_shared_data_dir(&username, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "lightdm_greeter_ensure_shared_data_dir_sync")]
    fn ensure_shared_data_dir_sync(&self, username: &str) -> Result<glib::GString, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::lightdm_greeter_ensure_shared_data_dir_sync(
                self.as_ref().to_glib_none().0,
                username.to_glib_none().0,
                &mut error,
            );
            if error.is_null() && !ret.is_null() {
                Ok(from_glib_full(ret))
            } else if ret.is_null() {
                Err(glib::Error::new(
                    GreeterError::__Unknown(14),
                    "lightdm_greeter_ensure_shared_data_dir_sync() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_respond")]
    fn respond(&self, response: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_respond(
                self.as_ref().to_glib_none().0,
                response.to_glib_none().0,
                &mut error,
            );

            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(15),
                    "lightdm_greeter_respond() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_set_language")]
    fn set_language(&self, language: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_set_language(
                self.as_ref().to_glib_none().0,
                language.to_glib_none().0,
                &mut error,
            );
            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(16),
                    "lightdm_greeter_set_language() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "lightdm_greeter_start_session")]
    fn start_session<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        session: Option<&str>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn start_session_trampoline<
            P: FnOnce(Result<(), glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let is_ok = unsafe {
                ffi::lightdm_greeter_start_session_finish(_source_object as *mut _, res, &mut error)
            };

            let result = if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(17),
                    "lightdm_greeter_start_session_trampoline() failed",
                ))
            } else {
                Err(unsafe { from_glib_full(error) })
            };

            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                unsafe { Box_::from_raw(user_data as *mut _) };
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = start_session_trampoline::<P>;
        unsafe {
            ffi::lightdm_greeter_start_session(
                self.as_ref().to_glib_none().0,
                session.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn start_session_future(
        &self,
        session: Option<&str>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let session = session.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.start_session(
                session.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "lightdm_greeter_start_session_sync")]
    fn start_session_sync(&self, session: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_start_session_sync(
                self.as_ref().to_glib_none().0,
                session.to_glib_none().0,
                &mut error,
            );
            if is_ok == glib::ffi::GTRUE && error.is_null() {
                Ok(())
            } else if error.is_null() {
                debug_assert_eq!(is_ok, glib::ffi::GFALSE);
                Err(glib::Error::new(
                    GreeterError::__Unknown(18),
                    "lightdm_greeter_start_session_sync() failed",
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
