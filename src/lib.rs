#![no_std]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::spi;
use embedded_hal::digital::v2::OutputPin;

/// Enumeration of instructions for the GC9A01A display.
pub enum Instruction {
    NOP      = 0x00,  // No Operation
    SWRESET  = 0x01,  // Software Reset
    RDDID    = 0x04,  // Read Display Identification Information
    RDDST    = 0x09,  // Read Display Status
    SLPIN    = 0x10,  // Enter Sleep Mode
    SLPOUT   = 0x11,  // Sleep Out Mode
    PTLON    = 0x12,  // Partial Mode ON
    NORON    = 0x13,  // Normal Display Mode ON
    INVOFF   = 0x20,  // Display Inversion OFF
    INVON    = 0x21,  // Display Inversion ON
    DISPOFF  = 0x28,  // Display OFF
    DISPON   = 0x29,  // Display ON
    CASET    = 0x2A,  // Column Address Set
    RASET    = 0x2B,  // Row Address Set
    RAMWR    = 0x2C,  // Memory Write
    RAMRD    = 0x2E,  // Memory Read
    PTLAR    = 0x30,  // Partial Area
    COLMOD   = 0x3A,  // Pixel Format Set
    MADCTL   = 0x36,  // Memory Access Control
    FRMCTR1  = 0xB1,  // Frame Rate Control (In normal mode/Full colors)
    FRMCTR2  = 0xB2,  // Frame Rate Control (In idle mode/8 colors)
    FRMCTR3  = 0xB3,  // Frame Rate Control (In partial mode/full colors)
    INVCTR   = 0xB4,  // Display Inversion Control
    DISSET5  = 0xB6,  // Display Function Control
    PWCTR1   = 0xC0,  // Power Control 1
    PWCTR2   = 0xC1,  // Power Control 2
    PWCTR3   = 0xC2,  // Power Control 3
    PWCTR4   = 0xC3,  // Power Control 4
    PWCTR5   = 0xC4,  // Power Control 5
    VMCTR1   = 0xC5,  // VCOM Control 1
    RDID1    = 0xDA,  // Read ID1
    RDID2    = 0xDB,  // Read ID2
    RDID3    = 0xDC,  // Read ID3
    RDID4    = 0xDD,  // Read ID4
    PWCTR6   = 0xFC,  // Power Control 6
    GMCTRP1  = 0xE0,  // Positive Gamma Correction
    GMCTRN1  = 0xE1   // Negative Gamma Correction
}

/// Driver for the GC9A01A display.
pub struct GC9A01A<SPI, DC, CS, RST>
where
    SPI: spi::Write<u8>,
    DC: OutputPin,
    CS: OutputPin,
    RST: OutputPin,
{
    /// SPI interface.
    spi: SPI,

    /// Data/command pin.
    dc: DC,

    /// Chip select pin.
    cs: CS,

    /// Reset pin.
    rst: RST,

    /// Whether the display is RGB (true) or BGR (false).
    rgb: bool,

    /// Global image offset.
    dx: u16,
    dy: u16,
    width: u32,
    height: u32,
}

/// Display orientation.
#[derive(Clone, Copy)]
pub enum Orientation {
    Portrait = 0x00,
    Landscape = 0x60,
    PortraitSwapped = 0xC0,
    LandscapeSwapped = 0xA0,
}

