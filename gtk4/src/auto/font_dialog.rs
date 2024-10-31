// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Filter, Window};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "GtkFontDialog")]
    pub struct FontDialog(Object<ffi::GtkFontDialog, ffi::GtkFontDialogClass>);

    match fn {
        type_ => || ffi::gtk_font_dialog_get_type(),
    }
}

impl FontDialog {
    #[doc(alias = "gtk_font_dialog_new")]
    pub fn new() -> FontDialog {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_font_dialog_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FontDialog`] objects.
    ///
    /// This method returns an instance of [`FontDialogBuilder`](crate::builders::FontDialogBuilder) which can be used to create [`FontDialog`] objects.
    pub fn builder() -> FontDialogBuilder {
        FontDialogBuilder::new()
    }

    #[doc(alias = "gtk_font_dialog_choose_face")]
    pub fn choose_face<P: FnOnce(Result<pango::FontFace, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        initial_value: Option<&impl IsA<pango::FontFace>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn choose_face_trampoline<
            P: FnOnce(Result<pango::FontFace, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::gtk_font_dialog_choose_face_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = choose_face_trampoline::<P>;
        unsafe {
            ffi::gtk_font_dialog_choose_face(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                initial_value.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn choose_face_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
        initial_value: Option<&(impl IsA<pango::FontFace> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<pango::FontFace, glib::Error>> + 'static>>
    {
        let parent = parent.map(ToOwned::to_owned);
        let initial_value = initial_value.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.choose_face(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                initial_value.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_font_dialog_choose_family")]
    pub fn choose_family<P: FnOnce(Result<pango::FontFamily, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        initial_value: Option<&impl IsA<pango::FontFamily>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn choose_family_trampoline<
            P: FnOnce(Result<pango::FontFamily, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::gtk_font_dialog_choose_family_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = choose_family_trampoline::<P>;
        unsafe {
            ffi::gtk_font_dialog_choose_family(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                initial_value.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn choose_family_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
        initial_value: Option<&(impl IsA<pango::FontFamily> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<pango::FontFamily, glib::Error>> + 'static>>
    {
        let parent = parent.map(ToOwned::to_owned);
        let initial_value = initial_value.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.choose_family(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                initial_value.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_font_dialog_choose_font")]
    pub fn choose_font<P: FnOnce(Result<pango::FontDescription, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        initial_value: Option<&pango::FontDescription>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn choose_font_trampoline<
            P: FnOnce(Result<pango::FontDescription, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::gtk_font_dialog_choose_font_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = choose_font_trampoline::<P>;
        unsafe {
            ffi::gtk_font_dialog_choose_font(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                mut_override(initial_value.to_glib_none().0),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn choose_font_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
        initial_value: Option<&pango::FontDescription>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<pango::FontDescription, glib::Error>> + 'static,
        >,
    > {
        let parent = parent.map(ToOwned::to_owned);
        let initial_value = initial_value.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.choose_font(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                initial_value.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_font_dialog_get_filter")]
    #[doc(alias = "get_filter")]
    pub fn filter(&self) -> Option<Filter> {
        unsafe { from_glib_none(ffi::gtk_font_dialog_get_filter(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_font_dialog_get_font_map")]
    #[doc(alias = "get_font_map")]
    #[doc(alias = "font-map")]
    pub fn font_map(&self) -> Option<pango::FontMap> {
        unsafe { from_glib_none(ffi::gtk_font_dialog_get_font_map(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_font_dialog_get_language")]
    #[doc(alias = "get_language")]
    pub fn language(&self) -> Option<pango::Language> {
        unsafe { from_glib_full(ffi::gtk_font_dialog_get_language(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_font_dialog_get_modal")]
    #[doc(alias = "get_modal")]
    #[doc(alias = "modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_font_dialog_get_modal(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_font_dialog_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_font_dialog_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_font_dialog_set_filter")]
    #[doc(alias = "filter")]
    pub fn set_filter(&self, filter: Option<&impl IsA<Filter>>) {
        unsafe {
            ffi::gtk_font_dialog_set_filter(
                self.to_glib_none().0,
                filter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_font_dialog_set_font_map")]
    #[doc(alias = "font-map")]
    pub fn set_font_map(&self, fontmap: Option<&impl IsA<pango::FontMap>>) {
        unsafe {
            ffi::gtk_font_dialog_set_font_map(
                self.to_glib_none().0,
                fontmap.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_font_dialog_set_language")]
    #[doc(alias = "language")]
    pub fn set_language(&self, language: &pango::Language) {
        unsafe {
            ffi::gtk_font_dialog_set_language(
                self.to_glib_none().0,
                mut_override(language.to_glib_none().0),
            );
        }
    }

    #[doc(alias = "gtk_font_dialog_set_modal")]
    #[doc(alias = "modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_font_dialog_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[doc(alias = "gtk_font_dialog_set_title")]
    #[doc(alias = "title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_font_dialog_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "filter")]
    pub fn connect_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<F: Fn(&FontDialog) + 'static>(
            this: *mut ffi::GtkFontDialog,
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
                b"notify::filter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_filter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "font-map")]
    pub fn connect_font_map_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_font_map_trampoline<F: Fn(&FontDialog) + 'static>(
            this: *mut ffi::GtkFontDialog,
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
                b"notify::font-map\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_font_map_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "language")]
    pub fn connect_language_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_language_trampoline<F: Fn(&FontDialog) + 'static>(
            this: *mut ffi::GtkFontDialog,
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
                b"notify::language\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_language_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&FontDialog) + 'static>(
            this: *mut ffi::GtkFontDialog,
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

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&FontDialog) + 'static>(
            this: *mut ffi::GtkFontDialog,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(feature = "v4_10")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
impl Default for FontDialog {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FontDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FontDialogBuilder {
    builder: glib::object::ObjectBuilder<'static, FontDialog>,
}

impl FontDialogBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn filter(self, filter: &impl IsA<Filter>) -> Self {
        Self {
            builder: self.builder.property("filter", filter.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn font_map(self, font_map: &impl IsA<pango::FontMap>) -> Self {
        Self {
            builder: self.builder.property("font-map", font_map.clone().upcast()),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn language(self, language: &pango::Language) -> Self {
        Self {
            builder: self.builder.property("language", language),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn modal(self, modal: bool) -> Self {
        Self {
            builder: self.builder.property("modal", modal),
        }
    }

    #[cfg(feature = "v4_10")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_10")))]
    pub fn title(self, title: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("title", title.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FontDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FontDialog {
        assert_initialized_main_thread!();
        self.builder.build()
    }
}
