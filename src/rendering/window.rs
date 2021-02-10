//! This file contains all our windowing functions to simplify interfacing with the window
//! it should make it easier to access input, modify the window and access window variables
//! for the user of the library and the developer

use winit::{dpi, event_loop, monitor, platform::run_return::EventLoopExtRunReturn, window};

use winit::event_loop::ControlFlow;
use winit::event::{WindowEvent, Event};

use crate::components::EventGUIComponent;


/// # Window
///
/// This struct contains information for the window used in a GUI application
///
/// It is designed to be used to abstract away from some of the low-levelness of winit
/// and create a simpler, although less powerful API to window functions
/// 
/// ## Usage
///
/// This struct should be made using a window builder
/// 
/// Once the window is build, set the event handler using `set_event_handler`
pub struct Window{
    pub window: window::Window,
    pub event_loop: Option<event_loop::EventLoop<()>>,
    pub event_callback_handler: Option<Box<dyn Fn(Event<()>, &mut window::Window, &mut crate::rendering::Renderer) -> ()>>,
}


impl Window{
    /// The main loop of the application. This function will loop until the window is closed.
    ///
    /// It'll render the screen (GUI contents), draw text and check inputs (Which can be setup by the user with custom input handlers).
    /// 
    /// For example, a user can create a callback to handle a keyboard input. Examples and setup to come
    ///
    /// We should also implement a basic check for buttons to check where the cursor is and automatically handle button callbacks if the
    /// user doesn't want to implement callbacks themselves.
    pub fn main_loop(&mut self, renderer: &mut crate::rendering::Renderer){

        // Take our event callback handler. We need to take before we start the run, as it isn't thread safe to take it
        // during the loop
        self.event_loop.take().unwrap().run_return(move |event, _, control_flow| {
            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            *control_flow = ControlFlow::Wait;
            
            match event {
                // This part checks for a window event, then checks if its either an exit or resize
                // all other window events will be up to the user
                Event::WindowEvent {
                        ref event,
                        window_id,
                    } if window_id == self.window.id() =>  {
                        match event{
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::KeyboardInput {
                            input,
                            ..
                        } => {
                            match input {
                                winit::event::KeyboardInput {
                                    state: winit::event::ElementState::Pressed,
                                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                    ..
                                } => {    
                                    *control_flow = ControlFlow::Exit
                                },
                                _ => {}
                            }
                        },
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            renderer.resize(**new_inner_size);
                        },
                        
                        _ => {}
                    }
                }

                Event::MainEventsCleared => {
                    // Application update code.
        
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    self.window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in MainEventsCleared, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.

                    renderer.render(); // Render a single frame.
                }
                _ => {}
            }

            match &self.event_callback_handler{
                Some(v) => {
                    // We have a callback handler, so run it below (with our required parameters)
                    v(event, &mut self.window, renderer);
                }
                None => {
                    // No callback handler set, so do nothing
                }
            }
        });
    }

    /// The default event callback handler.
    ///
    /// You can define your own to handle events
    ///
    /// Button presses will still be automatically handled.
    pub fn default_event_callback(event: Event<()>, _window: &mut window::Window, _renderer: &mut crate::rendering::Renderer){
        println!("Event: {:?}", event);
    }

    /// Sets the event callback handler. This cannot be changed once the GUI is running.
    pub fn set_event_handler(&mut self, event_handler: Box<dyn Fn(Event<()>, &mut window::Window, &mut crate::rendering::Renderer) -> ()>){
        self.event_callback_handler = Some(event_handler);
    }
}

/// # WindowBuilder
/// 
/// This builds a window struct, based either on default values or
/// user defined values. Meant to simplify and abstract winit's WindowBuilder,
/// for ease of use when making GUI applications.
#[derive(Debug)]
pub struct WindowBuilder{
    resolution: (u32, u32),
    title: String,
    vsync: bool,
    screen_mode: ScreenMode,
    resizeable: bool,
    decorations: bool,
}

/// Default init for WindowBuilder
impl Default for WindowBuilder{
    fn default() -> WindowBuilder{
        Self{
            resolution: (800, 600),
            title: String::from("Rusty GUI"),
            vsync: true,
            screen_mode: ScreenMode::Windowed,
            resizeable: true,
            decorations: true,
            
        }
    }
}

/// Helpful functions to define variables for a window
impl WindowBuilder{
    /// Create a new window builder with default values
    pub fn new() -> Self{
        Self::default()
    }

    /// Set the resolution for the window (Can be any u32, but be reasonable :D)
    pub fn set_resolution(&mut self, resolution: (u32, u32)) -> &mut Self{
        self.resolution = resolution;
        self
    }

    /// Set Vsync mode (can be true or false)
    pub fn set_vsync(&mut self, vsync_enabled: bool) -> &mut Self{
        self.vsync = vsync_enabled;
        self
    }

    /// Set the title of the window - can be any type that is a String
    pub fn set_title<S: Into<String>>(&mut self, title: S) -> &mut Self{
        self.title = title.into();
        self
    }

    /// Set the fullscreen to true or false - make this enum (FULL, BORDERLESS, WINDOW)
    pub fn set_screenmode(&mut self, screen_mode: ScreenMode) -> &mut Self{
        self.screen_mode = screen_mode;
        self
    }

    /// Enable or disable decorations (the bar at the top of the window)
    pub fn set_decorations(&mut self, decorations_enabled: bool) -> &mut Self{
        self.decorations = decorations_enabled;
        self
    }

    /// Enable or disable resizing
    pub fn set_resizeable(&mut self, resizable: bool) -> &mut Self{
        self.resizeable = resizable;
        self
    }

    /// Build the window and return a Window
    pub fn build(&mut self) -> Window{
        // Create our winit WindowBuilder
        let winit_builder = window::WindowBuilder::new();

                
        // Create an event loop
        let mut event_loop = event_loop::EventLoop::new();

  
        // Gather information about the monitor and video modes for fullscreen and stuff
        let mut x = 0;
        let mut monitor: Vec<monitor::MonitorHandle> = event_loop.available_monitors().filter(|_| if x == 0 { x += 1; true }else{ false }).collect();
        let monitor = monitor.swap_remove(0);
        
        let mut x = 0;
        let mut video_modes: Vec<monitor::VideoMode> = monitor.video_modes().filter(|_| if x == 0 { x += 1; true }else{ false }).collect();
        let video_modes = video_modes.swap_remove(0);

        // Vsync mode - refresh rate
        let _vsync_mode = match self.vsync{
            true => {
                wgpu::PresentMode::Fifo
            }
            false => {
                wgpu::PresentMode::Mailbox
            }
        };

        // Check if we're running fullscreen and/or set resolutions
        let winit_builder = match self.screen_mode{
            ScreenMode::Fullscreen => {
                winit_builder.with_fullscreen(Some(window::Fullscreen::Exclusive(video_modes)))
            }
            ScreenMode::Windowed => {
                winit_builder.with_inner_size(dpi::Size::from(dpi::LogicalSize{ width: self.resolution.0, height: self.resolution.1}))
            }
            ScreenMode::Borderless => {
                winit_builder.with_fullscreen(Some(window::Fullscreen::Borderless(Some(monitor))))
            }
        };

        
        // Build the window
        Window{
            window: winit_builder.with_resizable(self.resizeable).with_decorations(self.decorations).with_title(&self.title).build(&mut event_loop).expect("Failed to build window!"),
            event_loop: Some(event_loop),
            event_callback_handler: Some(Box::new(Window::default_event_callback)),
        }
        
    }
}
#[derive(Debug)]
pub enum ScreenMode{
    Fullscreen,
    Borderless,
    Windowed
}