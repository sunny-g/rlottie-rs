mod error;

use crate::error::Error;
use rlottie_sys as ffi;
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    path::Path,
    time::Duration,
};

///
pub trait Surface {
    fn new(width: usize, height: usize, bytes_per_line: usize) -> Self;

    fn set_draw_region(&mut self, x: usize, y: usize, width: usize, height: usize);

    fn buffer(&mut self) -> &mut [u32];
}

///
#[derive(Debug)]
pub struct LottieAnimation {
    key: CString,
    resource_path: CString,
    inner: *const ffi::Lottie_Animation,
    // _surface: PhantomData,
}

///
impl LottieAnimation {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        unimplemented!()
    }

    pub fn from_data(data: String, key: &str, resource_path: &str) -> Result<Self, Error> {
        let data = CString::new(data)?;
        let key = CString::new(key)?;
        let resource_path = CString::new(resource_path)?;

        let inner = unsafe {
            ffi::lottie_animation_from_data(data.as_ptr(), key.as_ptr(), resource_path.as_ptr())
        };

        if inner.is_null() {
            Err(Error::FFI("failed to instantiate LottieAnimation".into()))
        } else {
            Ok(Self {
                key,
                resource_path,
                inner,
                // _surface: PhantomData,
            })
        }
    }

    pub fn size(&self) -> (usize, usize) {
        unimplemented!()
    }

    pub fn duration(&self) -> Duration {
        unimplemented!()
    }

    pub fn total_frames(&self) -> usize {
        unimplemented!()
    }

    pub fn framerate(&self) -> f64 {
        unimplemented!()
    }

    pub fn frame_at_pos(&self, position: f32) -> usize {
        unimplemented!()
    }

    pub fn marker_list(&self) -> MarkerList {
        unimplemented!()
    }

    pub fn set_props(&mut self, prop: Property, keypath: &KeyPath) {}

    pub fn render_tree(&mut self, frame_num: usize, width: usize, height: usize) -> LayerNode {
        unimplemented!()
    }

    pub fn render(
        &mut self,
        frame_num: usize,
        buffer: &mut [u32],
        width: usize,
        height: usize,
        bytes_per_line: usize,
    ) {
    }

    pub fn render_async(
        &mut self,
        frame_num: usize,
        buffer: &mut [u32],
        width: usize,
        height: usize,
        bytes_per_line: usize,
    ) {
    }

    pub fn render_flush(&mut self) {}
}

impl Drop for LottieAnimation {
    fn drop(&mut self) {
        unsafe { ffi::lottie_animation_destroy(self.inner as *mut ffi::Lottie_Animation) }
    }
}

#[derive(Debug)]
pub struct KeyPath;

#[derive(Debug)]
pub struct LayerNode;

#[derive(Debug)]
pub struct Property;

#[derive(Debug)]
pub struct Marker;

#[derive(Debug)]
pub struct MarkerList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_animation() {
        let loading = std::fs::read_to_string("examples/loading.json").unwrap();
        assert!(LottieAnimation::from_data(loading, "loading", "").is_ok());
    }
}
