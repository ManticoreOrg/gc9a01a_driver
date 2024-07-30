# GC9A01A Display Driver

This project is a driver implementation for the GC9A01A display using Rust. The driver allows for the initialization, configuration, and control of the display, providing various functions for drawing graphics, setting pixel colors, and handling display orientation.

## Features

- No standard library (`#![no_std]`)
- Compatible with embedded systems
- Supports SPI interface
- Configurable RGB/BGR display mode
- Multiple display orientations
- Functions for drawing pixels and images

## Usage

To use this driver, include it in your Rust project as a dependency and instantiate the driver with the required SPI and GPIO pins.

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

API Reference
-------------

### Structs

#### `GC9A01A<SPI, DC, CS, RST>`

The main driver struct. Provides methods to interact with the display.

-   `new(spi: SPI, dc: DC, cs: CS, rst: RST, rgb: bool, width: u32, height: u32) -> Self`: Creates a new driver instance.
-   `init<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>`: Initializes the display.
-   `hard_reset<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>`: Performs a hard reset of the display.
-   `write_command(&mut self, command: u8, params: &[u8]) -> Result<(), ()>`: Writes a command to the display.
-   `start_data(&mut self) -> Result<(), ()>`: Starts data transmission.
-   `write_data(&mut self, data: &[u8]) -> Result<(), ()>`: Writes data to the display.
-   `write_word(&mut self, value: u16) -> Result<(), ()>`: Writes a data word to the display.
-   `write_words_buffered(&mut self, words: impl IntoIterator<Item = u16>) -> Result<(), ()>`: Writes buffered data words to the display.
-   `set_orientation(&mut self, orientation: &Orientation) -> Result<(), ()>`: Sets the orientation of the display.
-   `set_offset(&mut self, dx: u16, dy: u16)`: Sets the global offset of the displayed image.
-   `set_address_window(&mut self, start_x: u16, start_y: u16, end_x: u16, end_y: u16) -> Result<(), ()>`: Sets the address window for the display.
-   `write_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), ()>`: Sets a pixel color at the given coordinates.
-   `write_pixels_continuous<P: IntoIterator<Item = u16>>(&mut self, colors: P) -> Result<(), ()>`: Writes pixel colors sequentially into the current drawing window.
-   `write_pixels_buffered<P: IntoIterator<Item = u16>>(&mut self, colors: P) -> Result<(), ()>`: Writes buffered pixel colors sequentially into the current drawing window.
-   `set_window_and_write_pixels<P: IntoIterator<Item = u16>>(&mut self, start_x: u16, start_y: u16, end_x: u16, end_y: u16, colors: P) -> Result<(), ()>`: Sets pixel colors at the given drawing window.
-   `set_window_and_write_pixels_buffered<P: IntoIterator<Item = u16>>(&mut self, start_x: u16, start_y: u16, end_x: u16, end_y: u16, colors: P) -> Result<(), ()>`: Sets buffered pixel colors at the given drawing window.
-   `draw_image(&mut self, image_data: &[u8]) -> Result<(), ()>`: Draws an image from a slice of RGB565 data.
-   `show(&mut self, buffer: &[u8]) -> Result<(), ()>`: Displays the provided buffer on the screen.
-   `show_region(&mut self, buffer: &[u8], top_left_x: u16, top_left_y: u16, width: u16, height: u16) -> Result<(), ()>`: Updates only the specified region of the display with the provided buffer.

### Enums

#### `Instruction`

Enumeration of instructions for the GC9A01A display.

-   Variants: `Nop`, `SwReset`, `RddId`, `RddSt`, `SlpIn`, `SlpOut`, `PtlOn`, `NorOn`, `InvOff`, `InvOn`, `DispOff`, `DispOn`, `CaSet`, `RaSet`, `RamWr`, `RamRd`, `PtlAr`, `ColMod`, `MadCtl`, `FrmCtr1`, `FrmCtr2`, `FrmCtr3`, `InvCtr`, `DisSet5`, `PwCtr1`, `PwCtr2`, `PwCtr3`, `PwCtr4`, `PwCtr5`, `VmCtr1`, `RdId1`, `RdId2`, `RdId3`, `RdId4`, `PwCtr6`, `GmcTrp1`, `GmcTrn1`.

#### `Orientation`

Enumeration of display orientations.

-   Variants: `Portrait`, `Landscape`, `PortraitSwapped`, `LandscapeSwapped`.

License
-------

This project is licensed under the MIT License. See the `LICENSE` file for more details.

