//! Layouts are kind of like data structures that store the various components
//! required to render the GUI. This modular system, coupled with more control over
//! rendering allows us to swap layouts - things to render - at any point during rendering
//! with little to no delay.


use crate::components::{EventGUIComponent, GUIComponent, TextGUIComponent};

/// # Layout
///
/// Layout struct stores the data needed to render a layout
///
/// It stores one for regular image based GUI components,
/// and one for rendering text based components like labels.
/// It also stores event components, components which should check events.
pub struct Layout{
    pub components: Vec<Box<dyn GUIComponent>>,
    pub event_components: Vec<Box<dyn EventGUIComponent>>,
    pub text_components: Vec<Box<dyn TextGUIComponent>>,
}


// This part can definitely be improved but I'm not sure how
impl Layout{
    /// Initialize a new layout
    pub fn new() -> Self{
        Self{
            components: Vec::<Box<dyn GUIComponent>>::new(),
            event_components: Vec::<Box<dyn EventGUIComponent>>::new(),
            text_components: Vec::<Box<dyn TextGUIComponent>>::new(),
        }
    }
    
    /// Adds a new component, Only accepts a GUIComponent type, and returns the ID (location in vec) of the component
    pub fn add_component<T: GUIComponent + 'static>(&mut self, comp: Box<T>) -> usize{
        self.components.push(comp);

        self.components.len() - 1
    }

    /// Adds a new component, Only accepts a TextGUIComponent type and return the ID (location in the vec) of the component
    pub fn add_text_component<T: TextGUIComponent + 'static>(&mut self, comp: Box<T>) -> usize{
        self.text_components.push(comp);

        self.text_components.len() - 1
    }

    /// Adds a new event component, Only accepts a EventGUIComponent type, and returns the ID (location in vec) of the component
    pub fn add_event_component<T: EventGUIComponent + 'static>(&mut self, comp: Box<T>) -> usize{
        self.event_components.push(comp);

        self.event_components.len() - 1
    }

    /// Remove a component from the vec using the ID of the component
    pub fn remove_component_by_id(&mut self, id: usize){
        self.components.remove(id);
    }

    /// Remove a text component from the vec using the ID of the text component
    pub fn remove_text_component_by_id(&mut self, id: usize){
        self.text_components.remove(id);
    }

    /// Remove a event component from the vec using the ID of the component
    pub fn remove_event_component_by_id(&mut self, id: usize){
        self.event_components.remove(id);
    }

    /// Borrow a component (non modifiable)
    pub fn borrow_component(&self, id: usize) -> &Box<dyn GUIComponent>{
        &self.components[id]
    }

    /// Borrow a component as a type (non modifiable)
    pub fn borrow_component_as_type<T: GUIComponent + 'static>(&self, id: usize) -> Result<&T, &'static str>{
        let comp = self.components.get(id).take().unwrap();
        if let Some(downcast) = comp.as_any().downcast_ref::<T>(){
            return Ok(downcast);
        }
        return Err("Error, failed to downcast!");
    }

    /// Borrow a component mutably
    pub fn borrow_component_mut(&mut self, id: usize) -> &mut Box<dyn GUIComponent>{
        &mut self.components[id]
    }

    /// Borrow a component as a type (modifiable)
    pub fn borrow_component_as_type_mut<T: GUIComponent + 'static>(&mut self, id: usize) -> Result<&mut T, &'static str>{
        let comp = self.components.get_mut(id).take().unwrap();
        if let Some(downcast) = comp.as_any_mut().downcast_mut::<T>(){
            return Ok(downcast);
        }
        return Err("Error, failed to downcast!");
    }

    /// Borrow a text component (non modifiable)
    pub fn borrow_text_component(&mut self, id: usize) -> &Box<dyn TextGUIComponent>{
        &self.text_components[id]
    }

    /// Borrow a text component as a type (non modifiable)
    pub fn borrow_text_component_as_type<T: TextGUIComponent + 'static>(&self, id: usize) -> Result<&T, &'static str>{
        let comp = self.text_components.get(id).take().unwrap();
        if let Some(downcast) = comp.as_any().downcast_ref::<T>(){
            return Ok(downcast);
        }
        return Err("Error, failed to downcast!");
    }

    /// Borrow a text component mutably
    pub fn borrow_text_component_mut(&mut self, id: usize) -> &mut Box<dyn TextGUIComponent>{
        &mut self.text_components[id]
    }

    /// Borrow a text component as a type (modifiable)
    pub fn borrow_text_component_as_type_mut<T: TextGUIComponent + 'static>(&mut self, id: usize) -> Result<&mut T, &'static str>{
        let comp = self.text_components.get_mut(id).take().unwrap();
        if let Some(downcast) = comp.as_any_mut().downcast_mut::<T>(){
            return Ok(downcast);
        }
        return Err("Error, failed to downcast!");
    }

    /// Borrow a event component (non modifiable)
    pub fn borrow_event_component(&mut self, id: usize) -> &Box<dyn EventGUIComponent>{
        &self.event_components[id]
    }

    /// Borrow a event component as a type (non modifiable)
    pub fn borrow_event_component_as_type<T: EventGUIComponent + 'static>(&self, id: usize) -> Result<&T, &'static str>{
        let comp = self.event_components.get(id).take().unwrap();
        if let Some(downcast) = comp.as_any().downcast_ref::<T>(){
            return Ok(downcast);
        }
        return Err("Error, failed to downcast!");
    }

    /// Borrow a event component mutably
    pub fn borrow_event_component_mut(&mut self, id: usize) -> &mut Box<dyn EventGUIComponent>{
        &mut self.event_components[id]
    }

    /// Borrow a event component as a type (modifiable)
    pub fn borrow_event_component_as_type_mut<T: EventGUIComponent + 'static>(&mut self, id: usize) -> Result<&mut T, &'static str>{
        let comp = self.event_components.get_mut(id).take().unwrap();
        if let Some(downcast) = comp.as_any_mut().downcast_mut::<T>(){
            return Ok(downcast);
        }
        return Err("Error, failed to downcast!");
    }
}