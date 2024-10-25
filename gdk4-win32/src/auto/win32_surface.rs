// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::ffi;
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GdkWin32Surface")]
    pub struct Win32Surface(Object<ffi::GdkWin32Surface, ffi::GdkWin32SurfaceClass>) @extends gdk::Surface;

    match fn {
        type_ => || ffi::gdk_win32_surface_get_type(),
    }
}

impl Win32Surface {
    #[doc(alias = "gdk_win32_surface_set_urgency_hint")]
    pub fn set_urgency_hint(&self, urgent: bool) {
        unsafe {
            ffi::gdk_win32_surface_set_urgency_hint(self.to_glib_none().0, urgent.into_glib());
        }
    }

    #[cfg_attr(feature = "v4_8", deprecated = "Since 4.8")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_win32_surface_is_win32")]
    pub fn is_win32(surface: &impl IsA<gdk::Surface>) -> bool {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(ffi::gdk_win32_surface_is_win32(
                surface.as_ref().to_glib_none().0,
            ))
        }
    }
}