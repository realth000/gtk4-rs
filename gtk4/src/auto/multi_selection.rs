// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use SelectionModel;

glib_wrapper! {
    pub struct MultiSelection(Object<gtk_sys::GtkMultiSelection, gtk_sys::GtkMultiSelectionClass>) @implements gio::ListModel, SelectionModel;

    match fn {
        get_type => || gtk_sys::gtk_multi_selection_get_type(),
    }
}

impl MultiSelection {
    pub fn new<P: IsA<gio::ListModel>>(model: Option<&P>) -> MultiSelection {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_multi_selection_new(
                model.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct MultiSelectionBuilder {
    model: Option<gio::ListModel>,
}

impl MultiSelectionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MultiSelection {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        let ret = glib::Object::new(MultiSelection::static_type(), &properties)
            .expect("object new")
            .downcast::<MultiSelection>()
            .expect("downcast");
        ret
    }

    pub fn model<P: IsA<gio::ListModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }
}

pub const NONE_MULTI_SELECTION: Option<&MultiSelection> = None;

pub trait MultiSelectionExt: 'static {
    fn get_model(&self) -> Option<gio::ListModel>;

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>);

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MultiSelection>> MultiSelectionExt for O {
    fn get_model(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_multi_selection_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_multi_selection_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkMultiSelection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<MultiSelection>,
        {
            let f: &F = &*(f as *const F);
            f(&MultiSelection::from_glib_borrow(this).unsafe_cast_ref())
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
}

impl fmt::Display for MultiSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MultiSelection")
    }
}
