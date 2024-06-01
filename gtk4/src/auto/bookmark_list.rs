// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkBookmarkList")]
    pub struct BookmarkList(Object<ffi::GtkBookmarkList, ffi::GtkBookmarkListClass>) @implements gio::ListModel;

    match fn {
        type_ => || ffi::gtk_bookmark_list_get_type(),
    }
}

impl BookmarkList {
    #[doc(alias = "gtk_bookmark_list_new")]
    pub fn new(
        filename: Option<impl AsRef<std::path::Path>>,
        attributes: Option<&str>,
    ) -> BookmarkList {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_bookmark_list_new(
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
                attributes.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_bookmark_list_get_attributes")]
    #[doc(alias = "get_attributes")]
    pub fn attributes(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_bookmark_list_get_attributes(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bookmark_list_get_filename")]
    #[doc(alias = "get_filename")]
    pub fn filename(&self) -> std::path::PathBuf {
        unsafe { from_glib_none(ffi::gtk_bookmark_list_get_filename(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bookmark_list_is_loading")]
    #[doc(alias = "loading")]
    pub fn is_loading(&self) -> bool {
        unsafe { from_glib(ffi::gtk_bookmark_list_is_loading(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_bookmark_list_set_attributes")]
    #[doc(alias = "attributes")]
    pub fn set_attributes(&self, attributes: Option<&str>) {
        unsafe {
            ffi::gtk_bookmark_list_set_attributes(
                self.to_glib_none().0,
                attributes.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "attributes")]
    pub fn connect_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_attributes_trampoline<F: Fn(&BookmarkList) + 'static>(
            this: *mut ffi::GtkBookmarkList,
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
                b"notify::attributes\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_attributes_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "io-priority")]
    pub fn connect_io_priority_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_io_priority_trampoline<F: Fn(&BookmarkList) + 'static>(
            this: *mut ffi::GtkBookmarkList,
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
                b"notify::io-priority\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_io_priority_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "loading")]
    pub fn connect_loading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_loading_trampoline<F: Fn(&BookmarkList) + 'static>(
            this: *mut ffi::GtkBookmarkList,
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
                b"notify::loading\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_loading_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
