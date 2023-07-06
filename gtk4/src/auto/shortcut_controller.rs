// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Buildable, EventController, Shortcut, ShortcutScope};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkShortcutController")]
    pub struct ShortcutController(Object<ffi::GtkShortcutController, ffi::GtkShortcutControllerClass>) @extends EventController, @implements gio::ListModel, Buildable;

    match fn {
        type_ => || ffi::gtk_shortcut_controller_get_type(),
    }
}

impl ShortcutController {
    #[doc(alias = "gtk_shortcut_controller_new")]
    pub fn new() -> ShortcutController {
        assert_initialized_main_thread!();
        unsafe { EventController::from_glib_full(ffi::gtk_shortcut_controller_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_shortcut_controller_new_for_model")]
    #[doc(alias = "new_for_model")]
    pub fn for_model(model: &impl IsA<gio::ListModel>) -> ShortcutController {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_shortcut_controller_new_for_model(
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_shortcut_controller_add_shortcut")]
    pub fn add_shortcut(&self, shortcut: Shortcut) {
        unsafe {
            ffi::gtk_shortcut_controller_add_shortcut(
                self.to_glib_none().0,
                shortcut.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "gtk_shortcut_controller_get_mnemonics_modifiers")]
    #[doc(alias = "get_mnemonics_modifiers")]
    pub fn mnemonics_modifiers(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_shortcut_controller_get_mnemonics_modifiers(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_controller_get_scope")]
    #[doc(alias = "get_scope")]
    pub fn scope(&self) -> ShortcutScope {
        unsafe {
            from_glib(ffi::gtk_shortcut_controller_get_scope(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_shortcut_controller_remove_shortcut")]
    pub fn remove_shortcut(&self, shortcut: &Shortcut) {
        unsafe {
            ffi::gtk_shortcut_controller_remove_shortcut(
                self.to_glib_none().0,
                shortcut.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_shortcut_controller_set_mnemonics_modifiers")]
    pub fn set_mnemonics_modifiers(&self, modifiers: gdk::ModifierType) {
        unsafe {
            ffi::gtk_shortcut_controller_set_mnemonics_modifiers(
                self.to_glib_none().0,
                modifiers.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_shortcut_controller_set_scope")]
    pub fn set_scope(&self, scope: ShortcutScope) {
        unsafe {
            ffi::gtk_shortcut_controller_set_scope(self.to_glib_none().0, scope.into_glib());
        }
    }

    #[doc(alias = "mnemonic-modifiers")]
    pub fn mnemonic_modifiers(&self) -> gdk::ModifierType {
        ObjectExt::property(self, "mnemonic-modifiers")
    }

    #[doc(alias = "mnemonic-modifiers")]
    pub fn set_mnemonic_modifiers(&self, mnemonic_modifiers: gdk::ModifierType) {
        ObjectExt::set_property(self, "mnemonic-modifiers", mnemonic_modifiers)
    }

    #[doc(alias = "mnemonic-modifiers")]
    pub fn connect_mnemonic_modifiers_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mnemonic_modifiers_trampoline<
            F: Fn(&ShortcutController) + 'static,
        >(
            this: *mut ffi::GtkShortcutController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mnemonic-modifiers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mnemonic_modifiers_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scope")]
    pub fn connect_scope_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scope_trampoline<F: Fn(&ShortcutController) + 'static>(
            this: *mut ffi::GtkShortcutController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scope\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_scope_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ShortcutController {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ShortcutController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ShortcutController")
    }
}
