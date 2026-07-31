pub const DEAD_COLOR: u32 = 0x2B003D;
pub const ALIVE_COLOR: u32 = 0xFFFF00;

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub background_color: u32,
    pub current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![DEAD_COLOR; width * height],
            background_color: DEAD_COLOR,
            current_color: ALIVE_COLOR,
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn get_color(&self, x: isize, y: isize) -> Option<u32> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);
        (x < self.width && y < self.height).then(|| self.buffer[y * self.width + x])
    }
}
