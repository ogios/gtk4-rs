// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, LayoutManager,
    MediaStream, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkMediaControls")]
    pub struct MediaControls(Object<ffi::GtkMediaControls, ffi::GtkMediaControlsClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_media_controls_get_type(),
    }
}

impl MediaControls {
    #[doc(alias = "gtk_media_controls_new")]
    pub fn new(stream: Option<&impl IsA<MediaStream>>) -> MediaControls {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_media_controls_new(
                stream.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`MediaControls`] objects.
    ///
    /// This method returns an instance of [`MediaControlsBuilder`](crate::builders::MediaControlsBuilder) which can be used to create [`MediaControls`] objects.
    pub fn builder() -> MediaControlsBuilder {
        MediaControlsBuilder::new()
    }

    #[doc(alias = "gtk_media_controls_get_media_stream")]
    #[doc(alias = "get_media_stream")]
    #[doc(alias = "media-stream")]
    pub fn media_stream(&self) -> Option<MediaStream> {
        unsafe {
            from_glib_none(ffi::gtk_media_controls_get_media_stream(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_media_controls_set_media_stream")]
    #[doc(alias = "media-stream")]
    pub fn set_media_stream(&self, stream: Option<&impl IsA<MediaStream>>) {
        unsafe {
            ffi::gtk_media_controls_set_media_stream(
                self.to_glib_none().0,
                stream.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "media-stream")]
    pub fn connect_media_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_media_stream_trampoline<F: Fn(&MediaControls) + 'static>(
            this: *mut ffi::GtkMediaControls,
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
                b"notify::media-stream\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_media_stream_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for MediaControls {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`MediaControls`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MediaControlsBuilder {
    builder: glib::object::ObjectBuilder<'static, MediaControls>,
}

impl MediaControlsBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn media_stream(self, media_stream: &impl IsA<MediaStream>) -> Self {
        Self {
            builder: self
                .builder
                .property("media-stream", media_stream.clone().upcast()),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`MediaControls`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MediaControls {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
