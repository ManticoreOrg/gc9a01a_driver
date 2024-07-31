# GC9A01A Display Driver

A Rust library for interfacing with the GC9A01A display using the `embedded-hal` and `embedded-graphics` crates.

## Features

- Hardware SPI interface
- RGB and BGR support
- Display orientation support
- Drawing images and individual pixels
- Frame buffer for efficient display updates

## Getting Started

### Prerequisites

- Rust toolchain
- `embedded-hal` crate
- `embedded-graphics` crate

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
embedded-hal = "0.2"
embedded-graphics = "0.7"
gc9a01a = { path = "path/to/your/gc9a01a" }
```

### Example

Here's an example of how to use the driver to initialize the display:

``` rust

    // Initialize LCD pins
    let lcd_dc = pins.gp8.into_push_pull_output();
    let lcd_cs = pins.gp9.into_push_pull_output();
    let lcd_clk = pins.gp10.into_function::<hal::gpio::FunctionSpi>();
    let lcd_mosi = pins.gp11.into_function::<hal::gpio::FunctionSpi>();
    let lcd_rst = pins.gp12.into_push_pull_output_in_state(hal::gpio::PinState::High);
    let mut _lcd_bl = pins.gp25.into_push_pull_output_in_state(hal::gpio::PinState::Low);

    // Initialize SPI
    let spi = hal::Spi::<_, _, _, 8>::new(pac.SPI1, (lcd_mosi, lcd_clk));
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        40.MHz(),
        embedded_hal::spi::MODE_0,
    );

    // Initialize the display
    let mut display = GC9A01A::new(spi, lcd_dc, lcd_cs, lcd_rst, false, LCD_WIDTH, LCD_HEIGHT);
    display.init(&mut delay).unwrap();
    display.set_orientation(&Orientation::Landscape).unwrap();

```

### API Documentation

#### `Instruction` Enum

Enumeration of instructions for the GC9A01A display.

#### `GC9A01A` Struct

Driver for the GC9A01A display.

-   `new(spi, dc, cs, rst, rgb, width, height) -> Self`: Creates a new driver instance.
-   `init(&mut self, delay: &mut impl DelayMs<u8>) -> Result<(), ()>`: Initializes the display.
-   `set_orientation(&mut self, orientation: &Orientation) -> Result<(), ()>`: Sets the display orientation.
-   `write_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), ()>`: Sets a pixel color at the given coordinates.
-   `draw_image(&mut self, image_data: &[u8]) -> Result<(), ()>`: Draws an image from a slice of RGB565 data.
-   `show(&mut self, buffer: &[u8]) -> Result<(), ()>`: Displays the provided buffer on the screen.
-   `show_region(&mut self, buffer: &[u8], top_left_x: u16, top_left_y: u16, width: u16, height: u16) -> Result<(), ()>`: Updates only the specified region of the display with the provided buffer.

#### `Orientation` Enum

Display orientation options.

-   `Portrait`
-   `Landscape`
-   `PortraitSwapped`
-   `LandscapeSwapped`

#### `FrameBuffer` Struct

A structure representing a frame buffer.

-   `new(buffer: &mut [u8], width: u32, height: u32) -> Self`: Creates a new frame buffer.
-   `get_buffer(&self) -> &[u8]`: Returns a reference to the buffer.
-   `clear(&mut self, color: Rgb565)`: Clears the frame buffer with the specified color.
-   `copy_region(&mut self, src_buffer: &[u8], src_top_left: Point, src_size: Size, dest_top_left: Point)`: Copies a region from another buffer into this buffer.

Contributing
------------

1.  Fork the repository.
2.  Create a feature branch (`git checkout -b feature-branch`).
3.  Commit your changes (`git commit -am 'Add new feature'`).
4.  Push to the branch (`git push origin feature-branch`).
5.  Create a new Pull Request.

License
-------

This project is licensed under the MIT License.

Acknowledgments
---------------

-   [embedded-hal](https://github.com/rust-embedded/embedded-hal)
-   [embedded-graphics](https://github.com/embedded-graphics/embedded-graphics)

License
-------

This project is licensed under the MIT License. See the `LICENSE` file for more details.

