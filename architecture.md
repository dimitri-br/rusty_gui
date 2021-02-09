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