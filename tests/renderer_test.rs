use futures::executor::block_on;
use rusty_gui::rendering::{Renderer, WindowBuilder};


/// Test that renderer contruction works as expected
#[test]
fn test_renderer(){
    let window = unsafe { WindowBuilder::new().build_unsafe().unwrap() };
    let _renderer = block_on(Renderer::new(&window.window));
}