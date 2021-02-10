//! This module combines many of the other modules to provide a safe wrapper
//! around the modules like windowing and renderering. It will allow for sharing
//! of data around from the window to the renderer, without sacrificing much usability for
//! the user.

use crate::{layout::Layout, rendering::{Window, WindowBuilder, Renderer}};
use futures::executor::block_on;

pub struct GUI{
    pub window: Window,
    pub renderer: Renderer,
}

impl Default for GUI{
    fn default() -> GUI{
        let window = WindowBuilder::new().set_resolution((800, 600)).set_title("Rusty GUI app").build();
        let renderer = block_on(Renderer::new(&window.window));
        GUI{
            window: window,
            renderer: renderer,
        }
    }
}

impl GUI{

    /// This function takes the data required by a GUI struct and wraps it into itself
    /// 
    /// You can alternatively call default to generate a default renderer and window.
    pub fn new(window: Window, renderer: Renderer) -> Self{
        Self{
            window,
            renderer,
        }
    }
}

// This part just has some helpful functions to simplify adding components
// and managing the GUI. Still needs a lot more functionality
impl GUI{
    /// Runs mainloop on this structs window field
    pub fn main_loop(&mut self){
        self.window.main_loop(&mut self.renderer);
    }

    /// Sets the window event handler
    pub fn set_event_handler(&mut self, event_handler: Box<dyn Fn(winit::event::Event<()>, &mut winit::window::Window) -> ()>){
        self.window.set_event_handler(event_handler)
    }

    /// Gets a reference to the winit window. Used to make wgpu surfaces
    pub fn get_window_ref(&self) -> &winit::window::Window{
        &self.window.window
    }

    /// Sets the current components to render, consuming the layout in the process
    pub fn set_render_layout(&mut self, layout: Layout){
        self.renderer.layout = layout;
    }

    /// Returns a mutable reference to the currently active render layout
    pub fn borrow_render_layout(&mut self) -> &mut Layout{
        &mut self.renderer.layout
    }

    /// Borrow the render device (Used for things like creating buffers, and creating certain components)
    pub fn borrow_render_device(&self) -> &wgpu::Device{
        &self.renderer.device
    }

    /// Borrow the winit window handle
    pub fn borrow_raw_window(&mut self) -> &mut winit::window::Window{
        &mut self.window.window
    }

    /// Borrow the renderer (eg, if you require multiple fields from the renderer, it might be easier to just pass the whole struct)
    pub fn borrow_renderer(&self) -> &Renderer{
        &self.renderer
    }
}