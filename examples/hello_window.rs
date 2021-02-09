//! Simple example that shows how to get a window up and running,
//! with some basic event callbacks

use futures::executor::block_on;
use rusty_gui::{components::Label, layout::Layout};


/// A simple callback handler. Shows how it works, so you can extend it
fn event_callback_handler(_event: winit::event::Event<()>, _window: &mut winit::window::Window){
    //println!("Event: {:?}", event);
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
    let mut window_builder = rusty_gui::rendering::WindowBuilder::new();

    let mut window = window_builder.set_fullscreen(false).set_resolution((800, 600)).set_title("Hello Window!").set_vsync(true).build();

    window.set_event_handler(Box::new(event_callback_handler));

    let renderer = block_on(rusty_gui::rendering::Renderer::new(&window.window));

    let mut gui = rusty_gui::gui::GUI::new(window, renderer);

    let mut layout = Layout::new();

    let label = Label::new("Hello, world!", 128.0, [100.0, 100.0]);

    layout.add_text_component(Box::new(label));

    gui.set_render_layout(layout);

    gui.main_loop();
}

fn _from_default(){
    let mut gui = rusty_gui::gui::GUI::default();

    gui.set_event_handler(Box::new(event_callback_handler));

    let mut layout = Layout::new();

    let label = Label::new("Hello, world!", 128.0, [0.0, 0.0]);
    let label_1 = Label::new("Damn this sucks", 32.0, [200.0, 500.0]);
    let label_2 = Label::new("Big F", 16.0, [70.0, 350.0]);

    layout.add_text_component(Box::new(label));
    layout.add_text_component(Box::new(label_2));
    layout.add_text_component(Box::new(label_1));

    gui.set_render_layout(layout);

    gui.main_loop();
}