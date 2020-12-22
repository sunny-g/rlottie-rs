mod error;
mod props;
mod types;

pub use error::Error;
pub use props::*;
pub use types::*;

use rlottie_sys as ffi;
use std::{
    ffi::{CStr, CString},
    fs::read_to_string,
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
    inner: *mut ffi::Lottie_Animation,
    // _surface: PhantomData,
}

///
impl LottieAnimation {
    ///
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let path = path.as_ref();
        let data = read_to_string(path).map_err(|e| {
            Error::Animation(format!(
                "unable to read lottie file from path {:?}: {}",
                &path, e
            ))
        })?;
        let key = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::Animation(format!("invalid path filename: {:?}", &path)))?;
        let resource_path = path
            .to_str()
            .ok_or_else(|| Error::Animation(format!("invalid file path: {:?}", &path)))?;

        Self::from_data(data, key, resource_path)
    }

    ///
    pub fn from_data(data: String, key: &str, resource_path: &str) -> Result<Self, Error> {
        let data = CString::new(data).map_err(|e| Error::FFI(format!("{}", e)))?;
        let key = CString::new(key).map_err(|e| Error::FFI(format!("{}", e)))?;
        let resource_path =
            CString::new(resource_path).map_err(|e| Error::FFI(format!("{}", e)))?;

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

    ///
    pub fn size(&self) -> (usize, usize) {
        let mut width = 0usize;
        let mut height = 0usize;
        unsafe {
            ffi::lottie_animation_get_size(
                self.inner,
                &mut width as *mut usize,
                &mut height as *mut usize,
            );
        }
        (width, height)
    }

    ///
    pub fn duration(&self) -> Duration {
        let sec = unsafe { ffi::lottie_animation_get_duration(self.inner) };
        Duration::from_secs_f64(sec)
    }

    ///
    pub fn num_frames(&self) -> usize {
        unsafe { ffi::lottie_animation_get_totalframe(self.inner) }
    }

    ///
    pub fn framerate(&self) -> f64 {
        unsafe { ffi::lottie_animation_get_framerate(self.inner) }
    }

    pub fn frame_at_pos(&self, position: f32) -> usize {
        unsafe { ffi::lottie_animation_get_frame_at_pos(self.inner, position) }
    }

    // pub fn marker_list(&self) -> MarkerList {
    //     unimplemented!()
    // }

    pub fn set_prop(&mut self, keypath: KeyPath, prop: Property) {
        use props::PropertyValue::*;
        use std::os::raw::c_double;

        let keypath_ptr = keypath.as_ptr();
        let prop_type = prop.ffi_type();
        unsafe {
            match prop.val() {
                FillColor { r, g, b } => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    *r as c_double,
                    *g as c_double,
                    *b as c_double,
                ),
                FillOpacity(opacity) => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    *opacity as c_double,
                ),
                StrokeColor { r, g, b } => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    *r as c_double,
                    *g as c_double,
                    *b as c_double,
                ),
                StrokeOpacity(opacity) => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    *opacity as c_double,
                ),
                StrokeWidth(width) => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    *width as c_double,
                ),
                TransformAnchor(_) => unimplemented!(),
                TransformPosition { x, y } => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    f64::from(*x),
                    f64::from(*y),
                ),
                TransformScale { width, height } => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    *width as c_double,
                    *height as c_double,
                ),
                TransformRotation(degrees) => ffi::lottie_animation_property_override(
                    self.inner,
                    prop_type,
                    keypath_ptr,
                    *degrees as c_double,
                ),
                TransformOpacity(_) => unimplemented!(),
            }
        }
    }

    // pub fn render_tree(&mut self, frame_num: usize, width: usize, height: usize) -> LayerNode {
    //     unimplemented!()
    // }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api() {
        let animation = LottieAnimation::from_file("examples/loading.json");
        assert!(animation.is_ok());

        let animation = animation.unwrap();
        assert_eq!(animation.size(), (237, 237));
    }
}
