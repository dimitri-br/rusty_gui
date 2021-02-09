//! Layouts are kind of like data structures that store the various components
//! required to render the GUI. This modular system, coupled with more control over
//! rendering allows us to swap layouts - things to render - at any point during rendering
//! with little to no delay.

use crate::components::{GUIComponent, TextGUIComponent};

/// # Layout
///
/// Layout struct stores the data needed to render a layout
///
/// It stores one for regular image based GUI components,
/// and one for rendering text based components like labels.
pub struct Layout{
    pub components: Vec<Box<dyn GUIComponent>>,
    pub text_components: Vec<Box<dyn TextGUIComponent>>,
}

impl Layout{
    /// Initialize a new layout
    pub fn new() -> Self{
        Self{
            components: Vec::<Box<dyn GUIComponent>>::new(),
            text_components: Vec::<Box<dyn TextGUIComponent>>::new(),
        }
    }
    
    /// Adds a new component, Only accepts a GUIComponent type
    pub fn add_component<T: GUIComponent + 'static>(&mut self, comp: Box<T>){
        self.components.push(comp);
    }

    /// Adds a new component, Only accepts a TextGUIComponent type
    pub fn add_text_component<T: TextGUIComponent + 'static>(&mut self, comp: Box<T>){
        self.text_components.push(comp);
    }
}