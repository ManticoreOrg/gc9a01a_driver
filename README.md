GC9A01A Display Driver
======================

This crate provides a driver for the GC9A01A display, enabling basic operations such as initialization, clearing the screen, setting pixels, drawing images, and displaying buffers. The driver is built using the `embedded-hal` and `embedded-graphics` crates to ensure compatibility with various embedded platforms.

Features
--------

-   Initialization of the GC9A01A display
-   Setting display orientation
-   Clearing the screen
-   Setting individual pixels
-   Drawing images and displaying buffers
-   Partial updates to the display

Usage
-----

### Adding the Crate

To use this crate, add the following dependencies to your `Cargo.toml`:

-   `embedded-hal`
-   `embedded-graphics`
-   `gc9a01a_driver`

API Overview
------------

### GC9A01A

The `GC9A01A` struct provides methods to interact with the display. Key methods include:

-   `new`: Creates a new instance of the GC9A01A driver.
-   `init`: Initializes the display with a given delay provider.
-   `set_orientation`: Sets the display orientation.
-   `clear_screen`: Clears the screen with a specific color.
-   `write_pixel`: Sets the color of a single pixel.
-   `draw_image`: Draws an image from a slice of RGB565 data.
-   `show`: Displays the provided buffer on the screen.
-   `show_region`: Updates only the specified region of the display with the provided buffer.

### FrameBuffer

The `FrameBuffer` struct represents a frame buffer and includes methods to manipulate it:

-   `new`: Creates a new frame buffer.
-   `get_buffer`: Returns a reference to the buffer.
-   `clear`: Clears the frame buffer with the specified color.
-   `copy_region`: Copies a region from another buffer into this buffer.

Contributing
------------

Contributions are welcome! Please feel free to submit a pull request or open an issue.

License
-------

This project is licensed under the MIT License - see the LICENSE file for details.