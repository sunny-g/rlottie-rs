use self::PropertyValue::*;
use crate::Error;
use rlottie_sys as ffi;
use std::{convert::TryFrom, ffi::CString, ops::*, os::raw::c_char};

///
#[derive(Clone, Debug)]
pub struct Property(pub(crate) PropertyValue);

impl Property {
    #[inline(always)]
    pub fn val(&self) -> &PropertyValue {
        &self.0
    }

    #[inline(always)]
    pub(crate) fn ffi_type(&self) -> ffi::Lottie_Animation_Property {
        use ffi::Lottie_Animation_Property::*;
        match self.0 {
            FillColor { .. } => LOTTIE_ANIMATION_PROPERTY_FILLCOLOR,
            FillOpacity(_) => LOTTIE_ANIMATION_PROPERTY_FILLOPACITY,
            StrokeColor { .. } => LOTTIE_ANIMATION_PROPERTY_STROKECOLOR,
            StrokeOpacity(_) => LOTTIE_ANIMATION_PROPERTY_STROKEOPACITY,
            StrokeWidth(_) => LOTTIE_ANIMATION_PROPERTY_STROKEWIDTH,
            TransformAnchor(_) => LOTTIE_ANIMATION_PROPERTY_TR_ANCHOR,
            TransformPosition { .. } => LOTTIE_ANIMATION_PROPERTY_TR_POSITION,
            TransformScale { .. } => LOTTIE_ANIMATION_PROPERTY_TR_SCALE,
            TransformRotation(_) => LOTTIE_ANIMATION_PROPERTY_TR_ROTATION,
            TransformOpacity(_) => LOTTIE_ANIMATION_PROPERTY_TR_OPACITY,
        }
    }

    ///
    pub fn fill_color(r: f32, g: f32, b: f32) -> Option<Self> {
        match (r, g, b) {
            (0.0..=1.0, 0.0..=1.0, 0.0..=1.0) => Some(Self(FillColor { r, g, b })),
            _ => None,
        }
    }

    ///
    pub fn fill_opacity(opacity: f32) -> Option<Self> {
        match opacity {
            0.0..=100.0 => Some(Self(FillOpacity(opacity))),
            _ => None,
        }
    }

    ///
    pub fn stroke_color(r: f32, g: f32, b: f32) -> Option<Self> {
        match (r, g, b) {
            (0.0..=1.0, 0.0..=1.0, 0.0..=1.0) => Some(Self(StrokeColor { r, g, b })),
            _ => None,
        }
    }

    ///
    pub fn stroke_opacity(opacity: f32) -> Option<Self> {
        match opacity {
            0.0..=100.0 => Some(Self(StrokeOpacity(opacity))),
            _ => None,
        }
    }

    ///
    pub fn stroke_width(width: f32) -> Option<Self> {
        if width < 0.0 {
            None
        } else {
            Some(Self(PropertyValue::StrokeWidth(width)))
        }
    }

    ///
    pub fn transform_anchor(anchor: i32) -> Option<Self> {
        unimplemented!()
    }

    ///
    pub const fn transform_position(x: i32, y: i32) -> Self {
        Self(TransformPosition { x, y })
    }

    ///
    pub fn transform_scale(width: f32, height: f32) -> Option<Self> {
        match (width, height) {
            (0.0..=100.0, 0.0..=100.0) => Some(Self(TransformScale { width, height })),
            _ => None,
        }
    }

    ///
    pub fn transform_rotation(degrees: f32) -> Option<Self> {
        match degrees {
            0.0..=360.0 => Some(Self(TransformRotation(degrees))),
            _ => None,
        }
    }

    ///
    pub fn transform_opacity(opacity: f32) -> Option<Self> {
        match opacity {
            0.0..=100.0 => Some(Self(TransformOpacity(opacity))),
            _ => None,
        }
    }
}

///
#[derive(Copy, Clone, Debug)]
pub enum PropertyValue {
    FillColor { r: f32, g: f32, b: f32 },
    FillOpacity(f32),
    StrokeColor { r: f32, g: f32, b: f32 },
    StrokeOpacity(f32),
    StrokeWidth(f32),
    TransformAnchor(i32),
    TransformPosition { x: i32, y: i32 },
    TransformScale { width: f32, height: f32 },
    TransformRotation(f32),
    TransformOpacity(f32),
}

///
#[derive(Debug)]
pub struct KeyPath(CString);

impl KeyPath {
    #[inline(always)]
    pub(crate) fn as_ptr(&self) -> *const c_char {
        (*self.0).as_ptr()
    }
}

impl TryFrom<String> for KeyPath {
    type Error = Error;

    #[inline]
    fn try_from(keypath: String) -> Result<Self, Self::Error> {
        <Self as TryFrom<&str>>::try_from(&keypath)
    }
}

impl TryFrom<&str> for KeyPath {
    type Error = Error;

    #[inline]
    fn try_from(keypath: &str) -> Result<Self, Self::Error> {
        let cstring = CString::new(keypath).map_err(|e| {
            Error::FFI(format!(
                "unable to create `CString` from keypath {}",
                &keypath
            ))
        })?;
        Ok(Self(cstring))
    }
}
