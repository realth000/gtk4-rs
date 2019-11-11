// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Align;
use Bin;
use Buildable;
use Container;
use Frame;
use LayoutManager;
use Overflow;
use ShadowType;
use Widget;

glib_wrapper! {
    pub struct AspectFrame(Object<gtk_sys::GtkAspectFrame, gtk_sys::GtkAspectFrameClass, AspectFrameClass>) @extends Frame, Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_aspect_frame_get_type(),
    }
}

impl AspectFrame {
    pub fn new(
        label: Option<&str>,
        xalign: f32,
        yalign: f32,
        ratio: f32,
        obey_child: bool,
    ) -> AspectFrame {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_aspect_frame_new(
                label.to_glib_none().0,
                xalign,
                yalign,
                ratio,
                obey_child.to_glib(),
            ))
            .unsafe_cast()
        }
    }
}

pub struct AspectFrameBuilder {
    obey_child: Option<bool>,
    ratio: Option<f32>,
    xalign: Option<f32>,
    yalign: Option<f32>,
    label: Option<String>,
    label_widget: Option<Widget>,
    label_xalign: Option<f32>,
    shadow_type: Option<ShadowType>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    expand: Option<bool>,
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
}

impl AspectFrameBuilder {
    pub fn new() -> Self {
        Self {
            obey_child: None,
            ratio: None,
            xalign: None,
            yalign: None,
            label: None,
            label_widget: None,
            label_xalign: None,
            shadow_type: None,
            can_focus: None,
            can_target: None,
            css_name: None,
            cursor: None,
            expand: None,
            focus_on_click: None,
            halign: None,
            has_focus: None,
            has_tooltip: None,
            height_request: None,
            hexpand: None,
            hexpand_set: None,
            is_focus: None,
            layout_manager: None,
            margin: None,
            margin_bottom: None,
            margin_end: None,
            margin_start: None,
            margin_top: None,
            name: None,
            opacity: None,
            overflow: None,
            receives_default: None,
            sensitive: None,
            tooltip_markup: None,
            tooltip_text: None,
            valign: None,
            vexpand: None,
            vexpand_set: None,
            visible: None,
            width_request: None,
        }
    }

    pub fn build(self) -> AspectFrame {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref obey_child) = self.obey_child {
            properties.push(("obey-child", obey_child));
        }
        if let Some(ref ratio) = self.ratio {
            properties.push(("ratio", ratio));
        }
        if let Some(ref xalign) = self.xalign {
            properties.push(("xalign", xalign));
        }
        if let Some(ref yalign) = self.yalign {
            properties.push(("yalign", yalign));
        }
        if let Some(ref label) = self.label {
            properties.push(("label", label));
        }
        if let Some(ref label_widget) = self.label_widget {
            properties.push(("label-widget", label_widget));
        }
        if let Some(ref label_xalign) = self.label_xalign {
            properties.push(("label-xalign", label_xalign));
        }
        if let Some(ref shadow_type) = self.shadow_type {
            properties.push(("shadow-type", shadow_type));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        glib::Object::new(AspectFrame::static_type(), &properties)
            .expect("object new")
            .downcast()
            .expect("downcast")
    }

    pub fn obey_child(mut self, obey_child: bool) -> Self {
        self.obey_child = Some(obey_child);
        self
    }

    pub fn ratio(mut self, ratio: f32) -> Self {
        self.ratio = Some(ratio);
        self
    }

    pub fn xalign(mut self, xalign: f32) -> Self {
        self.xalign = Some(xalign);
        self
    }

    pub fn yalign(mut self, yalign: f32) -> Self {
        self.yalign = Some(yalign);
        self
    }

    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    pub fn label_widget<P: IsA<Widget>>(mut self, label_widget: &P) -> Self {
        self.label_widget = Some(label_widget.clone().upcast());
        self
    }

    pub fn label_xalign(mut self, label_xalign: f32) -> Self {
        self.label_xalign = Some(label_xalign);
        self
    }

    pub fn shadow_type(mut self, shadow_type: ShadowType) -> Self {
        self.shadow_type = Some(shadow_type);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }
}

pub const NONE_ASPECT_FRAME: Option<&AspectFrame> = None;

pub trait AspectFrameExt: 'static {
    fn set(&self, xalign: f32, yalign: f32, ratio: f32, obey_child: bool);

    fn get_property_obey_child(&self) -> bool;

    fn set_property_obey_child(&self, obey_child: bool);

    fn get_property_ratio(&self) -> f32;

    fn set_property_ratio(&self, ratio: f32);

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn connect_property_obey_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AspectFrame>> AspectFrameExt for O {
    fn set(&self, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) {
        unsafe {
            gtk_sys::gtk_aspect_frame_set(
                self.as_ref().to_glib_none().0,
                xalign,
                yalign,
                ratio,
                obey_child.to_glib(),
            );
        }
    }

    fn get_property_obey_child(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"obey-child\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `obey-child` getter")
                .unwrap()
        }
    }

    fn set_property_obey_child(&self, obey_child: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"obey-child\0".as_ptr() as *const _,
                Value::from(&obey_child).to_glib_none().0,
            );
        }
    }

    fn get_property_ratio(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ratio\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `ratio` getter")
                .unwrap()
        }
    }

    fn set_property_ratio(&self, ratio: f32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ratio\0".as_ptr() as *const _,
                Value::from(&ratio).to_glib_none().0,
            );
        }
    }

    fn get_property_xalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"xalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `xalign` getter")
                .unwrap()
        }
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"xalign\0".as_ptr() as *const _,
                Value::from(&xalign).to_glib_none().0,
            );
        }
    }

    fn get_property_yalign(&self) -> f32 {
        unsafe {
            let mut value = Value::from_type(<f32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"yalign\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `yalign` getter")
                .unwrap()
        }
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"yalign\0".as_ptr() as *const _,
                Value::from(&yalign).to_glib_none().0,
            );
        }
    }

    fn connect_property_obey_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_obey_child_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAspectFrame,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AspectFrame>,
        {
            let f: &F = &*(f as *const F);
            f(&AspectFrame::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::obey-child\0".as_ptr() as *const _,
                Some(transmute(notify_obey_child_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ratio_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAspectFrame,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AspectFrame>,
        {
            let f: &F = &*(f as *const F);
            f(&AspectFrame::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ratio\0".as_ptr() as *const _,
                Some(transmute(notify_ratio_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xalign_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAspectFrame,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AspectFrame>,
        {
            let f: &F = &*(f as *const F);
            f(&AspectFrame::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xalign\0".as_ptr() as *const _,
                Some(transmute(notify_xalign_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_yalign_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkAspectFrame,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<AspectFrame>,
        {
            let f: &F = &*(f as *const F);
            f(&AspectFrame::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::yalign\0".as_ptr() as *const _,
                Some(transmute(notify_yalign_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AspectFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AspectFrame")
    }
}
