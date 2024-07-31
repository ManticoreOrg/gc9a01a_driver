use core::convert::Infallible;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

pub struct FrameBuffer<'a> {
    buffer: &'a mut [u8],
    width: u32,
    height: u32,
}

impl<'a> FrameBuffer<'a> {
    pub fn new(buffer: &'a mut [u8], width: u32, height: u32) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn get_buffer(&self) -> &[u8] {
        self.buffer
    }

    pub fn clear(&mut self, color: Rgb565) {
        let raw_color = color.into_storage();
        for chunk in self.buffer.chunks_exact_mut(2) {
            chunk[0] = (raw_color >> 8) as u8;
            chunk[1] = raw_color as u8;
        }
    }

    pub fn copy_region(
        &mut self,
        src_buffer: &[u8],
        src_top_left: Point,
        src_size: Size,
        dest_top_left: Point,
    ) {
        for row in 0..src_size.height as usize {
            let src_row_start = (src_top_left.y as usize + row) * self.width as usize * 2
                + src_top_left.x as usize * 2;
            let src_row_end = src_row_start + src_size.width as usize * 2;

            let dest_row_start = (dest_top_left.y as usize + row) * self.width as usize * 2
                + dest_top_left.x as usize * 2;
            let dest_row_end = dest_row_start + src_size.width as usize * 2;

            self.buffer[dest_row_start..dest_row_end]
                .copy_from_slice(&src_buffer[src_row_start..src_row_end]);
        }
    }
}

impl<'a> DrawTarget for FrameBuffer<'a> {
    type Color = Rgb565;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if coord.x >= 0
                && coord.x < self.width as i32
                && coord.y >= 0
                && coord.y < self.height as i32
            {
                let index = ((coord.y as u32 * self.width + coord.x as u32) * 2) as usize;
                let raw_color = color.into_storage();
                self.buffer[index] = (raw_color >> 8) as u8;
                self.buffer[index + 1] = raw_color as u8;
            }
        }
        Ok(())
    }
}

impl<'a> OriginDimensions for FrameBuffer<'a> {
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }
}
