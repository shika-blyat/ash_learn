use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;
use std::ffi::CString;
use std::ptr;
use winit::{
    event::{KeyboardInput, ElementState, VirtualKeyCode, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

#[cfg(target_os = "windows")]
use ash::extensions::khr::Win32Surface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::khr::XlibSurface;
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;

use ash::vk::{self};
use ash::{self, vk_make_version};



struct VulkanApp {
    _entry: ash::Entry,
    instance: ash::Instance,
}

impl VulkanApp {

    pub fn new() -> VulkanApp {

        // init vulkan stuff
        let entry = ash::Entry::new().unwrap();
        let instance = VulkanApp::create_instance(&entry);

        // cleanup(); the 'drop' function will take care of it.
        VulkanApp {
            _entry: entry,
            instance,
        }
    }

    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        winit::window::WindowBuilder::new()
            .with_title("Salut")
            .with_inner_size((800, 600).into())
            .build(event_loop)
            .expect("Failed to create window.")
    }

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        
        let app_name = CString::new("Salut").unwrap();
        let engine_name = CString::new("Vulkan Engine").unwrap();
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: vk_make_version!(0, 0, 1),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk_make_version!(0, 0, 1),
            api_version: vk_make_version!(1, 0, 92),
        };

        let extension_names = required_extension_names();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names: ptr::null(),
            enabled_layer_count: 0,
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let instance: ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
    }

    fn draw_frame(&mut self) {
        // Drawing will be here
    }

    pub fn main_loop(mut self, event_loop: EventLoop<()>) {

        event_loop.run(move |event, _, control_flow| {

            match event {
                | Event::WindowEvent { event, .. } => {
                    match event {
                        | WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit
                        },
                        | WindowEvent::KeyboardInput { input, .. } => {
                            match input {
                                | KeyboardInput { virtual_keycode, state, .. } => {
                                    match (virtual_keycode, state) {
                                        | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                            *control_flow = ControlFlow::Exit
                                        },
                                        | _ => {},
                                    }
                                },
                            }
                        },
                        | _ => {},
                    }
                },
                | Event::EventsCleared => {
                    self.draw_frame();
                },
                _ => (),
            }

        })
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
pub fn required_extension_names() -> Vec<*const i8> {
    vec![
        Surface::name().as_ptr(),
        #[cfg(all(windows))]
        Win32Surface::name().as_ptr(),
        #[cfg(target_os = "macos")]
        MacOSSurface::name().as_ptr(),
        #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
        XlibSurface::name().as_ptr(),
        DebugUtils::name().as_ptr(),
    ]
}

fn main() {

    let event_loop = EventLoop::new();
    let _window = VulkanApp::init_window(&event_loop);

    let vulkan_app = VulkanApp::new();
    vulkan_app.main_loop(event_loop);
}
