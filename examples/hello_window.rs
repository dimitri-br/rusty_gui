//! Simple example that shows how to get a window up and running,
//! with some basic event callbacks

// We use block_on as Renderer creation requires async, but our app isn't configured to use async.
use futures::executor::block_on;
use rusty_gui::{components::{Button, EventGUIComponent, Label}, gui::GUI, layout::Layout, rendering::{Renderer, ScreenMode, Transform, WindowBuilder}};


/// A simple callback handler. Shows how it works, so you can extend it
fn event_callback_handler(event: winit::event::Event<()>, window: &mut winit::window::Window, renderer: &mut rusty_gui::rendering::Renderer){
    // Handle events


    // Run event components - things like buttons and so on
    for event_comp in renderer.layout.event_components.iter(){
        event_comp.handle_event_callback(&event, &window);
    }
}

fn main(){
    println!("Starting!");

    // Choose to either build the window and renderer ourselves and pass it to a GUI,
    // or build the GUI with default values and change them through the window.

    // Uncomment choice.

    //_from_scratch();

    _from_default();
}

fn _from_scratch(){
    let mut window_builder = WindowBuilder::new();

    let mut window = window_builder.set_screenmode(ScreenMode::Borderless).set_resolution((800, 600)).set_title("Hello Window!").set_vsync(true).build();

    window.set_event_handler(Box::new(event_callback_handler));

    let renderer = block_on(Renderer::new(&window.window));

    let mut gui = GUI::new(window, renderer);

    let mut layout = Layout::new();

    let label = Label::new("Hello, world!", 128.0, [100.0, 100.0]);

    layout.add_text_component(Box::new(label));

    gui.set_render_layout(layout);

    gui.main_loop();
}

fn test_button_func(){
    println!("Hello, world!");
}

// Shows how to create a simple label-based GUI from default vals
fn _from_default(){
    let mut gui = GUI::default(); // Create the gui with default values (which inits the window and renderer)

    gui.set_event_handler(Box::new(event_callback_handler)); // Set the event handler to our custom event handler

    let mut layout = Layout::new(); // We initialize a new layout

    // Set the components we want to add
    let label = Label::new("Hello, world!", 128.0, [0.0, 0.0]);
    let label_1 = Label::new("Damn this sucks", 32.0, [200.0, 500.0]);
    let label_2 = Label::new("Big F", 64.0, [70.0, 450.0]);
    
    /// Simple button
    let button = Button::new(
        Transform::new(
            cgmath::Vector3::<f32>::new(300.0, 350.0, 0.0), 
            cgmath::Quaternion::<f32>::new(0.0, 0.0, 0.0, 0.0), 
            cgmath::Vector3::<f32>::new(75.0, 75.0, 10.0,), gui.borrow_render_device()), 
        Box::new(test_button_func));

    // Add the components to the layout - the order only matters if you want the components to render in a specific way
    // Text will ALWAYS be rendered on top of everything else, that is something to fix
    layout.add_text_component(Box::new(label));
    layout.add_text_component(Box::new(label_2));
    layout.add_text_component(Box::new(label_1));

    layout.add_event_component(Box::new(button));

    // Set the renderer render layout to our layout - this will consume our layout, so to access it,
    // use `gui.borrow_render_layout()`
    gui.set_render_layout(layout);

    // This shows how we can access the winit window to modify values directly
    gui.borrow_raw_window().set_title("Hello window!");

    // We now loop, handing the app control of the program. We can use event (or component) callbacks to define
    // how we want the app to run given certain events.
    gui.main_loop();
}