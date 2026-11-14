use std::error::Error;
use std::fmt;

pub const CANVAS_WIDTH: usize = 100;
pub const CANVAS_HEIGHT: usize = 55;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasError {
    OutOfBounds { x: usize, y: usize },
    NotAscii(u8),
}

impl fmt::Display for CanvasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfBounds { x, y } => write!(
                f,
                "pixel ({x}, {y}) is outside the canvas (width {CANVAS_WIDTH}, height {CANVAS_HEIGHT})"
            ),
            Self::NotAscii(value) => write!(f, "pixel value {value} is not an ASCII value"),
        }
    }
}

impl Error for CanvasError {}

#[derive(Clone)]
pub struct Canvas {
    pixels: [[char; CANVAS_WIDTH]; CANVAS_HEIGHT],
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            pixels: [[' '; CANVAS_WIDTH]; CANVAS_HEIGHT],
        }
    }
    pub fn set_pixel(&mut self, x: usize, y: usize, ascii_value: u8) -> Result<(), CanvasError> {
        if x >= CANVAS_WIDTH || y >= CANVAS_HEIGHT {
            return Err(CanvasError::OutOfBounds { x, y });
        }
        if !ascii_value.is_ascii() {
            return Err(CanvasError::NotAscii(ascii_value));
        }

        self.pixels[y][x] = char::from(ascii_value);
        Ok(())
    }

    pub fn print(&self) {
        print!("{self}");
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.pixels {
            for pixel in row {
                write!(f, "{pixel}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
