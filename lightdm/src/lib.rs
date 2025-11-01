#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

pub use ffi;
pub use glib;

#[allow(unused_imports)]
mod auto;
pub use auto::*;

mod greeter;

pub mod prelude {
    pub use super::auto::traits::*;
    pub use super::greeter::GreeterExtManual;
}

pub mod functions {
    pub use super::auto::functions::*;
}
