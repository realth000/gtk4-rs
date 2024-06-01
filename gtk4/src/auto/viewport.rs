// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_12")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
use crate::ScrollInfo;
use crate::{
    ffi, Accessible, AccessibleRole, Adjustment, Align, Buildable, ConstraintTarget, LayoutManager,
    Overflow, Scrollable, ScrollablePolicy, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkViewport")]
    pub struct Viewport(Object<ffi::GtkViewport>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, Scrollable;

    match fn {
        type_ => || ffi::gtk_viewport_get_type(),
    }
}

impl Viewport {
    #[doc(alias = "gtk_viewport_new")]
    pub fn new(
        hadjustment: Option<&impl IsA<Adjustment>>,
        vadjustment: Option<&impl IsA<Adjustment>>,
    ) -> Viewport {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_viewport_new(
                hadjustment.map(|p| p.as_ref()).to_glib_none().0,
                vadjustment.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Viewport`] objects.
    ///
    /// This method returns an instance of [`ViewportBuilder`](crate::builders::ViewportBuilder) which can be used to create [`Viewport`] objects.
    pub fn builder() -> ViewportBuilder {
        ViewportBuilder::new()
    }

    #[doc(alias = "gtk_viewport_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_viewport_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_viewport_get_scroll_to_focus")]
    #[doc(alias = "get_scroll_to_focus")]
    #[doc(alias = "scroll-to-focus")]
    pub fn is_scroll_to_focus(&self) -> bool {
        unsafe { from_glib(ffi::gtk_viewport_get_scroll_to_focus(self.to_glib_none().0)) }
    }

    #[cfg(feature = "v4_12")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_12")))]
    #[doc(alias = "gtk_viewport_scroll_to")]
    pub fn scroll_to(&self, descendant: &impl IsA<Widget>, scroll: Option<ScrollInfo>) {
        unsafe {
            ffi::gtk_viewport_scroll_to(
                self.to_glib_none().0,
                descendant.as_ref().to_glib_none().0,
                scroll.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "gtk_viewport_set_child")]
    #[doc(alias = "child")]
    pub fn set_child(&self, child: Option<&impl IsA<Widget>>) {
        unsafe {
            ffi::gtk_viewport_set_child(
                self.to_glib_none().0,
                child.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_viewport_set_scroll_to_focus")]
    #[doc(alias = "scroll-to-focus")]
    pub fn set_scroll_to_focus(&self, scroll_to_focus: bool) {
        unsafe {
            ffi::gtk_viewport_set_scroll_to_focus(
                self.to_glib_none().0,
                scroll_to_focus.into_glib(),
            );
        }
    }

    #[doc(alias = "child")]
    pub fn connect_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_child_trampoline<F: Fn(&Viewport) + 'static>(
            this: *mut ffi::GtkViewport,
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
                b"notify::child\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_child_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scroll-to-focus")]
    pub fn connect_scroll_to_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scroll_to_focus_trampoline<F: Fn(&Viewport) + 'static>(
            this: *mut ffi::GtkViewport,
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
                b"notify::scroll-to-focus\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_scroll_to_focus_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Viewport {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Viewport`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ViewportBuilder {
    builder: glib::object::ObjectBuilder<'static, Viewport>,
}

impl ViewportBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn scroll_to_focus(self, scroll_to_focus: bool) -> Self {
        Self {
            builder: self.builder.property("scroll-to-focus", scroll_to_focus),
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

    pub fn hadjustment(self, hadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("hadjustment", hadjustment.clone().upcast()),
        }
    }

    pub fn hscroll_policy(self, hscroll_policy: ScrollablePolicy) -> Self {
        Self {
            builder: self.builder.property("hscroll-policy", hscroll_policy),
        }
    }

    pub fn vadjustment(self, vadjustment: &impl IsA<Adjustment>) -> Self {
        Self {
            builder: self
                .builder
                .property("vadjustment", vadjustment.clone().upcast()),
        }
    }

    pub fn vscroll_policy(self, vscroll_policy: ScrollablePolicy) -> Self {
        Self {
            builder: self.builder.property("vscroll-policy", vscroll_policy),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Viewport`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Viewport {
        self.builder.build()
    }
}
