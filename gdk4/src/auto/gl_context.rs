// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
use crate::GLAPI;
use crate::{ffi, Display, DrawContext, Surface};
#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v4_6")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkGLContext")]
    pub struct GLContext(Object<ffi::GdkGLContext>) @extends DrawContext;

    match fn {
        type_ => || ffi::gdk_gl_context_get_type(),
    }
}

impl GLContext {
    pub const NONE: Option<&'static GLContext> = None;

    #[doc(alias = "gdk_gl_context_clear_current")]
    pub fn clear_current() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_gl_context_clear_current();
        }
    }

    #[doc(alias = "gdk_gl_context_get_current")]
    #[doc(alias = "get_current")]
    pub fn current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_gl_context_get_current()) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GLContext>> Sealed for T {}
}

pub trait GLContextExt: IsA<GLContext> + sealed::Sealed + 'static {
    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gdk_gl_context_get_allowed_apis")]
    #[doc(alias = "get_allowed_apis")]
    #[doc(alias = "allowed-apis")]
    fn allowed_apis(&self) -> GLAPI {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_allowed_apis(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gdk_gl_context_get_api")]
    #[doc(alias = "get_api")]
    fn api(&self) -> GLAPI {
        unsafe { from_glib(ffi::gdk_gl_context_get_api(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gdk_gl_context_get_debug_enabled")]
    #[doc(alias = "get_debug_enabled")]
    fn is_debug_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_debug_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_forward_compatible")]
    #[doc(alias = "get_forward_compatible")]
    fn is_forward_compatible(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_forward_compatible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_required_version")]
    #[doc(alias = "get_required_version")]
    fn required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = std::mem::MaybeUninit::uninit();
            let mut minor = std::mem::MaybeUninit::uninit();
            ffi::gdk_gl_context_get_required_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            (major.assume_init(), minor.assume_init())
        }
    }

    #[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_gl_context_get_shared_context")]
    #[doc(alias = "get_shared_context")]
    #[doc(alias = "shared-context")]
    #[must_use]
    fn shared_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_shared_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_surface")]
    #[doc(alias = "get_surface")]
    fn surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_use_es")]
    #[doc(alias = "get_use_es")]
    fn uses_es(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_use_es(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_get_version")]
    #[doc(alias = "get_version")]
    fn version(&self) -> (i32, i32) {
        unsafe {
            let mut major = std::mem::MaybeUninit::uninit();
            let mut minor = std::mem::MaybeUninit::uninit();
            ffi::gdk_gl_context_get_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            (major.assume_init(), minor.assume_init())
        }
    }

    #[doc(alias = "gdk_gl_context_is_legacy")]
    fn is_legacy(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_is_legacy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v4_4")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gdk_gl_context_is_shared")]
    fn is_shared(&self, other: &impl IsA<GLContext>) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_is_shared(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_gl_context_make_current")]
    fn make_current(&self) {
        unsafe {
            ffi::gdk_gl_context_make_current(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gdk_gl_context_realize")]
    fn realize(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gdk_gl_context_realize(self.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gdk_gl_context_set_allowed_apis")]
    #[doc(alias = "allowed-apis")]
    fn set_allowed_apis(&self, apis: GLAPI) {
        unsafe {
            ffi::gdk_gl_context_set_allowed_apis(self.as_ref().to_glib_none().0, apis.into_glib());
        }
    }

    #[doc(alias = "gdk_gl_context_set_debug_enabled")]
    fn set_debug_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gdk_gl_context_set_debug_enabled(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    #[doc(alias = "gdk_gl_context_set_forward_compatible")]
    fn set_forward_compatible(&self, compatible: bool) {
        unsafe {
            ffi::gdk_gl_context_set_forward_compatible(
                self.as_ref().to_glib_none().0,
                compatible.into_glib(),
            );
        }
    }

    #[doc(alias = "gdk_gl_context_set_required_version")]
    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gdk_gl_context_set_required_version(self.as_ref().to_glib_none().0, major, minor);
        }
    }

    #[doc(alias = "gdk_gl_context_set_use_es")]
    fn set_use_es(&self, use_es: i32) {
        unsafe {
            ffi::gdk_gl_context_set_use_es(self.as_ref().to_glib_none().0, use_es);
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "allowed-apis")]
    fn connect_allowed_apis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allowed_apis_trampoline<
            P: IsA<GLContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkGLContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::allowed-apis\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_allowed_apis_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v4_6")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_6")))]
    #[doc(alias = "api")]
    fn connect_api_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_api_trampoline<P: IsA<GLContext>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkGLContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::api\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_api_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<GLContext>> GLContextExt for O {}
