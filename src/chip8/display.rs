pub struct Display {
    pub width: usize,
    pub height: usize,
    pub vram_updated: bool
}

impl Display {
    pub fn new(width: usize, height: usize) -> Display {
        Display {
            width: width,
            height: height,
            vram_updated: false
        }
    }
}