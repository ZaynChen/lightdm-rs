use glib::{object::IsA, translate::*};

use std::{boxed::Box as Box_, pin::Pin};

use super::{Greeter, GreeterError};

pub trait GreeterExtManual: IsA<Greeter> + 'static {
    #[doc(alias = "lightdm_greeter_authenticate")]
    fn authenticate(&self, username: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::lightdm_greeter_authenticate(
                self.as_ref().to_glib_none().0,
                username.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(5),
                        "lightdm_greeter_authenticate() return false with null error",
                    ))
                }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(6),
                        "lightdm_greeter_authenticate_as_guest() return false with null error",
                    ))
                }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(7),
                        "lightdm_greeter_authenticate_autologin() return false with null error",
                    ))
                }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(8),
                        "lightdm_greeter_authenticate_remote() return false with null error",
                    ))
                }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(9),
                        "lightdm_greeter_cancel_authentication() return false with null error",
                    ))
                }
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v1_11_1", deprecated = "Since 1.11.1")]
    #[allow(deprecated)]
    #[doc(alias = "lightdm_greeter_connect_sync")]
    fn connect_sync(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::lightdm_greeter_connect_sync(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(10),
                        "lightdm_greeter_connect_sync() return false with null error",
                    ))
                }
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
            let result = if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(11),
                        "lightdm_greeter_connect_to_daemon_finish() return false with null error",
                    ))
                }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(12),
                        "lightdm_greeter_connect_to_daemon_sync() return false with null error",
                    ))
                }
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
            unsafe {
                let mut error = std::ptr::null_mut();
                let ret = ffi::lightdm_greeter_ensure_shared_data_dir_finish(
                    _source_object as *mut _,
                    res,
                    &mut error,
                );
                let result = if error.is_null() {
                    if !ret.is_null() {
                        Ok(from_glib_full(ret))
                    } else {
                        Err(glib::Error::new(
                            GreeterError::__Unknown(13),
                            "lightdm_greeter_ensure_shared_data_dir_finish() return null with null error",
                        ))
                    }
                } else {
                    Err(from_glib_full(error))
                };
                let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                    Box_::from_raw(user_data as *mut _);
                let callback: P = callback.into_inner();
                callback(result);
            }
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
            if error.is_null() {
                if !ret.is_null() {
                    Ok(from_glib_full(ret))
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(14),
                        "lightdm_greeter_ensure_shared_data_dir_sync() return null with null error",
                    ))
                }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(15),
                        "lightdm_greeter_respond() return false with null error",
                    ))
                }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(16),
                        "lightdm_greeter_set_language() return false with null error",
                    ))
                }
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
            unsafe {
                let mut error = std::ptr::null_mut();
                let is_ok = ffi::lightdm_greeter_start_session_finish(
                    _source_object as *mut _,
                    res,
                    &mut error,
                );
                let result = if error.is_null() {
                    if is_ok == glib::ffi::GTRUE {
                        Ok(())
                    } else {
                        Err(glib::Error::new(
                            GreeterError::__Unknown(17),
                            "lightdm_greeter_start_session_finish() return false with null error",
                        ))
                    }
                } else {
                    Err(from_glib_full(error))
                };
                let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                    Box_::from_raw(user_data as *mut _);
                let callback: P = callback.into_inner();
                callback(result);
            }
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
            if error.is_null() {
                if is_ok == glib::ffi::GTRUE {
                    Ok(())
                } else {
                    Err(glib::Error::new(
                        GreeterError::__Unknown(18),
                        "lightdm_greeter_start_session_sync() return false with null error",
                    ))
                }
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<O: IsA<Greeter>> GreeterExtManual for O {}
