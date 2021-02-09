//! Simple example that shows how to get a window up and running,
//! with some basic event callbacks

use futures::executor::block_on;


/// A simple callback handler. It takes the event from the event loop, and will print out the contents
/// of the current event
fn event_callback_handler(event: winit::event::Event<()>, window: &mut winit::window::Window){
    //println!("Event: {:?}", event);
}

fn main(){
    println!("Starting!");

    // Choose to either build the window and renderer ourselves and pass it to a GUI,
    // or build the GUI with default values and change them through the window.

    // Uncomment choice.

    from_scratch();

    // from_default();
}

fn from_scratch(){
    let mut window_builder = rusty_gui::rendering::WindowBuilder::new();

    let mut window = window_builder.set_fullscreen(false).set_resolution((800, 600)).set_title("Hello Window!").set_vsync(true).build();

    window.set_event_handler(Box::new(event_callback_handler));

    let renderer = block_on(rusty_gui::rendering::Renderer::new(&window.window));

    let mut gui = rusty_gui::gui::GUI::new(window, renderer);

    gui.main_loop();
}

fn from_default(){
    let mut gui = rusty_gui::gui::GUI::default();

    gui.set_event_handler(Box::new(event_callback_handler));

    gui.main_loop();
}