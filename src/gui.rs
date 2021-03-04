//! This module combines many of the other modules to provide a safe wrapper
//! around the modules like windowing and renderering. It will allow for sharing
//! of data around from the window to the renderer, without sacrificing much usability for
//! the user.

use std::time::{Duration, Instant};

use crate::{layout::Layout, rendering::{Window, WindowBuilder, Renderer}};
use futures::executor::block_on;

use winit::event_loop::ControlFlow;
use winit::event::{WindowEvent, Event};

pub struct GUI{
    pub window: Window,
    pub renderer: Renderer,
    pub clear_color: wgpu::Color,
}

impl Default for GUI{
    fn default() -> GUI{
        let window = WindowBuilder::new().set_resolution((800, 600)).set_title("Rusty GUI app").build().expect("Error building window");
        let renderer = block_on(Renderer::new(&window.window));
        let clear_color = wgpu::Color::WHITE;
        GUI{
            window: window,
            renderer: renderer,
            clear_color: clear_color,
        }
    }
}

impl GUI{

    /// This function takes the data required by a GUI struct and wraps it into itself
    /// 
    /// You can alternatively call default to generate a default renderer and window.
    pub fn new(window: Window, renderer: Renderer, clear_color: wgpu::Color,) -> Self{
        Self{
            window,
            renderer,
            clear_color
        }
    }
}

// This part just has some helpful functions to simplify adding components
// and managing the GUI. Still needs a lot more functionality
impl GUI{
    /// The main loop of the application. This function will loop until the window is closed.
    ///
    /// It'll render the screen (GUI contents), draw text and check inputs (Which can be setup by the user with custom input handlers).
    /// 
    /// For example, a user can create a callback to handle a keyboard input. Examples and setup to come
    ///
    /// We should also implement a basic check for buttons to check where the cursor is and automatically handle button callbacks if the
    /// user doesn't want to implement callbacks themselves.
    pub fn main_loop(self){
        main_loop(self);
    }

    /// Sets the window event handler
    pub fn set_event_handler(&mut self, event_handler: Box<dyn Fn(&winit::event::Event<()>, &mut winit::window::Window, &mut crate::rendering::Renderer) -> ()>){
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


/// This function consumes a GUI struct and loops until application exit
/// 
/// This loop does NOT return once started
/// 
/// This workaround was also required as I had a lot of issues with references
fn main_loop(gui: GUI){
    let mut renderer = gui.renderer;
    let mut window = gui.window.window;
    let mut event_loop = gui.window.event_loop;
    let clear_color = gui.clear_color;
    let event_loop_handler = gui.window.event_callback_handler;
    let mut minimized = false;

    event_loop.take().unwrap().run(move |event, _, control_flow| {
        // ControlFlow::WaitUntil pauses the event loop if no events are available to process.
        // If no events are called, it will update every 10ms to make sure everything stays up to date
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::WaitUntil(Instant::now().checked_add(Duration::from_millis(250)).unwrap());

        if !minimized{
            // Run event components - things like buttons and so on
            for event_comp in renderer.layout.event_components.iter_mut(){
                event_comp.handle_event_callback(&event, &mut window);
            }
        }

        match &event_loop_handler{
            Some(v) => {
                // We have a callback handler, so run it below (with our required parameters)
                v(&event, &mut window, &mut renderer);
            }
            None => {
                // No callback handler set, so do nothing
            }
        }

        match event {
            // This part checks for a window event, then checks if its either an exit or resize
            // all other window events will be up to the user
            Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() =>  {
                    match event{
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(*physical_size);
                        if renderer.size.width == 0 && renderer.size.height == 0{
                            minimized = true;
                        }else{
                            minimized = false;
                        }
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        renderer.resize(**new_inner_size);
                        if renderer.size.width == 0 && renderer.size.height == 0{
                            minimized = true;
                        }else{
                            minimized = false;
                        }
                    },              
                    
                    _ => {}
                }
            }

            Event::MainEventsCleared => {
                // Application update code.
                if !minimized{
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    window.request_redraw();
                }
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
                renderer.prepass(); // Update the layout and stuff
                renderer.render(clear_color); // Render a single frame.
            }
            _ => {}
        }
    });
}