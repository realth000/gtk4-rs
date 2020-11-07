// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Device;
use DeviceTool;
use Display;
use SeatCapabilities;

glib_wrapper! {
    pub struct Seat(Object<gdk_sys::GdkSeat>);

    match fn {
        get_type => || gdk_sys::gdk_seat_get_type(),
    }
}

impl Seat {
    pub fn get_capabilities(&self) -> SeatCapabilities {
        unsafe { from_glib(gdk_sys::gdk_seat_get_capabilities(self.to_glib_none().0)) }
    }

    pub fn get_devices(&self, capabilities: SeatCapabilities) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_seat_get_devices(
                self.to_glib_none().0,
                capabilities.to_glib(),
            ))
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(gdk_sys::gdk_seat_get_display(self.to_glib_none().0)) }
    }

    pub fn get_keyboard(&self) -> Option<Device> {
        unsafe { from_glib_none(gdk_sys::gdk_seat_get_keyboard(self.to_glib_none().0)) }
    }

    pub fn get_pointer(&self) -> Option<Device> {
        unsafe { from_glib_none(gdk_sys::gdk_seat_get_pointer(self.to_glib_none().0)) }
    }

    pub fn get_tools(&self) -> Vec<DeviceTool> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(gdk_sys::gdk_seat_get_tools(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn connect_device_added<F: Fn(&Seat, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_added_trampoline<F: Fn(&Seat, &Device) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            device: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_device_removed<F: Fn(&Seat, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn device_removed_trampoline<F: Fn(&Seat, &Device) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            device: *mut gdk_sys::GdkDevice,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"device-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    device_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_tool_added<F: Fn(&Seat, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn tool_added_trampoline<F: Fn(&Seat, &DeviceTool) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            tool: *mut gdk_sys::GdkDeviceTool,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_tool_removed<F: Fn(&Seat, &DeviceTool) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn tool_removed_trampoline<F: Fn(&Seat, &DeviceTool) + 'static>(
            this: *mut gdk_sys::GdkSeat,
            tool: *mut gdk_sys::GdkDeviceTool,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(tool))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"tool-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    tool_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Seat")
    }
}
