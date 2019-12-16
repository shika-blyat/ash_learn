use crate::utility::vulkanapp::VulkanApp;
use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    platform::desktop::EventLoopExtDesktop,
};
impl VulkanApp {
    pub fn run_loop(&self, window: Window, mut event_loop: EventLoop<()>) {
        event_loop.run_return(move |event, _, control_flow| match event {
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
