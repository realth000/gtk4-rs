// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use Buildable;
use Constraint;
use ConstraintGuide;
use LayoutManager;

glib_wrapper! {
    pub struct ConstraintLayout(Object<gtk_sys::GtkConstraintLayout, gtk_sys::GtkConstraintLayoutClass>) @extends LayoutManager, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_constraint_layout_get_type(),
    }
}

impl ConstraintLayout {
    pub fn new() -> ConstraintLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(gtk_sys::gtk_constraint_layout_new()).unsafe_cast() }
    }
}

impl Default for ConstraintLayout {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CONSTRAINT_LAYOUT: Option<&ConstraintLayout> = None;

pub trait ConstraintLayoutExt: 'static {
    fn add_constraint<P: IsA<Constraint>>(&self, constraint: &P);

    //fn add_constraints_from_description(&self, lines: &[&str], hspacing: i32, vspacing: i32, error: &mut glib::Error, first_view: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Vec<Constraint>;

    //fn add_constraints_from_descriptionv(&self, lines: &[&str], hspacing: i32, vspacing: i32, views: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 6 }) -> Result<Vec<Constraint>, glib::Error>;

    fn add_guide<P: IsA<ConstraintGuide>>(&self, guide: &P);

    fn observe_constraints(&self) -> Option<gio::ListModel>;

    fn observe_guides(&self) -> Option<gio::ListModel>;

    fn remove_all_constraints(&self);

    fn remove_constraint<P: IsA<Constraint>>(&self, constraint: &P);

    fn remove_guide<P: IsA<ConstraintGuide>>(&self, guide: &P);
}

impl<O: IsA<ConstraintLayout>> ConstraintLayoutExt for O {
    fn add_constraint<P: IsA<Constraint>>(&self, constraint: &P) {
        unsafe {
            gtk_sys::gtk_constraint_layout_add_constraint(
                self.as_ref().to_glib_none().0,
                constraint.as_ref().to_glib_full(),
            );
        }
    }

    //fn add_constraints_from_description(&self, lines: &[&str], hspacing: i32, vspacing: i32, error: &mut glib::Error, first_view: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Vec<Constraint> {
    //    unsafe { TODO: call gtk_sys:gtk_constraint_layout_add_constraints_from_description() }
    //}

    //fn add_constraints_from_descriptionv(&self, lines: &[&str], hspacing: i32, vspacing: i32, views: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 6 }) -> Result<Vec<Constraint>, glib::Error> {
    //    unsafe { TODO: call gtk_sys:gtk_constraint_layout_add_constraints_from_descriptionv() }
    //}

    fn add_guide<P: IsA<ConstraintGuide>>(&self, guide: &P) {
        unsafe {
            gtk_sys::gtk_constraint_layout_add_guide(
                self.as_ref().to_glib_none().0,
                guide.as_ref().to_glib_full(),
            );
        }
    }

    fn observe_constraints(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_full(gtk_sys::gtk_constraint_layout_observe_constraints(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn observe_guides(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_full(gtk_sys::gtk_constraint_layout_observe_guides(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_all_constraints(&self) {
        unsafe {
            gtk_sys::gtk_constraint_layout_remove_all_constraints(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_constraint<P: IsA<Constraint>>(&self, constraint: &P) {
        unsafe {
            gtk_sys::gtk_constraint_layout_remove_constraint(
                self.as_ref().to_glib_none().0,
                constraint.as_ref().to_glib_none().0,
            );
        }
    }

    fn remove_guide<P: IsA<ConstraintGuide>>(&self, guide: &P) {
        unsafe {
            gtk_sys::gtk_constraint_layout_remove_guide(
                self.as_ref().to_glib_none().0,
                guide.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for ConstraintLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConstraintLayout")
    }
}
