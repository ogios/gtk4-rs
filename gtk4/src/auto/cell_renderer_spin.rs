// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{Adjustment, CellRenderer, CellRendererMode, CellRendererText};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkCellRendererSpin")]
    pub struct CellRendererSpin(Object<ffi::GtkCellRendererSpin>) @extends CellRendererText, CellRenderer;

    match fn {
        type_ => || ffi::gtk_cell_renderer_spin_get_type(),
    }
}

impl CellRendererSpin {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_renderer_spin_new")]
    pub fn new() -> CellRendererSpin {
        assert_initialized_main_thread!();
        unsafe { CellRenderer::from_glib_none(ffi::gtk_cell_renderer_spin_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellRendererSpin`] objects.
    ///
    /// This method returns an instance of [`CellRendererSpinBuilder`](crate::builders::CellRendererSpinBuilder) which can be used to create [`CellRendererSpin`] objects.
    pub fn builder() -> CellRendererSpinBuilder {
        CellRendererSpinBuilder::new()
    }

    pub fn adjustment(&self) -> Option<Adjustment> {
        ObjectExt::property(self, "adjustment")
    }

    pub fn set_adjustment<P: IsA<Adjustment>>(&self, adjustment: Option<&P>) {
        ObjectExt::set_property(self, "adjustment", adjustment)
    }

    #[doc(alias = "climb-rate")]
    pub fn climb_rate(&self) -> f64 {
        ObjectExt::property(self, "climb-rate")
    }

    #[doc(alias = "climb-rate")]
    pub fn set_climb_rate(&self, climb_rate: f64) {
        ObjectExt::set_property(self, "climb-rate", climb_rate)
    }

    pub fn digits(&self) -> u32 {
        ObjectExt::property(self, "digits")
    }

    pub fn set_digits(&self, digits: u32) {
        ObjectExt::set_property(self, "digits", digits)
    }

    #[doc(alias = "adjustment")]
    pub fn connect_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_adjustment_trampoline<F: Fn(&CellRendererSpin) + 'static>(
            this: *mut ffi::GtkCellRendererSpin,
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
                b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_adjustment_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "climb-rate")]
    pub fn connect_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_climb_rate_trampoline<F: Fn(&CellRendererSpin) + 'static>(
            this: *mut ffi::GtkCellRendererSpin,
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
                b"notify::climb-rate\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_climb_rate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "digits")]
    pub fn connect_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_digits_trampoline<F: Fn(&CellRendererSpin) + 'static>(
            this: *mut ffi::GtkCellRendererSpin,
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
                b"notify::digits\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_digits_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellRendererSpin {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellRendererSpin`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellRendererSpinBuilder {
    builder: glib::object::ObjectBuilder<'static, CellRendererSpin>,
}

impl CellRendererSpinBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn adjustment(self, adjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("adjustment", adjustment.clone().upcast()),
        }
    }

    pub fn climb_rate(self, climb_rate: f64) -> Self {
        Self {
            builder: self.builder.property("climb-rate", climb_rate),
        }
    }

    pub fn digits(self, digits: u32) -> Self {
        Self {
            builder: self.builder.property("digits", digits),
        }
    }

    pub fn align_set(self, align_set: bool) -> Self {
        Self {
            builder: self.builder.property("align-set", align_set),
        }
    }

    pub fn alignment(self, alignment: pango::Alignment) -> Self {
        Self {
            builder: self.builder.property("alignment", alignment),
        }
    }

    pub fn attributes(self, attributes: &pango::AttrList) -> Self {
        Self {
            builder: self.builder.property("attributes", attributes.clone()),
        }
    }

    pub fn background(self, background: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("background", background.into()),
        }
    }

    pub fn background_rgba(self, background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self.builder.property("background-rgba", background_rgba),
        }
    }

    pub fn background_set(self, background_set: bool) -> Self {
        Self {
            builder: self.builder.property("background-set", background_set),
        }
    }

    pub fn editable(self, editable: bool) -> Self {
        Self {
            builder: self.builder.property("editable", editable),
        }
    }

    pub fn editable_set(self, editable_set: bool) -> Self {
        Self {
            builder: self.builder.property("editable-set", editable_set),
        }
    }

    pub fn ellipsize(self, ellipsize: pango::EllipsizeMode) -> Self {
        Self {
            builder: self.builder.property("ellipsize", ellipsize),
        }
    }

    pub fn ellipsize_set(self, ellipsize_set: bool) -> Self {
        Self {
            builder: self.builder.property("ellipsize-set", ellipsize_set),
        }
    }

