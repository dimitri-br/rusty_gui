use rusty_gui::rendering::{WindowBuilder};


/// Test that window building works as expected
#[test]
pub fn window_builder_test(){
    let _ = unsafe { WindowBuilder::new().build_unsafe().unwrap() };
}