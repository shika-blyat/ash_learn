use crate::utility::vulkanapp::VulkanApp;
use ash::{
    extensions::khr::Surface,
    vk::{StructureType, SurfaceKHR},
    Entry, Instance,
};

#[cfg(target_os = "windows")]
use ash::{extensions::khr::Win32Surface, vk::Win32SurfaceCreateInfoKHR};

#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::{extensions::khr::XlibSurface, vk::XlibSurfaceCreateInfoKHR};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::os::raw::c_void;
use winit::window::Window;

impl VulkanApp {
    #[cfg(target_os = "windows")]
    pub fn create_surface(
        instance: &Instance,
        entry: &Entry,
        window: &Window,
    ) -> (Surface, SurfaceKHR) {
        let window_handle = if let RawWindowHandle::Windows(handle) = window.raw_window_handle() {
            handle
        } else {
            panic!("Cannot get window handle");
        };
        //
        let hinstance = window_handle.hinstance;
        let hwnd = window_handle.hwnd;
        let win32_create_info = Win32SurfaceCreateInfoKHR {
            s_type: StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            hinstance,
            hwnd,
            ..Default::default()
        };
        let win32_surface_loader = Win32Surface::new(entry, instance);
        (
            Surface::new(entry, instance),
            win32_surface_loader
                .create_win32_surface(&win32_create_info, None)
                .expect("Failed to created surface"),
        )
    }
    #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
    pub fn create_surface(
        instance: &Instance,
        entry: &Entry,
        window: &Window,
    ) -> (Surface, SurfaceKHR) {
        let window_handle = if let RawWindowHandle::Xlib(handle) = window.raw_window_handle() {
            handle
        } else {
            panic!("Cannot get window handle");
        };
        //
        let window = window_handle.window;
        let display = window_handle.display as *mut *const c_void;
        let xlib_create_info = XlibSurfaceCreateInfoKHR {
            s_type: StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            window,
            dpy: display,
            ..Default::default()
        };
        let win32_surface_loader = XlibSurface::new(entry, instance);
        (Surface::new(entry, instance), unsafe {
            win32_surface_loader
                .create_xlib_surface(&xlib_create_info, None)
                .expect("Failed to created surface")
        })
    }
}
