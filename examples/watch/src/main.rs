//! Example of graphics on the LCD of the Waveshare RP2040-LCD-1.28
//!
//! Draws a red and green line with a blue rectangle.
//! After that it fills the screen line for line, at the end it starts over with
//! another colour, RED, GREEN and BLUE.
#![no_std]
#![no_main]


mod waveshare_rp2040_lcd_1_28;
mod frame_buffer;

use gc9a01a_driver::{Orientation,GC9A01A};
use cortex_m::delay::Delay;
use fugit::RateExtU32;
use panic_halt as _;

use waveshare_rp2040_lcd_1_28::entry;
use waveshare_rp2040_lcd_1_28::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        pac,
        pio::PIOExt,
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};

use embedded_hal::digital::v2::OutputPin;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Rectangle,PrimitiveStyleBuilder, Line, PrimitiveStyle},
    image::{ImageRaw, Image},
};

use frame_buffer::FrameBuffer;

const LCD_WIDTH: u32 = 240;
const LCD_HEIGHT: u32 = 240;
// Define static buffers
const BUFFER_SIZE: usize = (LCD_WIDTH * LCD_HEIGHT * 2) as usize;

/// Main entry point for the application
#[entry]
fn main() -> ! {
    // Take ownership of peripheral instances
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Initialize watchdog
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Initialize clocks and PLLs
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Initialize SIO
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the delay for the first core
    let sys_freq = clocks.system_clock.freq().to_Hz();
    let mut delay = Delay::new(core.SYST, sys_freq);

    let (mut _pio, _sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

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
    
    // Allocate the buffer in main and pass it to the FrameBuffer
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut framebuffer = FrameBuffer::new(&mut buffer, LCD_WIDTH, LCD_HEIGHT);
    framebuffer.clear(Rgb565::BLACK);
    display.show(framebuffer.get_buffer()).unwrap();
    
    // Clear the screen before turning on the backlight
    display.clear(Rgb565::BLACK).unwrap();
    _lcd_bl.set_high().unwrap();
    delay.delay_ms(1000);

    let lcd_zero = Point::zero();
    let lcd_max_corner = Point::new((LCD_WIDTH - 1) as i32, (LCD_HEIGHT - 1) as i32);

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLUE)
        .build();

    Rectangle::with_corners(lcd_zero, lcd_max_corner)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();
    delay.delay_ms(1000);

    let style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();

    Rectangle::with_corners(
        Point::new(1, 1),
        Point::new((LCD_WIDTH - 2) as i32, (LCD_HEIGHT - 2) as i32),
    )
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    Line::new(lcd_zero, lcd_max_corner)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
        .draw(&mut display)
        .unwrap();

    Line::new(
        Point::new(0, (LCD_HEIGHT - 1) as i32),
        Point::new((LCD_WIDTH - 1) as i32, 0),
    )
    .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
    .draw(&mut display)
    .unwrap();

    // Load image data
    let image_data = include_bytes!("rust-logo-240x240.raw");

    let raw_image: ImageRaw<Rgb565> = ImageRaw::new(image_data, LCD_WIDTH);
    let image = Image::new(&raw_image, Point::zero());

    // Draw the image on both frame buffers
    image.draw(&mut framebuffer).unwrap();
    //image.draw(&mut frame_buffer_2).unwrap();
    display.show(framebuffer.get_buffer()).unwrap();
    delay.delay_ms(1000);


    // Infinite colour wheel loop
    /* 
    let mut l: i32 = 0;
    let mut c = Rgb565::RED;
    */
    loop {
        /* 
        Line::new(Point::new(0, l), Point::new((LCD_WIDTH - 1) as i32, l))
            .into_styled(PrimitiveStyle::with_stroke(c, 1))
            .draw(&mut display)
            .unwrap();
        delay.delay_ms(10);
        l += 1;
        if l == LCD_HEIGHT as i32 {
            l = 0;
            c = match c {
                Rgb565::RED => Rgb565::GREEN,
                Rgb565::GREEN => Rgb565::BLUE,
                _ => Rgb565::RED,
            }
        }
        */
        delay.delay_ms(1000);
    }
}
