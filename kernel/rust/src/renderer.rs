//	renderer.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

use crate::sync::Mutex;
use crate::limine;
use crate::font;
//pub use crate::cell::SyncCell;

pub const TAB_SIZE: usize = 6;
pub const SPACE_BETWEEN_LINES: u32 = 3;

#[repr(C, packed)]
pub struct ColorRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    none: u8,
}

#[repr(C, packed)]
pub union Color {
    pub rgb: ColorRgb,
    pub uint: u32,
}

impl Copy for ColorRgb {}
impl Clone for ColorRgb {
    fn clone(&self) -> Self { *self }
}
impl Color {
    pub fn new(uint: u32) -> Self {
        Self { uint, }
    }
    pub fn create(red: u8, green: u8, blue: u8) -> Self {
        Self {rgb: ColorRgb {red, green, blue, none: 0}}
    }
}

pub struct FrameBuffer {
    width: usize,
    height: usize,
    address: *mut Color,
    bpp: usize,
}

impl FrameBuffer {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            address: core::ptr::null_mut(),
            bpp: 0,
        }
    }
    pub fn init(&mut self, fb: &limine::request::FramebufferRequest) -> Result<(), ()> {
        if let Some(res) = fb.get_response() {
            if let Some(framebuffer) = res.framebuffers().next() {
                self.bpp = framebuffer.bpp() as usize;
                self.width = framebuffer.width() as usize;
                self.height = framebuffer.height() as usize;
                self.address = framebuffer.addr() as *mut Color;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn bpp(&self) -> usize { self.bpp }
    pub fn address(&self) -> *mut Color {
        self.address
    }
}


pub struct Renderer {
    row: usize,
    line: usize,
    fb: FrameBuffer,
    col: Color,
    space: u32,
}

unsafe impl Sync for Renderer {}
unsafe impl Send for Renderer {}

impl Renderer {
    pub const fn new() -> Self {
        Self {
            row: 0,
            line: 0,
            fb: FrameBuffer::new(),
            col: Color {uint: 0xffffff},
            space: 0,
        }
    }

    pub fn init(&mut self, fb: &limine::request::FramebufferRequest) -> Result<(), ()> {
        self.col = Color::create(255, 255, 255);
        self.row = 0;
        self.line = 0;
        self.space = SPACE_BETWEEN_LINES;
        FrameBuffer::init(&mut self.fb, fb)?;
        Ok(())
    }

    pub fn row(&self) -> usize { self.row }
    pub fn line(&self) -> usize { self.line }
    pub fn fb(&self) -> &FrameBuffer { &self.fb }
    pub fn color(&self) -> Color { Color {uint: unsafe {self.col.uint}} }

    #[inline(always)]
    pub fn set_color(&mut self, color: u32) {self.col.uint = color;}
    pub fn set_color_rgb(&mut self, red: u8, green: u8, blue: u8) {
        self.col.rgb = ColorRgb {red, green, blue, none: 0};
    }
    pub fn space(&mut self) {
        self.row += 1;
        if self.row >= self.fb.width {
            self.row = 0;
            self.line += 1;
        }
    }
    pub fn endl(&mut self) {
        self.line += 1;
        self.row = 0;
    }
    pub fn tab(&mut self) {
        self.row += TAB_SIZE - (self.row % TAB_SIZE);
        if self.row >= self.fb.width {
            self.endl();
        }
    }

    pub fn render(&mut self, c: u8) {
        match c {
            0..31 => {
                match c {
                    b'\n' => {
                        self.endl();
                        return;
                    },
                    b'\t' => {
                        self.tab();
                        return;
                    },
                    _ => {
                        return;
                    }
                }
            },
            _ => {
                let fnt = unsafe { font::FONT.borrow().get_char(c).unwrap() };
                let fb = self.fb();
                let arr = unsafe {fb.address().cast::<u32>().add((self.line * fb.width * (font::FONT_BITS + self.space as usize)) + self.row * font::FONT_BITS)};

                for i in 0..font::FONT_BITS {
                    for ii in 0..font::FONT_BITS {
                        unsafe {arr.add((i as usize * fb.width) + (font::FONT_BITS - ii as usize)).write(self.color().uint * ((fnt[i] as u32 >> ii as u32) & 1))};
                    }
                }
                self.row += 1;
                if self.row >= self.fb.height {
                    self.endl();
                }
            }
        }
    }
    pub fn printstr(&mut self, str: &[u8]) {
        for i in 0..str.len() {
            self.render(str[i]);
        }
    }
}

impl AsRef<Renderer> for Renderer {
    #[inline(always)]
    fn as_ref(&self) -> &Renderer {
        &self
    }
}

impl AsMut<Renderer> for Renderer {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Renderer {
        self
    }
}
pub trait Render {
    /// simple trait for direct data rendering
    /// no allocations fo primitive types
    fn render(&self);
}


//pub static mut RENDERER: Renderer = Renderer::new();
pub static RENDERER: Mutex<Renderer> = Mutex::new(Renderer::new());
//pub static mut RENDERER: SyncCell<Renderer> = SyncCell::new(Renderer::new());

