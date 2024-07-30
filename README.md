# gc9a01a_driver
 gc9a01a_driver

# GC9A01A Display Driver This is a Rust driver for the GC9A01A display, designed to work with embedded systems using the `embedded-hal` traits. The driver supports various display commands and allows for efficient updating of the display content. ## Features - Hardware SPI interface - Support for RGB and BGR color formats - Flexible display orientation - Partial and full display updates - Buffered pixel writes for efficient data transmission 

## Usage ### Dependencies Add the following dependencies to your `Cargo.toml`: 

```toml [dependencies] embedded-hal = "0.2.6"API Documentation
```

### Example
```
#![no_std]

#![no_main]

use embedded_hal::blocking::delay::DelayMs;

use embedded_hal::blocking::spi::Write;

use embedded_hal::digital::v2::OutputPin;

use my_display_driver::GC9A01A;

use my_display_driver::Instruction;

use my_display_driver::Orientation;

struct MyDelay;

impl DelayMs<u8> for MyDelay {

    fn delay_ms(&mut self, ms: u8) {

        // Implement delay function here

    }

}

fn main() {

    // Initialize SPI, DC, CS, and RST pins

    let spi = // Initialize SPI interface here

    let dc = // Initialize data/command pin here

    let cs = // Initialize chip select pin here

    let rst = // Initialize reset pin here

    // Create a new display driver instance

    let mut display = GC9A01A::new(spi, dc, cs, rst, true, 240, 240);

    // Initialize the display

    let mut delay = MyDelay;

    display.init(&mut delay).unwrap();

    // Set display orientation

    display.set_orientation(&Orientation::Portrait).unwrap();

    // Draw a single pixel

    display.set_pixel(120, 120, 0xFFFF).unwrap(); // White pixel in the center

    // Draw an image

    let image_data = [0x00; 240 * 240 * 2]; // Example image data

    display.draw_image(&image_data).unwrap();

    // Update a region of the display

    let region_data = [0xFF; 60 * 60 * 2]; // Example region data

    display.show_region(&region_data, 60, 60, 60, 60).unwrap();

}
```
-----------------

### GC9A01A Struct

#### `new`

Creates a new driver instance.

**Arguments:**

-   `spi`: SPI interface.
-   `dc`: Data/command pin.
-   `cs`: Chip select pin.
-   `rst`: Reset pin.
-   `rgb`: Whether the display is RGB (true) or BGR (false).
-   `width`: Width of the display.
-   `height`: Height of the display.

#### `init`

Initializes the display.

**Arguments:**

-   `delay`: Delay provider.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `set_orientation`

Sets the display orientation.

**Arguments:**

-   `orientation`: Orientation to set.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `set_pixel`

Sets a single pixel at the given coordinates.

**Arguments:**

-   `x`: X-coordinate.
-   `y`: Y-coordinate.
-   `color`: Color of the pixel.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `show_region`

Updates a specified region of the display with the provided buffer.

**Arguments:**

-   `buffer`: Buffer to display.
-   `top_left_x`: The x-coordinate of the top-left corner of the region to update.
-   `top_left_y`: The y-coordinate of the top-left corner of the region to update.
-   `width`: The width of the region to update.
-   `height`: The height of the region to update.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `draw_image`

Draws an image from a slice of RGB565 data.

**Arguments:**

-   `image_data`: Image data to draw.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

License
-------

This project is licensed under the MIT License. See the LICENSE file for details.

API Documentation
-----------------

### GC9A01A Struct

#### `new`

Creates a new driver instance.

**Arguments:**

-   `spi`: SPI interface.
-   `dc`: Data/command pin.
-   `cs`: Chip select pin.
-   `rst`: Reset pin.
-   `rgb`: Whether the display is RGB (true) or BGR (false).
-   `width`: Width of the display.
-   `height`: Height of the display.

#### `init`

Initializes the display.

**Arguments:**

-   `delay`: Delay provider.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `set_orientation`

Sets the display orientation.

**Arguments:**

-   `orientation`: Orientation to set.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `set_pixel`

Sets a single pixel at the given coordinates.

**Arguments:**

-   `x`: X-coordinate.
-   `y`: Y-coordinate.
-   `color`: Color of the pixel.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `show_region`

Updates a specified region of the display with the provided buffer.

**Arguments:**

-   `buffer`: Buffer to display.
-   `top_left_x`: The x-coordinate of the top-left corner of the region to update.
-   `top_left_y`: The y-coordinate of the top-left corner of the region to update.
-   `width`: The width of the region to update.
-   `height`: The height of the region to update.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

#### `draw_image`

Draws an image from a slice of RGB565 data.

**Arguments:**

-   `image_data`: Image data to draw.

**Returns:**

-   `Result<(), ()>` indicating success or failure.

License
-------

This project is licensed under the MIT License. See the LICENSE file for details.