impl<SPI, DC, CS, RST> GC9A01A<SPI, DC, CS, RST>
where
    SPI: spi::Write<u8>,
    DC: OutputPin,
    CS: OutputPin,
    RST: OutputPin,
{
    /// Creates a new driver instance that uses hardware SPI.
    ///
    /// # Arguments
    ///
    /// * `spi` - SPI interface.
    /// * `dc` - Data/command pin.
    /// * `cs` - Chip select pin.
    /// * `rst` - Reset pin.
    /// * `rgb` - Whether the display is RGB (true) or BGR (false).
    /// * `width` - Width of the display.
    /// * `height` - Height of the display.
    pub fn new(
        spi: SPI,
        dc: DC,
        cs: CS,
        rst: RST,
        rgb: bool,
        width: u32,
        height: u32,
    ) -> Self {
        GC9A01A {
            spi,
            dc,
            cs,
            rst,
            rgb,
            dx: 0,
            dy: 0,
            width,
            height,
        }
    }

    /// Initializes the display.
    ///
    /// This function initializes the display by sending a sequence of commands and settings
    /// to configure the display properly. It includes a hardware reset and various configuration
    /// commands.
    ///
    /// # Arguments
    ///
    /// * `delay` - Delay provider.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn init<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>
    where
        DELAY: DelayMs<u8>,
    {
        self.hard_reset(delay)?;
        self.write_command(0xEF as u8, &[])?;               // Inter Register Enable 2 (0xEF)
        self.write_command(0xEB as u8, &[0x14])?;           // Not found in the PDF
        self.write_command(0xFE, &[])?;                     // Inter Register Enable 1 (0xFE)
        self.write_command(0xEF, &[])?;                     // Inter Register Enable 2 (0xEF)
        self.write_command(0xEB, &[0x14])?;                 // Not found in the PDF
        self.write_command(0x84, &[0x40])?;                 // Not found in the PDF
        self.write_command(0x85, &[0xFF])?;                 // Not found in the PDF
        self.write_command(0x86, &[0xFF])?;                 // Not found in the PDF
        self.write_command(0x87, &[0xFF])?;                 // Not found in the PDF
        self.write_command(0x88, &[0x0A])?;                 // Not found in the PDF
        self.write_command(0x89, &[0x21])?;                 // Not found in the PDF
        self.write_command(0x8A, &[0x00])?;                 // Not found in the PDF
        self.write_command(0x8B, &[0x80])?;                 // Not found in the PDF
        self.write_command(0x8C, &[0x01])?;                 // Not found in the PDF
        self.write_command(0x8D, &[0x01])?;                 // Not found in the PDF
        self.write_command(0x8E, &[0xFF])?;                 // Not found in the PDF
        self.write_command(0x8F, &[0xFF])?;                 // Not found in the PDF
        self.write_command(0xB6, &[0x00, 0x20])?;           // Display Function Control (0xB6)
        self.write_command(0x36, &[0x98])?;                 // Memory Access Control (MADCTL)
        self.write_command(0x3A, &[0x05])?;                 // Pixel Format Set (COLMOD)
        self.write_command(0x90, &[0x08, 0x08, 0x08, 0x08])?; // Not found in the PDF
        self.write_command(0xBD, &[0x06])?;                 // Not found in the PDF
        self.write_command(0xBC, &[0x00])?;                 // Not found in the PDF
        self.write_command(0xFF, &[0x60, 0x01, 0x04])?;     // Not found in the PDF
        self.write_command(0xC3, &[0x13])?;                 // Power Control 4 (PWCTR4)
        self.write_command(0xC4, &[0x13])?;                 // Power Control 5 (PWCTR5)
        self.write_command(0xC9, &[0x22])?;                 // Not found in the PDF
        self.write_command(0xBE, &[0x11])?;                 // Not found in the PDF
        self.write_command(0xE1, &[0x10, 0x0E])?;           // Negative Gamma Correction (GMCTRN1)
        self.write_command(0xDF, &[0x21, 0x0C, 0x02])?;     // Not found in the PDF
        self.write_command(0xF0, &[0x45, 0x09, 0x08, 0x08, 0x26, 0x2A])?; // Positive Gamma Correction (GMCTRP1)
        self.write_command(0xF1, &[0x43, 0x70, 0x72, 0x36, 0x37, 0x6F])?; // SET_GAMMA2 (0xF1)
        self.write_command(0xF2, &[0x45, 0x09, 0x08, 0x08, 0x26, 0x2A])?; // Not found in the PDF
        self.write_command(0xF3, &[0x43, 0x70, 0x72, 0x36, 0x37, 0x6F])?; // Not found in the PDF
        self.write_command(0xED, &[0x1B, 0x0B])?;           // Not found in the PDF
        self.write_command(0xAE, &[0x77])?;                 // Not found in the PDF
        self.write_command(0xCD, &[0x63])?;                 // Not found in the PDF
        self.write_command(0x70, &[0x07, 0x07, 0x04, 0x0E, 0x0F, 0x09, 0x07, 0x08, 0x03])?; // Not found in the PDF
        self.write_command(0xE8, &[0x34])?;                 // Frame Rate Control (FRMCTR1)
        self.write_command(0x62, &[0x18, 0x0D, 0x71, 0xED, 0x70, 0x70, 0x18, 0x0F, 0x71, 0xEF, 0x70, 0x70])?; // Not found in the PDF
        self.write_command(0x63, &[0x18, 0x11, 0x71, 0xF1, 0x70, 0x70, 0x18, 0x13, 0x71, 0xF3, 0x70, 0x70])?; // Not found in the PDF
        self.write_command(0x64, &[0x28, 0x29, 0xF1, 0x01, 0xF1, 0x00, 0x07])?; // Not found in the PDF
        self.write_command(0x66, &[0x3C, 0x00, 0xCD, 0x67, 0x45, 0x45, 0x10, 0x00, 0x00, 0x00])?; // Not found in the PDF
        self.write_command(0x67, &[0x00, 0x3C, 0x00, 0x00, 0x00, 0x01, 0x54, 0x10, 0x32, 0x98])?; // Not found in the PDF
        self.write_command(0x74, &[0x10, 0x85, 0x80, 0x00, 0x00, 0x4E, 0x00])?; // Not found in the PDF
        self.write_command(0x98, &[0x3E, 0x07])?;           // Not found in the PDF
        self.write_command(0x35, &[])?;                     // Not found in the PDF
        self.write_command(0x21, &[])?;                     // Display Inversion ON (INVON)
        self.write_command(0x11, &[])?;                     // Sleep Out Mode (SLPOUT)
        self.write_command(0x29, &[])?;                     // Display ON (DISPON)

        delay.delay_ms(200);

        Ok(())
    }

    /// Performs a hard reset of the display.
    ///
    /// This function performs a hard reset by toggling the reset pin, ensuring the display
    /// is in a known state before initialization.
    ///
    /// # Arguments
    ///
    /// * `delay` - Delay provider.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn hard_reset<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), ()>
    where
        DELAY: DelayMs<u8>,
    {
        self.rst.set_high().map_err(|_| ())?;
        delay.delay_ms(10);
        self.rst.set_low().map_err(|_| ())?;
        delay.delay_ms(10);
        self.rst.set_high().map_err(|_| ())?;
        delay.delay_ms(10);

        Ok(())
    }

    /// Writes a command to the display.
    ///
    /// This function sends a command followed by optional parameters to the display.
    ///
    /// # Arguments
    ///
    /// * `command` - Command to write.
    /// * `params` - Parameters for the command.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    fn write_command(&mut self, command: u8, params: &[u8]) -> Result<(), ()> {
        self.cs.set_high().map_err(|_| ())?;
        self.dc.set_low().map_err(|_| ())?;
        self.cs.set_low().map_err(|_| ())?;
        self.spi.write(&[command]).map_err(|_| ())?;
        if !params.is_empty() {
            self.start_data()?;
            self.write_data(params)?;
        }
        self.cs.set_high().map_err(|_| ())?;
        Ok(())
    }

    /// Starts data transmission.
    ///
    /// Sets the data/command pin to indicate data mode for subsequent transmissions.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    fn start_data(&mut self) -> Result<(), ()> {
        self.dc.set_high().map_err(|_| ())
    }

    /// Writes data to the display.
    ///
    /// This function writes data to the display through the SPI interface.
    ///
    /// # Arguments
    ///
    /// * `data` - Data to write.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    fn write_data(&mut self, data: &[u8]) -> Result<(), ()> {
        self.cs.set_high().map_err(|_| ())?;
        self.dc.set_high().map_err(|_| ())?;
        self.cs.set_low().map_err(|_| ())?;
        self.spi.write(data).map_err(|_| ())?;
        self.cs.set_high().map_err(|_| ())?;
        Ok(())
    }

    /// Writes a data word to the display.
    ///
    /// This function writes a 16-bit word to the display.
    ///
    /// # Arguments
    ///
    /// * `value` - Data word to write.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    fn write_word(&mut self, value: u16) -> Result<(), ()> {
        self.write_data(&value.to_be_bytes())
    }

    /// Writes buffered data words to the display.
    ///
    /// This function writes an iterator of 16-bit words to the display in buffered mode.
    ///
    /// # Arguments
    ///
    /// * `words` - Data words to write.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    fn write_words_buffered(&mut self, words: impl IntoIterator<Item = u16>) -> Result<(), ()> {
        let mut buffer = [0; 32];
        let mut index = 0;
        for word in words {
            let as_bytes = word.to_be_bytes();
            buffer[index] = as_bytes[0];
            buffer[index + 1] = as_bytes[1];
            index += 2;
            if index >= buffer.len() {
                self.write_data(&buffer)?;
                index = 0;
            }
        }
        self.write_data(&buffer[0..index])
    }

    /// Sets the orientation of the display.
    ///
    /// This function sets the display orientation to one of the predefined modes.
    ///
    /// # Arguments
    ///
    /// * `orientation` - Orientation to set.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn set_orientation(&mut self, orientation: &Orientation) -> Result<(), ()> {
        if self.rgb {
            self.write_command(Instruction::MADCTL as u8, &[*orientation as u8])?;
        } else {
            self.write_command(Instruction::MADCTL as u8, &[*orientation as u8 | 0x08])?;
        }
        Ok(())
    }

    /// Sets the global offset of the displayed image.
    ///
    /// # Arguments
    ///
    /// * `dx` - Horizontal offset.
    /// * `dy` - Vertical offset.
    pub fn set_offset(&mut self, dx: u16, dy: u16) {
        self.dx = dx;
        self.dy = dy;
    }

    /// Sets the address window for the display.
    ///
    /// This function sets the address window for subsequent drawing commands.
    ///
    /// # Arguments
    ///
    /// * `sx` - Start x-coordinate.
    /// * `sy` - Start y-coordinate.
    /// * `ex` - End x-coordinate.
    /// * `ey` - End y-coordinate.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn set_address_window(&mut self, sx: u16, sy: u16, ex: u16, ey: u16) -> Result<(), ()> {
        self.write_command(Instruction::CASET as u8, &[])?;
        self.start_data()?;
        self.write_word(sx + self.dx)?;
        self.write_word(ex + self.dx)?;
        self.write_command(Instruction::RASET as u8, &[])?;
        self.start_data()?;
        self.write_word(sy + self.dy)?;
        self.write_word(ey + self.dy)
    }

    /// Sets a pixel color at the given coordinates.
    ///
    /// This function sets the color of a single pixel at the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - X-coordinate.
    /// * `y` - Y-coordinate.
    /// * `color` - Color of the pixel.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn set_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<(), ()> {
        self.set_address_window(x, y, x, y)?;
        self.write_command(Instruction::RAMWR as u8, &[])?;
        self.start_data()?;
        self.write_word(color)
    }

    /// Writes pixel colors sequentially into the current drawing window.
    ///
    /// This function writes a sequence of pixel colors into the current drawing window.
    ///
    /// # Arguments
    ///
    /// * `colors` - Pixel colors to write.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn write_pixels<P: IntoIterator<Item = u16>>(&mut self, colors: P) -> Result<(), ()> {
        self.write_command(Instruction::RAMWR as u8, &[])?;
        self.start_data()?;
        for color in colors {
            self.write_word(color)?;
        }
        Ok(())
    }

    /// Writes buffered pixel colors sequentially into the current drawing window.
    ///
    /// This function writes a sequence of pixel colors into the current drawing window in buffered mode.
    ///
    /// # Arguments
    ///
    /// * `colors` - Pixel colors to write.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn write_pixels_buffered<P: IntoIterator<Item = u16>>(
        &mut self,
        colors: P,
    ) -> Result<(), ()> {
        self.write_command(Instruction::RAMWR as u8, &[])?;
        self.start_data()?;
        self.write_words_buffered(colors)
    }

    /// Sets pixel colors at the given drawing window.
    ///
    /// This function sets the colors of pixels in a specified rectangular region.
    ///
    /// # Arguments
    ///
    /// * `sx` - Start x-coordinate.
    /// * `sy` - Start y-coordinate.
    /// * `ex` - End x-coordinate.
    /// * `ey` - End y-coordinate.
    /// * `colors` - Pixel colors to write.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn set_pixels<P: IntoIterator<Item = u16>>(
        &mut self,
        sx: u16,
        sy: u16,
        ex: u16,
        ey: u16,
        colors: P,
    ) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey)?;
        self.write_pixels(colors)
    }

    /// Sets buffered pixel colors at the given drawing window.
    ///
    /// This function sets the colors of pixels in a specified rectangular region in buffered mode.
    ///
    /// # Arguments
    ///
    /// * `sx` - Start x-coordinate.
    /// * `sy` - Start y-coordinate.
    /// * `ex` - End x-coordinate.
    /// * `ey` - End y-coordinate.
    /// * `colors` - Pixel colors to write.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn set_pixels_buffered<P: IntoIterator<Item = u16>>(
        &mut self,
        sx: u16,
        sy: u16,
        ex: u16,
        ey: u16,
        colors: P,
    ) -> Result<(), ()> {
        self.set_address_window(sx, sy, ex, ey)?;
        self.write_pixels_buffered(colors)
    }

    /// Draws an image from a slice of RGB565 data.
    ///
    /// This function draws an image from a slice of pixel data in RGB565 format.
    /// It assumes the image dimensions match the display dimensions.
    ///
    /// # Arguments
    ///
    /// * `image_data` - Image data to draw.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn draw_image(&mut self, image_data: &[u8]) -> Result<(), ()> {
        let width = self.width as u16;
        let height = self.height as u16;

        self.set_address_window(0, 0, width - 1, height - 1)?;
        self.write_command(Instruction::RAMWR as u8, &[])?;
        self.start_data()?;
        
        for chunk in image_data.chunks(32) {
            self.write_data(chunk)?;
        }
        
        Ok(())
    }

    /// Displays the provided buffer on the screen.
    ///
    /// This function writes the entire buffer to the display, assuming the buffer
    /// contains pixel data for the full display area.
    ///
    /// # Arguments
    ///
    /// * `buffer` - Buffer to display.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success or failure.
    pub fn show(&mut self, buffer: &[u8]) -> Result<(), ()> {
        self.write_command(Instruction::CASET as u8, &[])?;
        self.write_data(&[0x00, 0x00, 0x00, 0xEF])?;

        self.write_command(Instruction::RASET as u8, &[])?;
        self.write_data(&[0x00, 0x00, 0x00, 0xEF])?;

        self.write_command(Instruction::RAMWR as u8, &[])?;

        self.cs.set_high().map_err(|_| ())?;
        self.dc.set_high().map_err(|_| ())?;
        self.cs.set_low().map_err(|_| ())?;
        self.spi.write(buffer).map_err(|_| ())?;
        self.cs.set_high().map_err(|_| ())?;
        
        Ok(())
    }

    /// Updates only the specified region of the display with the provided buffer.
    ///
    /// This function updates a specified rectangular region of the display with the pixel data 
    /// provided in the buffer. It calculates the necessary offsets and addresses to update only 
    /// the designated area, ensuring efficient display refresh.
    ///
    /// # Arguments
    ///
    /// * `buffer` - A slice of bytes representing the pixel data in RGB565 format.
    /// * `top_left_x` - The x-coordinate of the top-left corner of the region to update.
    /// * `top_left_y` - The y-coordinate of the top-left corner of the region to update.
    /// * `width` - The width of the region to update.
    /// * `height` - The height of the region to update.
    ///
    /// # Returns
    ///
    /// `Result<(), ()>` indicating success (`Ok`) or failure (`Err`).
    pub fn show_region(&mut self, buffer: &[u8], top_left_x: u16, top_left_y: u16, width: u16, height: u16) -> Result<(), ()> {
        let sx = top_left_x as u16;  // Start x-coordinate
        let sy = top_left_y as u16;  // Start y-coordinate
        let ex = (top_left_x + width - 1) as u16;  // End x-coordinate
        let ey = (top_left_y + height - 1) as u16; // End y-coordinate

        // Calculate the buffer offset for the region
        let buffer_width = self.width as usize;  // Width of the buffer
        let bytes_per_pixel = 2;  // Number of bytes per pixel in RGB565 format

        // Set the address window for the region to be updated
        self.set_address_window(sx, sy, ex, ey)?;
        
        // Send the command to write to RAM
        self.write_command(Instruction::RAMWR as u8, &[])?;
        
        // Start data transmission
        self.start_data()?;

        // Iterate over each row in the region
        for y in sy..=ey {
            let start_index = ((y as usize) * buffer_width + (sx as usize)) * bytes_per_pixel;
            let end_index = start_index + (width as usize) * bytes_per_pixel;

            // Write data to the display in chunks of 32 bytes
            for chunk in buffer[start_index..end_index].chunks(32) {
                self.write_data(chunk)?;
            }
        }

        Ok(())
    }
}
