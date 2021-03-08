// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Gravity;
use crate::PopupLayout;
use crate::Surface;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct Popup(Interface<ffi::GdkPopup, ffi::GdkPopupInterface>) @requires Surface;

    match fn {
        get_type => || ffi::gdk_popup_get_type(),
    }
}

pub const NONE_POPUP: Option<&Popup> = None;

pub trait PopupExt: 'static {
    #[doc(alias = "gdk_popup_get_autohide")]
    fn get_autohide(&self) -> bool;

    #[doc(alias = "gdk_popup_get_parent")]
    fn get_parent(&self) -> Option<Surface>;

    #[doc(alias = "gdk_popup_get_position_x")]
    fn get_position_x(&self) -> i32;

    #[doc(alias = "gdk_popup_get_position_y")]
    fn get_position_y(&self) -> i32;

    #[doc(alias = "gdk_popup_get_rect_anchor")]
    fn get_rect_anchor(&self) -> Gravity;

    #[doc(alias = "gdk_popup_get_surface_anchor")]
    fn get_surface_anchor(&self) -> Gravity;

    #[doc(alias = "gdk_popup_present")]
    fn present(&self, width: i32, height: i32, layout: &PopupLayout) -> bool;
}

impl<O: IsA<Popup>> PopupExt for O {
    fn get_autohide(&self) -> bool {
        unsafe { from_glib(ffi::gdk_popup_get_autohide(self.as_ref().to_glib_none().0)) }
    }

    fn get_parent(&self) -> Option<Surface> {
        unsafe { from_glib_none(ffi::gdk_popup_get_parent(self.as_ref().to_glib_none().0)) }
    }

    fn get_position_x(&self) -> i32 {
        unsafe { ffi::gdk_popup_get_position_x(self.as_ref().to_glib_none().0) }
    }

    fn get_position_y(&self) -> i32 {
        unsafe { ffi::gdk_popup_get_position_y(self.as_ref().to_glib_none().0) }
    }

    fn get_rect_anchor(&self) -> Gravity {
        unsafe {
            from_glib(ffi::gdk_popup_get_rect_anchor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_surface_anchor(&self) -> Gravity {
        unsafe {
            from_glib(ffi::gdk_popup_get_surface_anchor(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn present(&self, width: i32, height: i32, layout: &PopupLayout) -> bool {
        unsafe {
            from_glib(ffi::gdk_popup_present(
                self.as_ref().to_glib_none().0,
                width,
                height,
                layout.to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Popup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Popup")
    }
}
