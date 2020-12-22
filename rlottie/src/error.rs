use std::ffi::NulError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("rlottie FFI error: {0}")]
    FFI(String),

    #[error("animation data error: {0}")]
    Animation(String),
}
