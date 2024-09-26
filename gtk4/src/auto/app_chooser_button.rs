// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Accessible, AccessibleRole, Align, AppChooser, Buildable, ConstraintTarget, LayoutManager,
    Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkAppChooserButton")]
    pub struct AppChooserButton(Object<ffi::GtkAppChooserButton>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, AppChooser;

    match fn {
        type_ => || ffi::gtk_app_chooser_button_get_type(),
    }
}

impl AppChooserButton {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_new")]
    pub fn new(content_type: &str) -> AppChooserButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_button_new(
                content_type.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AppChooserButton`] objects.
    ///
    /// This method returns an instance of [`AppChooserButtonBuilder`](crate::builders::AppChooserButtonBuilder) which can be used to create [`AppChooserButton`] objects.
    pub fn builder() -> AppChooserButtonBuilder {
        AppChooserButtonBuilder::new()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_append_custom_item")]
    pub fn append_custom_item(&self, name: &str, label: &str, icon: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::gtk_app_chooser_button_append_custom_item(
                self.to_glib_none().0,
                name.to_glib_none().0,
                label.to_glib_none().0,
                icon.as_ref().to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_append_separator")]
    pub fn append_separator(&self) {
        unsafe {
            ffi::gtk_app_chooser_button_append_separator(self.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_get_heading")]
    #[doc(alias = "get_heading")]
    pub fn heading(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_button_get_heading(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_get_modal")]
    #[doc(alias = "get_modal")]
    #[doc(alias = "modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_app_chooser_button_get_modal(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_get_show_default_item")]
    #[doc(alias = "get_show_default_item")]
    #[doc(alias = "show-default-item")]
    pub fn shows_default_item(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_button_get_show_default_item(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_get_show_dialog_item")]
    #[doc(alias = "get_show_dialog_item")]
    #[doc(alias = "show-dialog-item")]
    pub fn shows_dialog_item(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_button_get_show_dialog_item(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_set_active_custom_item")]
    pub fn set_active_custom_item(&self, name: &str) {
        unsafe {
            ffi::gtk_app_chooser_button_set_active_custom_item(
                self.to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_set_heading")]
    #[doc(alias = "heading")]
    pub fn set_heading(&self, heading: &str) {
        unsafe {
            ffi::gtk_app_chooser_button_set_heading(
                self.to_glib_none().0,
                heading.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_set_modal")]
    #[doc(alias = "modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_app_chooser_button_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_set_show_default_item")]
    #[doc(alias = "show-default-item")]
    pub fn set_show_default_item(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_button_set_show_default_item(
                self.to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_app_chooser_button_set_show_dialog_item")]
    #[doc(alias = "show-dialog-item")]
    pub fn set_show_dialog_item(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_button_set_show_dialog_item(
                self.to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<F: Fn(&AppChooserButton) + 'static>(
            this: *mut ffi::GtkAppChooserButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    pub fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "changed")]
    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&AppChooserButton) + 'static>(
            this: *mut ffi::GtkAppChooserButton,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "custom-item-activated")]
    pub fn connect_custom_item_activated<F: Fn(&Self, &str) + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn custom_item_activated_trampoline<
            F: Fn(&AppChooserButton, &str) + 'static,
        >(
            this: *mut ffi::GtkAppChooserButton,
            item_name: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                &glib::GString::from_glib_borrow(item_name),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name =
                detail.map(|name| format!("custom-item-activated::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(&b"custom-item-activated\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    custom_item_activated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "heading")]
    pub fn connect_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_heading_trampoline<F: Fn(&AppChooserButton) + 'static>(
            this: *mut ffi::GtkAppChooserButton,
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
                b"notify::heading\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_heading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&AppChooserButton) + 'static>(
            this: *mut ffi::GtkAppChooserButton,
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
                b"notify::modal\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_modal_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-default-item")]
    pub fn connect_show_default_item_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_default_item_trampoline<
            F: Fn(&AppChooserButton) + 'static,
        >(
            this: *mut ffi::GtkAppChooserButton,
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
                b"notify::show-default-item\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_default_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "show-dialog-item")]
    pub fn connect_show_dialog_item_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_dialog_item_trampoline<
            F: Fn(&AppChooserButton) + 'static,
        >(
            this: *mut ffi::GtkAppChooserButton,
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
                b"notify::show-dialog-item\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_show_dialog_item_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for AppChooserButton {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AppChooserButton`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AppChooserButtonBuilder {
    builder: glib::object::ObjectBuilder<'static, AppChooserButton>,
}

impl AppChooserButtonBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn heading(self, heading: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("heading", heading.into()),
        }
    }

    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    pub fn show_default_item(self, show_default_item: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("show-default-item", show_default_item),
        }
    }

    pub fn show_dialog_item(self, show_dialog_item: bool) -> Self {
        Self {
            builder: self.builder.property("show-dialog-item", show_dialog_item),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn can_target(self, can_target: bool) -> Self {
        Self {
            builder: self.builder.property("can-target", can_target),
        }
    }

    pub fn css_classes(self, css_classes: impl Into<glib::StrV>) -> Self {
        Self {
            builder: self.builder.property("css-classes", css_classes.into()),
        }
    }

    pub fn css_name(self, css_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("css-name", css_name.into()),
        }
    }

    pub fn cursor(self, cursor: &gdk::Cursor) -> Self {
        Self {
            builder: self.builder.property("cursor", cursor.clone()),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn focusable(self, focusable: bool) -> Self {
        Self {
            builder: self.builder.property("focusable", focusable),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn layout_manager(self, layout_manager: &impl IsA<LayoutManager>) -> Self {
        Self {
            builder: self
                .builder
                .property("layout-manager", layout_manager.clone().upcast()),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn overflow(self, overflow: Overflow) -> Self {
        Self {
            builder: self.builder.property("overflow", overflow),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn accessible_role(self, accessible_role: AccessibleRole) -> Self {
        Self {
            builder: self.builder.property("accessible-role", accessible_role),
        }
    }

    pub fn content_type(self, content_type: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("content-type", content_type.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AppChooserButton`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AppChooserButton {
        self.builder.build()
    }
}
