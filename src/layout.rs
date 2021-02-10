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


// This part can definitely be improved but I'm not sure how
impl Layout{
    /// Initialize a new layout
    pub fn new() -> Self{
        Self{
            components: Vec::<Box<dyn GUIComponent>>::new(),
            text_components: Vec::<Box<dyn TextGUIComponent>>::new(),
        }
    }
    
    /// Adds a new component, Only accepts a GUIComponent type, and returns the ID (location in vec) of the component
    pub fn add_component<T: GUIComponent + 'static>(&mut self, comp: Box<T>) -> usize{
        self.components.push(comp);

        self.text_components.len() - 1
    }

    /// Adds a new component, Only accepts a TextGUIComponent type and return the ID (location in the vec) of the component
    pub fn add_text_component<T: TextGUIComponent + 'static>(&mut self, comp: Box<T>) -> usize{
        self.text_components.push(comp);

        self.text_components.len() - 1
    }

    /// Remove a component from the vec using the ID of the component
    pub fn remove_component_by_id(&mut self, id: usize){
        self.components.remove(id);
    }

    /// Remove a text component from the vec using the ID of the text component
    pub fn remove_text_component_by_id(&mut self, id: usize){
        self.text_components.remove(id);
    }

    /// Borrow a component (non modifiable)
    pub fn borrow_component(&mut self, id: usize) -> &Box<dyn GUIComponent>{
        &self.components[id]
    }

    /// Borrow a component mutably
    pub fn borrow_component_mut(&mut self, id: usize) -> &mut Box<dyn GUIComponent>{
        &mut self.components[id]
    }

    /// Borrow a text component (non modifiable)
    pub fn borrow_text_component(&mut self, id: usize) -> &Box<dyn TextGUIComponent>{
        &self.text_components[id]
    }

    /// Borrow a text component mutably
    pub fn borrow_text_component_mut(&mut self, id: usize) -> &mut Box<dyn TextGUIComponent>{
        &mut self.text_components[id]
    }
}