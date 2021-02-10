# Simple explaination of how it all fits together

You start off with a GUI, which will automatically create a Window and then a Renderer. The window has some parameters exposed to the user,
like the title, size, and more. The renderer however, only has vsync as an available option as well as the render layout.

### What is a render layout?

A render layout is a set of components, split into `text_components` and regular `components`. These get drawn by the renderer every frame.

A render layout can be switched in between frames, so you can split your application into multiple parts without worrying about having to sort and draw
in one big, messy vec.

### What does runtime code look like?

From the example `hello_window.rs`, here is what happens on execution:

1. Initialize a window

1. (OPTIONAL) Set the event handler, before we use the window for other things
   NOTE: this can still be changed through the GUI object

2. Initialize a renderer, using the window surface

3. Create a GUI, using the window and renderer as its fields

4. Create a layout

5. Create all the components we want to use in the layout

6. Add the components to the layout

7. Add the render layout to our renderer, setting it as the default for usage (using the GUI's abstracted methods)

8. Start the mainloop. From this point, you can only control the various GUI components through the event loop (for example, callbacks on button press).


### What about the code?

The code is split into multiple subfolders and submodules, to keep the code cleaner, and easier to understand.

* lib.rs -> Just rexports various structs, functions and whatever else we want to expose to the user

* layout.rs -> This file stores the struct `Layout`, which can be used to store the various components needed to render

* gui.rs -> This file contains the `GUI` struct, which holds a `Window` and `Renderer` struct. It simplifies the creation of windows and
            the renderer, and has some helpful functions to change the properties of the GUI window and renderer at runtime (such as changing the current layout)

* rendering -> This module holds the various files we will use to draw to the window.

* rendering/window.rs -> This file stores a struct called `Window`. This struct stores the event loop, winit window and the event callback handler. The various window
properties can be modified during the window build phase (with a struct called `WindowBuilder`). `gui.rs` has some helpful functions to modify things like the event callback
handler at runtime.

* rendering/render.rs -> This stores the `Renderer` struct, which is a low power implementation of wgpu-rs. It typically runs on dx12, metal or vulkan, however thanks to the 
event loop waiting instead of polling each frame, it is pretty lightweight on CPU and GPU resources. This struct handles rendering to the screen, and takes in a layout. This layout
can be swapped at runtime to swap what you want to render. 

* components -> This module stores the various components that come with this library, as well as the traits to build your own components, for a more user-driven modular design.

* components/base_components.rs -> This file stores the traits, `GUIComponent` and `TextGUIComponent`. Adding these traits will make a struct addable to the layout, and therefore will be renderered by the renderer. It also contains some basic components such as `Label`, `Button` and some more TODO.

* shaders -> This folder contains the shaders used by the renderer to render to the screen. It is recommended to leave this alone unless you know what you're doing. They're not meant to be complex or powerful, but simple, fast and efficient

* fonts -> This folder contains the fonts to be used by the GUI to render. Hopefully we can get this to render more fonts, as well as user defined fonts per label (or `TextGUIComponent`).

* examples -> This folder contains some commented and documented examples to help you get started with rusty_gui. It shows how you can use various components of this crate and put them together.