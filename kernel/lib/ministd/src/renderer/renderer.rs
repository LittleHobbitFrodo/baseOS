//	renderer.rs (ministd crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

use limine_rs::framebuffer::Framebuffer;
use limine_rs as limine;
use spin::MutexGuard;
use crate::renderer::font;


use crate::renderer::color::Color;

pub const TAB_SIZE: usize = 6;
pub const SPACE_BETWEEN_LINES: u16 = 3;


/// Helper trait for the [`Renderer`]
/// - classic [`core::fmt::Display`] should be prefered
pub trait Render {
    fn render(&self);
    fn render_locked<'l>(&self, guard: &'l mut MutexGuard<DefaultRenderer>);
}

/// Unifying the API for the Renderer and how should it work
/// 
/// all functions that renders something does not return any value
/// - if framebuffer is missing, passes the data to `TODO` function
pub trait Renderer: Render + Send + core::fmt::Write {

    /// Constructs new `Renderer`
    /// - may not work for it has not been initializes
    fn new() -> Self;

    /// Initializes the renderer and makes it work
    /// - returns `Err` if failes
    fn init(&mut self, fb: &limine::request::FramebufferRequest) -> Result<(), ()>;

    /// Returns the current horizontal position of the cursor
    /// - left to right
    fn column(&self) -> usize;

    /// Returns the current vertical position of the cursor
    /// - up to bottom
    fn line(&self) -> usize;

    /// Returns the position of the cursor as `(x, y)`
    fn position(&self) -> (usize, usize);

    /// Returns reference to raw framebuffer
    fn fb(&self) -> &FrameBuffer;

    /// Clears the display (sets all pixels to default background colors)
    fn clear(&mut self);

    /// Sets the vertical position of the cursor to the specified value
    /// - returns `Err` if `line` is out of bounds of the framebuffer
    fn set_line(&mut self, line: usize) -> Result<(), ()>;

    /// Sets the horizontal position of the cursor to the specified value
    fn set_column(&mut self, column: usize) -> Result<(), ()>;

    /// Sets horizontal and verical position of the cursor
    /// - returns `Err` if `line` or `row` is out of bounds of the framebuffer
    fn set_pos(&mut self, line: usize, row: usize) -> Result<(), ()>;

    /// Renders one character at the cursor position while moving the cursor to right
    /// - potentionally moves the cursor one line below
    /// - alignes the cursor to `TAB_SIZE` constant on tab character
    fn render(&mut self, c: u8);

    /// Renders the string at the cursor position
    fn print(&mut self, str: &[u8]);

    /// Renders the string at the cursor position and breaks the line
    fn println(&mut self, str: &[u8]);

    /// Breaks the line
    fn endl(&mut self);

    /// Prints the tab character (aligns the horizontal position of the cursor to the `TAB_SIZE` constant)
    fn tab(&mut self);


}


//pub static mut RENDERER: Renderer = Renderer::new();
pub static RENDERER: spin::Mutex<DefaultRenderer> = spin::Mutex::new(DefaultRenderer::new());
//pub static mut RENDERER: SyncCell<Renderer> = SyncCell::new(Renderer::new());

pub struct DefaultRenderer {
    row: usize,
    line: usize,
    fb: FrameBuffer,
    col: Color,
    space: u16,
    initialized: bool,
}

//unsafe impl Sync for DefaultRenderer {}
unsafe impl Send for DefaultRenderer {}

impl DefaultRenderer {
    pub const fn new() -> Self {
        Self {
            row: 0,
            line: 0,
            fb: FrameBuffer::new(),
            col: Color::new(0xffffff),
            space: 0,
            initialized: false,
        }
    }

    fn init(&mut self, fb: &limine::request::FramebufferRequest) -> Result<(), ()> {
        self.col = Color::new_rgb(255, 255, 255);
        self.row = 0;
        self.line = 0;
        self.space = SPACE_BETWEEN_LINES;
        if let Ok(_) = FrameBuffer::init(&mut self.fb, fb) {
            self.initialized = true;
            Ok(())
        } else {
            self.initialized = false;
            Err(())
        }
    }

