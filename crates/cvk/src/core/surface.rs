use ash::vk;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

use crate::{Handle, core::instance::Instance};

pub struct Surface {
    handle: vk::SurfaceKHR,
}

impl Surface {
    pub fn new(instance: &Instance, window: &Window) -> Self {
        let display_handle = window.display_handle().expect("Failed to acquire display handle").as_raw();
        let window_handle = window.window_handle().expect("Failed to acquire window handle").as_raw();

        Self {
            handle: unsafe {
                ash_window::create_surface(&instance.entry, &instance.instance, display_handle, window_handle, None).expect("Failed to create surface")
            },
        }
    }
}

impl Handle<vk::SurfaceKHR> for Surface {
    fn handle(&self) -> vk::SurfaceKHR {
        self.handle
    }
}
