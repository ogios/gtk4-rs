// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    Accessible, AccessibleRole, Align, Buildable, CellEditable, CellLayout, ConstraintTarget,
    LayoutManager, Overflow, ScrollType, SensitivityType, TreeIter, TreeModel, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkComboBox")]
    pub struct ComboBox(Object<ffi::GtkComboBox, ffi::GtkComboBoxClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, CellEditable, CellLayout;

    match fn {
        type_ => || ffi::gtk_combo_box_get_type(),
    }
}

impl ComboBox {
    pub const NONE: Option<&'static ComboBox> = None;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_new")]
    pub fn new() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_combo_box_new()).unsafe_cast() }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_new_with_entry")]
    #[doc(alias = "new_with_entry")]
    pub fn with_entry() -> ComboBox {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_combo_box_new_with_entry()).unsafe_cast() }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_new_with_model")]
    #[doc(alias = "new_with_model")]
    pub fn with_model(model: &impl IsA<TreeModel>) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model(
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_new_with_model_and_entry")]
    #[doc(alias = "new_with_model_and_entry")]
    pub fn with_model_and_entry(model: &impl IsA<TreeModel>) -> ComboBox {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_combo_box_new_with_model_and_entry(
                model.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ComboBox`] objects.
    ///
    /// This method returns an instance of [`ComboBoxBuilder`](crate::builders::ComboBoxBuilder) which can be used to create [`ComboBox`] objects.
    pub fn builder() -> ComboBoxBuilder {
        ComboBoxBuilder::new()
    }
}

impl Default for ComboBox {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ComboBox`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ComboBoxBuilder {
    builder: glib::object::ObjectBuilder<'static, ComboBox>,
}

impl ComboBoxBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn active(self, active: i32) -> Self {
        Self {
            builder: self.builder.property("active", active),
        }
    }

    pub fn active_id(self, active_id: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("active-id", active_id.into()),
        }
    }

    pub fn button_sensitivity(self, button_sensitivity: SensitivityType) -> Self {
        Self {
            builder: self
                .builder
                .property("button-sensitivity", button_sensitivity),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn entry_text_column(self, entry_text_column: i32) -> Self {
        Self {
            builder: self
                .builder
                .property("entry-text-column", entry_text_column),
        }
    }

    pub fn has_entry(self, has_entry: bool) -> Self {
        Self {
            builder: self.builder.property("has-entry", has_entry),
        }
    }

    pub fn has_frame(self, has_frame: bool) -> Self {
        Self {
            builder: self.builder.property("has-frame", has_frame),
        }
    }

    pub fn id_column(self, id_column: i32) -> Self {
        Self {
            builder: self.builder.property("id-column", id_column),
        }
    }

    pub fn model(self, model: &impl IsA<TreeModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
        }
    }

    pub fn popup_fixed_width(self, popup_fixed_width: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("popup-fixed-width", popup_fixed_width),
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

    pub fn editing_canceled(self, editing_canceled: bool) -> Self {
        Self {
            builder: self.builder.property("editing-canceled", editing_canceled),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ComboBox`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ComboBox {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ComboBox>> Sealed for T {}
}

pub trait ComboBoxExt: IsA<ComboBox> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_active_id")]
    #[doc(alias = "get_active_id")]
    fn active_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_combo_box_get_active_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_active_iter")]
    #[doc(alias = "get_active_iter")]
    fn active_iter(&self) -> Option<TreeIter> {
        unsafe {
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(ffi::gtk_combo_box_get_active_iter(
                self.as_ref().to_glib_none().0,
                iter.to_glib_none_mut().0,
            ));
            if ret {
                Some(iter)
            } else {
                None
            }
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_button_sensitivity")]
    #[doc(alias = "get_button_sensitivity")]
    fn button_sensitivity(&self) -> SensitivityType {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_button_sensitivity(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_combo_box_get_child(self.as_ref().to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_entry_text_column")]
    #[doc(alias = "get_entry_text_column")]
    fn entry_text_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_entry_text_column(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_has_entry")]
    #[doc(alias = "get_has_entry")]
    fn has_entry(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_has_entry(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_id_column")]
    #[doc(alias = "get_id_column")]
    fn id_column(&self) -> i32 {
        unsafe { ffi::gtk_combo_box_get_id_column(self.as_ref().to_glib_none().0) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_model")]
    #[doc(alias = "get_model")]
    fn model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_combo_box_get_model(self.as_ref().to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_get_popup_fixed_width")]
    #[doc(alias = "get_popup_fixed_width")]
    fn is_popup_fixed_width(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_get_popup_fixed_width(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_popdown")]
    fn popdown(&self) {
        unsafe {
            ffi::gtk_combo_box_popdown(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_popup")]
    fn popup(&self) {
        unsafe {
            ffi::gtk_combo_box_popup(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_popup_for_device")]
    fn popup_for_device(&self, device: &gdk::Device) {
        unsafe {
            ffi::gtk_combo_box_popup_for_device(
                self.as_ref().to_glib_none().0,
                device.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_active_id")]
    fn set_active_id(&self, active_id: Option<&str>) -> bool {
        unsafe {
            from_glib(ffi::gtk_combo_box_set_active_id(
                self.as_ref().to_glib_none().0,
                active_id.to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_active_iter")]
    fn set_active_iter(&self, iter: Option<&TreeIter>) {
        unsafe {
            ffi::gtk_combo_box_set_active_iter(
                self.as_ref().to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_button_sensitivity")]
    fn set_button_sensitivity(&self, sensitivity: SensitivityType) {
        unsafe {
            ffi::gtk_combo_box_set_button_sensitivity(
                self.as_ref().to_glib_none().0,
                sensitivity.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_child")]
    fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_combo_box_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_entry_text_column")]
    fn set_entry_text_column(&self, text_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_entry_text_column(self.as_ref().to_glib_none().0, text_column);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_id_column")]
    fn set_id_column(&self, id_column: i32) {
        unsafe {
            ffi::gtk_combo_box_set_id_column(self.as_ref().to_glib_none().0, id_column);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_model")]
    fn set_model(&self, model: Option<&impl IsA<TreeModel>>) {
        unsafe {
            ffi::gtk_combo_box_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_popup_fixed_width")]
    fn set_popup_fixed_width(&self, fixed: bool) {
        unsafe {
            ffi::gtk_combo_box_set_popup_fixed_width(
                self.as_ref().to_glib_none().0,
                fixed.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_combo_box_set_row_separator_func")]
    fn set_row_separator_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(&self, func: P) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            model: *mut ffi::GtkTreeModel,
            iter: *mut ffi::GtkTreeIter,
            data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let model = from_glib_borrow(model);
            let iter = from_glib_borrow(iter);
            let callback: &P = &*(data as *mut _);
            (*callback)(&model, &iter).into_glib()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<P: Fn(&TreeModel, &TreeIter) -> bool + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_combo_box_set_row_separator_func(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "has-frame")]
    fn has_frame(&self) -> bool {
        ObjectExt::property(self.as_ref(), "has-frame")
    }

    #[doc(alias = "has-frame")]
    fn set_has_frame(&self, has_frame: bool) {
        ObjectExt::set_property(self.as_ref(), "has-frame", has_frame)
    }

    #[doc(alias = "popup-shown")]
    fn is_popup_shown(&self) -> bool {
        ObjectExt::property(self.as_ref(), "popup-shown")
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "format-entry-text")]
    fn connect_format_entry_text<F: Fn(&Self, &str) -> String + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn format_entry_text_trampoline<
            P: IsA<ComboBox>,
            F: Fn(&P, &str) -> String + 'static,
        >(
            this: *mut ffi::GtkComboBox,
            path: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let f: &F = &*(f as *const F);
            f(
                ComboBox::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(path),
            )
            .to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"format-entry-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    format_entry_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "move-active")]
    fn connect_move_active<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn move_active_trampoline<
            P: IsA<ComboBox>,
            F: Fn(&P, ScrollType) + 'static,
        >(
            this: *mut ffi::GtkComboBox,
            scroll_type: ffi::GtkScrollType,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                ComboBox::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(scroll_type),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_active(&self, scroll_type: ScrollType) {
        self.emit_by_name::<()>("move-active", &[&scroll_type]);
    }

    #[doc(alias = "popdown")]
    fn connect_popdown<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popdown_trampoline<P: IsA<ComboBox>, F: Fn(&P) -> bool + 'static>(
            this: *mut ffi::GtkComboBox,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popdown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    popdown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_popdown(&self) -> bool {
        self.emit_by_name("popdown", &[])
    }

    #[doc(alias = "popup")]
    fn connect_popup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn popup_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_popup(&self) {
        self.emit_by_name::<()>("popup", &[]);
    }

    #[doc(alias = "active")]
    fn connect_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "active-id")]
    fn connect_active_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_id_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "button-sensitivity")]
    fn connect_button_sensitivity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_button_sensitivity_trampoline<
            P: IsA<ComboBox>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::button-sensitivity\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_button_sensitivity_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "entry-text-column")]
    fn connect_entry_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_entry_text_column_trampoline<
            P: IsA<ComboBox>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::entry-text-column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_entry_text_column_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-frame")]
    fn connect_has_frame_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_frame_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-frame\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_frame_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "id-column")]
    fn connect_id_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_column_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id-column\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_column_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P: IsA<ComboBox>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "popup-fixed-width")]
    fn connect_popup_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_fixed_width_trampoline<
            P: IsA<ComboBox>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::popup-fixed-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popup_fixed_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "popup-shown")]
    fn connect_popup_shown_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_popup_shown_trampoline<
            P: IsA<ComboBox>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkComboBox,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ComboBox::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::popup-shown\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_popup_shown_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ComboBox>> ComboBoxExt for O {}

impl fmt::Display for ComboBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ComboBox")
    }
}