    #[inline(always)] pub fn column(&self) -> usize { self.row }
    #[inline(always)] pub fn line(&self) -> usize { self.line }
    #[inline(always)] pub fn fb(&self) -> &FrameBuffer { &self.fb }
    #[inline(always)] pub fn color(&self) -> Color { self.col }
    #[inline(always)] pub fn set_color(&mut self, color: u32) {self.col.set_int(color);}



    fn space(&mut self) {
        self.row += 1;
        if self.row >= self.fb.width {
            self.row = 0;
            self.line += 1;
        }
    }
    #[inline]
    pub fn endl(&mut self) {
        self.line += 1;
        self.row = 0;
    }
    #[inline]
    pub fn tab(&mut self) {
        self.row += TAB_SIZE - (self.row % TAB_SIZE);
        if self.row >= self.fb.width {
            self.endl();
        }
    }

    pub fn clear(&mut self) {
        for i in unsafe { core::slice::from_raw_parts_mut(self.fb.address, self.fb.width*self.fb.height) } {
            i.set_int(0);       //  black
        }
    }

    pub fn set_pos(&mut self, line: usize, row: usize) -> Result<(), ()> {
        if line < self.fb.height && row < self.fb.width {
            self.line = line;
            self.row = row;
            Ok(())
        } else {
            Err(())
        }
    }

    fn rend(&mut self, c: u8) {
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
                    _ => return,
                }
            },
            _ => {
                let fnt = match crate::renderer::font::FONT.as_ref().get_char(c) {
                    Some(f) => f,
                    None => &font::ERR_CHAR,
                };
                let fb = self.fb();
                let arr = unsafe {fb.address().cast::<u32>().add((self.line * fb.width * (font::FONT_BITS + self.space as usize)) + self.row * font::FONT_BITS)};

                for i in 0..font::FONT_BITS {
                    for ii in 0..font::FONT_BITS {
                        unsafe {arr.add((i as usize * fb.width) + (font::FONT_BITS - ii as usize)).write(self.color().as_int() * ((fnt[i] as u32 >> ii as u32) & 1))};
                    }
                }
                self.row += 1;
                if self.row >= self.fb.height {
                    self.endl();
                }
            }
        }
    }

    #[inline(always)]
    pub fn render(&mut self, c: u8) {
        if self.initialized {
            self.rend(c);
        }
    }

    #[inline(always)]
    pub fn print(&mut self, str: &[u8]) {
        if self.initialized {
            for i in 0..str.len() {
                self.rend(str[i]);
            }
        }
    }
    pub fn println(&mut self, str: &[u8]) {
        if self.initialized {
            for i in 0..str.len() {
                self.rend(str[i]);
            }
            self.endl();
        }
    }
}

impl AsRef<DefaultRenderer> for DefaultRenderer {
    #[inline(always)]
    fn as_ref(&self) -> &DefaultRenderer {
        &self
    }
}

impl AsMut<DefaultRenderer> for DefaultRenderer {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut DefaultRenderer {
        self
    }
}

pub struct FrameBuffer {
    width: usize,
    height: usize,
    address: *mut Color,
    bpp: usize,
    initialized: bool
}

impl FrameBuffer {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            address: core::ptr::null_mut(),
            bpp: 0,
            initialized: false
        }
    }
    pub fn init(&mut self, fb: &limine::request::FramebufferRequest) -> Result<(), ()> {
        if let Some(res) = fb.get_response() {
            if let Some(framebuffer) = res.framebuffers().next() {
                self.bpp = framebuffer.bpp() as usize;
                self.width = framebuffer.width() as usize;
                self.height = framebuffer.height() as usize;
                self.address = framebuffer.addr() as *mut Color;
                self.initialized = true;
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

/*impl TryFrom<limine::request::FramebufferRequest> for FrameBuffer {
    type Error = ();
    fn try_from(value: limine::request::FramebufferRequest) -> Result<Self, ()> {
        if let Some(r) = value.get_response() {
            if let Some(fb) = r.framebuffers().nth(0) {
                Self {
                    width: fb.width(),
                    height: fb.height(),

                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
        
    }
}*/


#[inline(always)]
pub fn init() -> Result<(), ()> {
    RENDERER.lock().init(&bootloader::FRAMEBUFFER)
}


impl core::fmt::Write for DefaultRenderer {
    #[inline]
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.render(c as u8);
        Ok(())
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }

}

impl DefaultRenderer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}