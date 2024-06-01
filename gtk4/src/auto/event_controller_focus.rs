// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, EventController, PropagationLimit, PropagationPhase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkEventControllerFocus")]
    pub struct EventControllerFocus(Object<ffi::GtkEventControllerFocus, ffi::GtkEventControllerFocusClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_focus_get_type(),
    }
}

impl EventControllerFocus {
    #[doc(alias = "gtk_event_controller_focus_new")]
    pub fn new() -> EventControllerFocus {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_focus_new()).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`EventControllerFocus`] objects.
    ///
    /// This method returns an instance of [`EventControllerFocusBuilder`](crate::builders::EventControllerFocusBuilder) which can be used to create [`EventControllerFocus`] objects.
    pub fn builder() -> EventControllerFocusBuilder {
        EventControllerFocusBuilder::new()
    }

    #[doc(alias = "gtk_event_controller_focus_contains_focus")]
    #[doc(alias = "contains-focus")]
    pub fn contains_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_focus_contains_focus(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_focus_is_focus")]
    #[doc(alias = "is-focus")]
    pub fn is_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_focus_is_focus(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "enter")]
    pub fn connect_enter<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "leave")]
    pub fn connect_leave<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "contains-focus")]
    pub fn connect_contains_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_contains_focus_trampoline<
            F: Fn(&EventControllerFocus) + 'static,
        >(
            this: *mut ffi::GtkEventControllerFocus,
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
                b"notify::contains-focus\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_contains_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-focus")]
    pub fn connect_is_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_focus_trampoline<F: Fn(&EventControllerFocus) + 'static>(
            this: *mut ffi::GtkEventControllerFocus,
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
                b"notify::is-focus\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_is_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerFocus {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`EventControllerFocus`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EventControllerFocusBuilder {
    builder: glib::object::ObjectBuilder<'static, EventControllerFocus>,
}

impl EventControllerFocusBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EventControllerFocus`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EventControllerFocus {
        self.builder.build()
    }
}
