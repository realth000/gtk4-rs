// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    ffi, Accessible, AccessibleRole, Actionable, Align, Buildable, ConstraintTarget, LayoutManager,
    Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkListBoxRow")]
    pub struct ListBoxRow(Object<ffi::GtkListBoxRow, ffi::GtkListBoxRowClass>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Actionable;

    match fn {
        type_ => || ffi::gtk_list_box_row_get_type(),
    }
}

impl ListBoxRow {
    pub const NONE: Option<&'static ListBoxRow> = None;

    #[doc(alias = "gtk_list_box_row_new")]
    pub fn new() -> ListBoxRow {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_list_box_row_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ListBoxRow`] objects.
    ///
    /// This method returns an instance of [`ListBoxRowBuilder`](crate::builders::ListBoxRowBuilder) which can be used to create [`ListBoxRow`] objects.
    pub fn builder() -> ListBoxRowBuilder {
        ListBoxRowBuilder::new()
    }
}

impl Default for ListBoxRow {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ListBoxRow`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ListBoxRowBuilder {
    builder: glib::object::ObjectBuilder<'static, ListBoxRow>,
}

impl ListBoxRowBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn activatable(self, activatable: bool) -> Self {
        Self {
            builder: self.builder.property("activatable", activatable),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn selectable(self, selectable: bool) -> Self {
        Self {
            builder: self.builder.property("selectable", selectable),
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

    pub fn action_name(self, action_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("action-name", action_name.into()),
        }
    }

    pub fn action_target(self, action_target: &glib::Variant) -> Self {
        Self {
            builder: self
                .builder
                .property("action-target", action_target.clone()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ListBoxRow`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ListBoxRow {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ListBoxRow>> Sealed for T {}
}

pub trait ListBoxRowExt: IsA<ListBoxRow> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_list_box_row_changed")]
    fn changed(&self) {
        unsafe {
            ffi::gtk_list_box_row_changed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_list_box_row_get_activatable")]
    #[doc(alias = "get_activatable")]
    #[doc(alias = "activatable")]
    fn is_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_activatable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_row_get_child")]
    #[doc(alias = "get_child")]
    fn child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_row_get_child(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_row_get_header")]
    #[doc(alias = "get_header")]
    fn header(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_row_get_header(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_row_get_index")]
    #[doc(alias = "get_index")]
    fn index(&self) -> i32 {
        unsafe { ffi::gtk_list_box_row_get_index(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_list_box_row_get_selectable")]
    #[doc(alias = "get_selectable")]
    #[doc(alias = "selectable")]
    fn is_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_selectable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_row_is_selected")]
    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_is_selected(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_list_box_row_set_activatable")]
    #[doc(alias = "activatable")]
    fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_activatable(
                self.as_ref().to_glib_none().0,
                activatable.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_list_box_row_set_child")]
    #[doc(alias = "child")]
    fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_list_box_row_set_child(
                self.as_ref().to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_box_row_set_header")]
    fn set_header(&self, header: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_list_box_row_set_header(
                self.as_ref().to_glib_none().0,
                header.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_list_box_row_set_selectable")]
    #[doc(alias = "selectable")]
    fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_selectable(
                self.as_ref().to_glib_none().0,
                selectable.into_glib(),
            );
        }
    }

    #[doc(alias = "activate")]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<P: IsA<ListBoxRow>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBoxRow,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_activate(&self) {
        self.emit_by_name::<()>("activate", &[]);
    }

    #[doc(alias = "activatable")]
    fn connect_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_activatable_trampoline<
            P: IsA<ListBoxRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkListBoxRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::activatable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_activatable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "child")]
    fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<P: IsA<ListBoxRow>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkListBoxRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "selectable")]
    fn connect_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_selectable_trampoline<
            P: IsA<ListBoxRow>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkListBoxRow,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ListBoxRow::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::selectable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_selectable_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ListBoxRow>> ListBoxRowExt for O {}
