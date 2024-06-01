// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{ffi, Buildable, CellArea, CellLayout, CellRenderer, Orientable, Orientation};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkCellAreaBox")]
    pub struct CellAreaBox(Object<ffi::GtkCellAreaBox>) @extends CellArea, @implements Buildable, CellLayout, Orientable;

    match fn {
        type_ => || ffi::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_box_new")]
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe { CellArea::from_glib_none(ffi::gtk_cell_area_box_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`CellAreaBox`] objects.
    ///
    /// This method returns an instance of [`CellAreaBoxBuilder`](crate::builders::CellAreaBoxBuilder) which can be used to create [`CellAreaBox`] objects.
    pub fn builder() -> CellAreaBoxBuilder {
        CellAreaBoxBuilder::new()
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_box_get_spacing")]
    #[doc(alias = "get_spacing")]
    pub fn spacing(&self) -> i32 {
        unsafe { ffi::gtk_cell_area_box_get_spacing(self.to_glib_none().0) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_box_pack_end")]
    pub fn pack_end(
        &self,
        renderer: &impl IsA<CellRenderer>,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_box_pack_end(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_box_pack_start")]
    pub fn pack_start(
        &self,
        renderer: &impl IsA<CellRenderer>,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            ffi::gtk_cell_area_box_pack_start(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.into_glib(),
                align.into_glib(),
                fixed.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_cell_area_box_set_spacing")]
    #[doc(alias = "spacing")]
    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_cell_area_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[doc(alias = "spacing")]
    pub fn connect_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&CellAreaBox) + 'static>(
            this: *mut ffi::GtkCellAreaBox,
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
                b"notify::spacing\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellAreaBox {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`CellAreaBox`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct CellAreaBoxBuilder {
    builder: glib::object::ObjectBuilder<'static, CellAreaBox>,
}

impl CellAreaBoxBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
        }
    }

    pub fn focus_cell(self, focus_cell: &impl IsA<CellRenderer>) -> Self {
        Self {
            builder: self
                .builder
                .property("focus-cell", focus_cell.clone().upcast()),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`CellAreaBox`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> CellAreaBox {
        self.builder.build()
    }
}
