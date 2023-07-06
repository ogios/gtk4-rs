// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    Accessible, AccessibleRole, Align, Buildable, CellArea, CellAreaContext, CellLayout,
    ConstraintTarget, LayoutManager, Orientable, Orientation, Overflow, TreeModel, TreePath,
    Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellView")]
    pub struct CellView(Object<ffi::GtkCellView>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_view_get_type(),
    }
}

impl CellView {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_new")]
    pub fn new() -> CellView {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_cell_view_new()).unsafe_cast() }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_new_with_context")]
    #[doc(alias = "new_with_context")]
    pub fn with_context(
        area: &impl IsA<CellArea>,
        context: &impl IsA<CellAreaContext>,
    ) -> CellView {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_context(
                area.as_ref().to_glib_none().0,
                context.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_new_with_markup")]
    #[doc(alias = "new_with_markup")]
    pub fn with_markup(markup: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_markup(markup.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_new_with_text")]
    #[doc(alias = "new_with_text")]
    pub fn with_text(text: &str) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_text(text.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_new_with_texture")]
    #[doc(alias = "new_with_texture")]
    pub fn with_texture(texture: &impl IsA<gdk::Texture>) -> CellView {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_cell_view_new_with_texture(
                texture.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellView`] objects.
    ///
    /// This method returns an instance of [`CellViewBuilder`](crate::builders::CellViewBuilder) which can be used to create [`CellView`] objects.
    pub fn builder() -> CellViewBuilder {
        CellViewBuilder::new()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_get_displayed_row")]
    #[doc(alias = "get_displayed_row")]
    pub fn displayed_row(&self) -> Option<TreePath> {
        unsafe { from_glib_full(ffi::gtk_cell_view_get_displayed_row(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_get_draw_sensitive")]
    #[doc(alias = "get_draw_sensitive")]
    pub fn draws_sensitive(&self) -> bool {
        unsafe { from_glib(ffi::gtk_cell_view_get_draw_sensitive(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_get_fit_model")]
    #[doc(alias = "get_fit_model")]
    pub fn fits_model(&self) -> bool {
        unsafe { from_glib(ffi::gtk_cell_view_get_fit_model(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_get_model")]
    #[doc(alias = "get_model")]
    pub fn model(&self) -> Option<TreeModel> {
        unsafe { from_glib_none(ffi::gtk_cell_view_get_model(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_set_displayed_row")]
    pub fn set_displayed_row(&self, path: Option<&TreePath>) {
        unsafe {
            ffi::gtk_cell_view_set_displayed_row(
                self.to_glib_none().0,
                mut_override(path.to_glib_none().0),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_set_draw_sensitive")]
    pub fn set_draw_sensitive(&self, draw_sensitive: bool) {
        unsafe {
            ffi::gtk_cell_view_set_draw_sensitive(
                self.to_glib_none().0,
                draw_sensitive.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_set_fit_model")]
    pub fn set_fit_model(&self, fit_model: bool) {
        unsafe {
            ffi::gtk_cell_view_set_fit_model(self.to_glib_none().0, fit_model.into_glib());
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_view_set_model")]
    pub fn set_model(&self, model: Option<&impl IsA<TreeModel>>) {
        unsafe {
            ffi::gtk_cell_view_set_model(
                self.to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "cell-area")]
    pub fn cell_area(&self) -> Option<CellArea> {
        ObjectExt::property(self, "cell-area")
    }

    #[doc(alias = "cell-area-context")]
    pub fn cell_area_context(&self) -> Option<CellAreaContext> {
        ObjectExt::property(self, "cell-area-context")
    }

    #[doc(alias = "draw-sensitive")]
    pub fn connect_draw_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_draw_sensitive_trampoline<F: Fn(&CellView) + 'static>(
            this: *mut ffi::GtkCellView,
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
                b"notify::draw-sensitive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_draw_sensitive_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fit-model")]
    pub fn connect_fit_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fit_model_trampoline<F: Fn(&CellView) + 'static>(
            this: *mut ffi::GtkCellView,
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
                b"notify::fit-model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fit_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "model")]
    pub fn connect_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<F: Fn(&CellView) + 'static>(
            this: *mut ffi::GtkCellView,
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
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellView {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellViewBuilder {
    builder: glib::object::ObjectBuilder<'static, CellView>,
}

impl CellViewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn cell_area(self, cell_area: &impl IsA<CellArea>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-area", cell_area.clone().upcast()),
        }
    }

    pub fn cell_area_context(self, cell_area_context: &impl IsA<CellAreaContext>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-area-context", cell_area_context.clone().upcast()),
        }
    }

    pub fn draw_sensitive(self, draw_sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("draw-sensitive", draw_sensitive),
        }
    }

    pub fn fit_model(self, fit_model: bool) -> Self {
        Self {
            builder: self.builder.property("fit-model", fit_model),
        }
    }

    pub fn model(self, model: &impl IsA<TreeModel>) -> Self {
        Self {
            builder: self.builder.property("model", model.clone().upcast()),
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

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellView {
        self.builder.build()
    }
}

impl fmt::Display for CellView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellView")
    }
}
