// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::{
    ffi, Accessible, AccessibleRole, Align, Buildable, ConstraintTarget, IconSize, ImageType,
    LayoutManager, Overflow, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkImage")]
    pub struct Image(Object<ffi::GtkImage>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget;

    match fn {
        type_ => || ffi::gtk_image_get_type(),
    }
}

impl Image {
    #[doc(alias = "gtk_image_new")]
    pub fn new() -> Image {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_image_new()).unsafe_cast() }
    }

    #[doc(alias = "gtk_image_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file(filename: impl AsRef<std::path::Path>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_file(
                filename.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_gicon")]
    #[doc(alias = "new_from_gicon")]
    pub fn from_gicon(icon: &impl IsA<gio::Icon>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_gicon(
                icon.as_ref().to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_icon_name")]
    #[doc(alias = "new_from_icon_name")]
    pub fn from_icon_name(icon_name: &str) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_icon_name(
                icon_name.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_paintable")]
    #[doc(alias = "new_from_paintable")]
    pub fn from_paintable(paintable: Option<&impl IsA<gdk::Paintable>>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_paintable(
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[cfg_attr(feature = "v4_12", deprecated = "Since 4.12")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_image_new_from_pixbuf")]
    #[doc(alias = "new_from_pixbuf")]
    pub fn from_pixbuf(pixbuf: Option<&gdk_pixbuf::Pixbuf>) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_pixbuf(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_image_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource(resource_path: &str) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_resource(
                resource_path.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Image`] objects.
    ///
    /// This method returns an instance of [`ImageBuilder`](crate::builders::ImageBuilder) which can be used to create [`Image`] objects.
    pub fn builder() -> ImageBuilder {
        ImageBuilder::new()
    }

    #[doc(alias = "gtk_image_clear")]
    pub fn clear(&self) {
        unsafe {
            ffi::gtk_image_clear(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_get_gicon")]
    #[doc(alias = "get_gicon")]
    pub fn gicon(&self) -> Option<gio::Icon> {
        unsafe { from_glib_none(ffi::gtk_image_get_gicon(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_image_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_icon_size")]
    #[doc(alias = "get_icon_size")]
    #[doc(alias = "icon-size")]
    pub fn icon_size(&self) -> IconSize {
        unsafe { from_glib(ffi::gtk_image_get_icon_size(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_paintable")]
    #[doc(alias = "get_paintable")]
    pub fn paintable(&self) -> Option<gdk::Paintable> {
        unsafe { from_glib_none(ffi::gtk_image_get_paintable(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_get_pixel_size")]
    #[doc(alias = "get_pixel_size")]
    #[doc(alias = "pixel-size")]
    pub fn pixel_size(&self) -> i32 {
        unsafe { ffi::gtk_image_get_pixel_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_image_get_storage_type")]
    #[doc(alias = "get_storage_type")]
    #[doc(alias = "storage-type")]
    pub fn storage_type(&self) -> ImageType {
        unsafe { from_glib(ffi::gtk_image_get_storage_type(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_image_set_from_file")]
    #[doc(alias = "file")]
    pub fn set_from_file(&self, filename: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::gtk_image_set_from_file(
                self.to_glib_none().0,
                filename.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_image_set_from_gicon")]
    #[doc(alias = "gicon")]
    pub fn set_from_gicon(&self, icon: &impl IsA<gio::Icon>) {
        unsafe {
            ffi::gtk_image_set_from_gicon(self.to_glib_none().0, icon.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_from_icon_name")]
    #[doc(alias = "icon-name")]
    pub fn set_from_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_from_paintable")]
    #[doc(alias = "paintable")]
    pub fn set_from_paintable(&self, paintable: Option<&impl IsA<gdk::Paintable>>) {
        unsafe {
            ffi::gtk_image_set_from_paintable(
                self.to_glib_none().0,
                paintable.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v4_12", deprecated = "Since 4.12")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_image_set_from_pixbuf")]
    #[doc(alias = "paintable")]
    pub fn set_from_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_image_set_from_pixbuf(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_from_resource")]
    #[doc(alias = "resource")]
    pub fn set_from_resource(&self, resource_path: Option<&str>) {
        unsafe {
            ffi::gtk_image_set_from_resource(self.to_glib_none().0, resource_path.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_image_set_icon_size")]
    #[doc(alias = "icon-size")]
    pub fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_image_set_icon_size(self.to_glib_none().0, icon_size.into_glib());
        }
    }

    #[doc(alias = "gtk_image_set_pixel_size")]
    #[doc(alias = "pixel-size")]
    pub fn set_pixel_size(&self, pixel_size: i32) {
        unsafe {
            ffi::gtk_image_set_pixel_size(self.to_glib_none().0, pixel_size);
        }
    }

    pub fn file(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "file")
    }

    pub fn resource(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "resource")
    }

    #[doc(alias = "use-fallback")]
    pub fn uses_fallback(&self) -> bool {
        ObjectExt::property(self, "use-fallback")
    }

    #[doc(alias = "use-fallback")]
    pub fn set_use_fallback(&self, use_fallback: bool) {
        ObjectExt::set_property(self, "use-fallback", use_fallback)
    }

    #[doc(alias = "file")]
    pub fn connect_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_file_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::file\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_file_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "gicon")]
    pub fn connect_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_gicon_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::gicon\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_gicon_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-size")]
    pub fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::icon-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "paintable")]
    pub fn connect_paintable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paintable_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::paintable\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_paintable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "pixel-size")]
    pub fn connect_pixel_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pixel_size_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::pixel-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_pixel_size_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resource")]
    pub fn connect_resource_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resource_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::resource\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_resource_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "storage-type")]
    pub fn connect_storage_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_storage_type_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::storage-type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_storage_type_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-fallback")]
    pub fn connect_use_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_fallback_trampoline<F: Fn(&Image) + 'static>(
            this: *mut ffi::GtkImage,
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
                b"notify::use-fallback\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_fallback_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Image`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ImageBuilder {
    builder: glib::object::ObjectBuilder<'static, Image>,
}

impl ImageBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn file(self, file: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("file", file.into()),
        }
    }

    pub fn gicon(self, gicon: &impl IsA<gio::Icon>) -> Self {
        Self {
            builder: self.builder.property("gicon", gicon.clone().upcast()),
        }
    }

    pub fn icon_name(self, icon_name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("icon-name", icon_name.into()),
        }
    }

    pub fn icon_size(self, icon_size: IconSize) -> Self {
        Self {
            builder: self.builder.property("icon-size", icon_size),
        }
    }

    pub fn paintable(self, paintable: &impl IsA<gdk::Paintable>) -> Self {
        Self {
            builder: self
                .builder
                .property("paintable", paintable.clone().upcast()),
        }
    }

    pub fn pixel_size(self, pixel_size: i32) -> Self {
        Self {
            builder: self.builder.property("pixel-size", pixel_size),
        }
    }

    pub fn resource(self, resource: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("resource", resource.into()),
        }
    }

    pub fn use_fallback(self, use_fallback: bool) -> Self {
        Self {
            builder: self.builder.property("use-fallback", use_fallback),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`Image`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Image {
        self.builder.build()
    }
}
