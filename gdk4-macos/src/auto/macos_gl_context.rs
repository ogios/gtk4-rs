// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;

glib::wrapper! {
    #[doc(alias = "GdkMacosGLContext")]
    pub struct MacosGLContext(Object<ffi::GdkMacosGLContext, ffi::GdkMacosGLContextClass>) @extends gdk::GLContext, gdk::DrawContext;

    match fn {
        type_ => || ffi::gdk_macos_gl_context_get_type(),
    }
}

impl MacosGLContext {}
