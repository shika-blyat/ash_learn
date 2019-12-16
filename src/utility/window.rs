use crate::utility::vulkanapp::VulkanApp;
use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
impl VulkanApp {
    pub fn run_loop(&self) {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).expect("Window creation failed");
        event_loop.run(move |event, _, control_flow| match event {
            Event::EventsCleared => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {}
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Closing Window !");
                *control_flow = ControlFlow::Exit
            }
            _ => *control_flow = ControlFlow::Poll,
        });
    }
}
