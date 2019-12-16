use crate::utility::vulkanapp::VulkanApp;
use ash::{
    extensions::khr::{Surface, Win32Surface},
    vk::{StructureType, SurfaceKHR, Win32SurfaceCreateInfoKHR},
    Entry, Instance,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::window::Window;

impl VulkanApp {
    pub unsafe fn create_surface(
        instance: &Instance,
        entry: &Entry,
        window: &Window,
    ) -> (Surface, SurfaceKHR) {
        let raw_window_handle = match window.raw_window_handle() {
            RawWindowHandle::Windows(handle) => handle,
            _ => panic!("Surface creation failed: platforms other than Windows not supported yet"),
        };
        let hinstance = raw_window_handle.hinstance;
        let hwnd = raw_window_handle.hwnd;
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
}