    pub fn family(self, family: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("family", family.into()),
        }
    }

    pub fn family_set(self, family_set: bool) -> Self {
        Self {
            builder: self.builder.property("family-set", family_set),
        }
    }

    pub fn font(self, font: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("font", font.into()),
        }
    }

    pub fn font_desc(self, font_desc: &pango::FontDescription) -> Self {
        Self {
            builder: self.builder.property("font-desc", font_desc),
        }
    }

    pub fn foreground(self, foreground: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("foreground", foreground.into()),
        }
    }

    pub fn foreground_rgba(self, foreground_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self.builder.property("foreground-rgba", foreground_rgba),
        }
    }

    pub fn foreground_set(self, foreground_set: bool) -> Self {
        Self {
            builder: self.builder.property("foreground-set", foreground_set),
        }
    }

    pub fn language(self, language: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("language", language.into()),
        }
    }

    pub fn language_set(self, language_set: bool) -> Self {
        Self {
            builder: self.builder.property("language-set", language_set),
        }
    }

    pub fn markup(self, markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("markup", markup.into()),
        }
    }

    pub fn max_width_chars(self, max_width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("max-width-chars", max_width_chars),
        }
    }

    pub fn placeholder_text(self, placeholder_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("placeholder-text", placeholder_text.into()),
        }
    }

    pub fn rise(self, rise: i32) -> Self {
        Self {
            builder: self.builder.property("rise", rise),
        }
    }

    pub fn rise_set(self, rise_set: bool) -> Self {
        Self {
            builder: self.builder.property("rise-set", rise_set),
        }
    }

    pub fn scale(self, scale: f64) -> Self {
        Self {
            builder: self.builder.property("scale", scale),
        }
    }

    pub fn scale_set(self, scale_set: bool) -> Self {
        Self {
            builder: self.builder.property("scale-set", scale_set),
        }
    }

    pub fn single_paragraph_mode(self, single_paragraph_mode: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("single-paragraph-mode", single_paragraph_mode),
        }
    }

    pub fn size(self, size: i32) -> Self {
        Self {
            builder: self.builder.property("size", size),
        }
    }

    pub fn size_points(self, size_points: f64) -> Self {
        Self {
            builder: self.builder.property("size-points", size_points),
        }
    }

    pub fn size_set(self, size_set: bool) -> Self {
        Self {
            builder: self.builder.property("size-set", size_set),
        }
    }

    pub fn stretch(self, stretch: pango::Stretch) -> Self {
        Self {
            builder: self.builder.property("stretch", stretch),
        }
    }

    pub fn stretch_set(self, stretch_set: bool) -> Self {
        Self {
            builder: self.builder.property("stretch-set", stretch_set),
        }
    }

    pub fn strikethrough(self, strikethrough: bool) -> Self {
        Self {
            builder: self.builder.property("strikethrough", strikethrough),
        }
    }

    pub fn strikethrough_set(self, strikethrough_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("strikethrough-set", strikethrough_set),
        }
    }

    pub fn style(self, style: pango::Style) -> Self {
        Self {
            builder: self.builder.property("style", style),
        }
    }

    pub fn style_set(self, style_set: bool) -> Self {
        Self {
            builder: self.builder.property("style-set", style_set),
        }
    }

    pub fn text(self, text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("text", text.into()),
        }
    }

    pub fn underline(self, underline: pango::Underline) -> Self {
        Self {
            builder: self.builder.property("underline", underline),
        }
    }

    pub fn underline_set(self, underline_set: bool) -> Self {
        Self {
            builder: self.builder.property("underline-set", underline_set),
        }
    }

    pub fn variant(self, variant: pango::Variant) -> Self {
        Self {
            builder: self.builder.property("variant", variant),
        }
    }

    pub fn variant_set(self, variant_set: bool) -> Self {
        Self {
            builder: self.builder.property("variant-set", variant_set),
        }
    }

    pub fn weight(self, weight: i32) -> Self {
        Self {
            builder: self.builder.property("weight", weight),
        }
    }

    pub fn weight_set(self, weight_set: bool) -> Self {
        Self {
            builder: self.builder.property("weight-set", weight_set),
        }
    }

    pub fn width_chars(self, width_chars: i32) -> Self {
        Self {
            builder: self.builder.property("width-chars", width_chars),
        }
    }

    pub fn wrap_mode(self, wrap_mode: pango::WrapMode) -> Self {
        Self {
            builder: self.builder.property("wrap-mode", wrap_mode),
        }
    }

    pub fn wrap_width(self, wrap_width: i32) -> Self {
        Self {
            builder: self.builder.property("wrap-width", wrap_width),
        }
    }

    pub fn cell_background(self, cell_background: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background", cell_background.into()),
        }
    }

    pub fn cell_background_rgba(self, cell_background_rgba: &gdk::RGBA) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-rgba", cell_background_rgba),
        }
    }

    pub fn cell_background_set(self, cell_background_set: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("cell-background-set", cell_background_set),
        }
    }

    pub fn height(self, height: i32) -> Self {
        Self {
            builder: self.builder.property("height", height),
        }
    }

    pub fn is_expanded(self, is_expanded: bool) -> Self {
        Self {
            builder: self.builder.property("is-expanded", is_expanded),
        }
    }

    pub fn is_expander(self, is_expander: bool) -> Self {
        Self {
            builder: self.builder.property("is-expander", is_expander),
        }
    }

    pub fn mode(self, mode: CellRendererMode) -> Self {
        Self {
            builder: self.builder.property("mode", mode),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width(self, width: i32) -> Self {
        Self {
            builder: self.builder.property("width", width),
        }
    }

    pub fn xalign(self, xalign: f32) -> Self {
        Self {
            builder: self.builder.property("xalign", xalign),
        }
    }

    pub fn xpad(self, xpad: u32) -> Self {
        Self {
            builder: self.builder.property("xpad", xpad),
        }
    }

    pub fn yalign(self, yalign: f32) -> Self {
        Self {
            builder: self.builder.property("yalign", yalign),
        }
    }

    pub fn ypad(self, ypad: u32) -> Self {
        Self {
            builder: self.builder.property("ypad", ypad),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellRendererSpin`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellRendererSpin {
        self.builder.build()
    }
}

impl fmt::Display for CellRendererSpin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellRendererSpin")
    }
}
