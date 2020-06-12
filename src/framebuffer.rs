use std::fs::File;
use std::io::prelude::*;
use std::ptr;

// Important TODO: Make these variable and set from runtime args
// These are hard-coded for my current monitor.
pub const FRAMEBUFFER_LEN_BYTES: usize = 1920 * 1080 * 4;
//1920x1080 x 32bpp

pub type Pixel = u32;

#[derive(Clone)]
pub struct Display {
    pub width_px: u32,
    pub height_px: u32,
    pub bit_depth: u32,
    pub framebuffer: Box<[u8]>,
}

impl Display {

    /// Intended to zero-out the frame buffer doing an equivalent to memset.
    pub fn zero(&mut self) {
        unsafe {
            ptr::write_bytes(self.framebuffer.as_mut_ptr(), 0, FRAMEBUFFER_LEN_BYTES);
        };
    }

    /// Draw the contents of the buffer to the Linux FrameBuffer /dev/fb0
    pub fn draw(&self) {
        let mut f = File::create("/dev/fb0").unwrap();
        f.write_all(self.framebuffer.as_ref()).unwrap();
        f.sync_all().unwrap();
    }

    /// Set a given pixel to a given color
    pub fn setpx(&mut self, x: usize, y: usize, value: Pixel) {
        // TODO - Is it possible to make this `inline`?
        let offset = x * 4 + (y * 1920 * 4);
        let pixel_as_bytes = value.to_be_bytes();
        self.framebuffer[offset + 0] = pixel_as_bytes[0];
        self.framebuffer[offset + 1] = pixel_as_bytes[1];
        self.framebuffer[offset + 2] = pixel_as_bytes[2];
        self.framebuffer[offset + 3] = pixel_as_bytes[3];
    }

    /// Makes 32-bit pixel value from a RGB tuple
    pub fn fromrgb(&self, red: u8, green: u8, blue: u8) -> Pixel {
        (red as Pixel) << 8 | (green as Pixel) << 16 | (blue as Pixel) << 24
    }
}
