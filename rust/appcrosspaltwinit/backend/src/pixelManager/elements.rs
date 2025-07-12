use std::any::Any;

use crate::PixelManager::PixelBuffer;

pub type DrawnFunc = fn(&mut PixelBuffer, &dyn BufferElem);

pub trait BufferElem: Any {
    //besoin car rust sait pas que BufferElem pour etre cast en any
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn add(&self, pixelbuffer: &mut PixelBuffer);
    fn click(&self, x: u32, y: u32) -> bool;
}

pub struct Button {
    pub startX: u32,
    pub startY: u32,
    pub width: u32,
    pub height: u32,
    pub border: [u32; 4],
    pub color: [u8; 4],
    pub drawFuncs: Box<[DrawnFunc]>,
}

impl Button {
    pub fn new(
        startX: u32,
        startY: u32,
        width: u32,
        height: u32,
        border: [u32; 4],
        color: [u8; 4],
        drawFuncs: Box<[DrawnFunc]>,
    ) -> Self {
        Self {
            startX,
            startY,
            width,
            height,
            border,
            color,
            drawFuncs,
        }
    }
}

impl BufferElem for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn add(&self, pixelBuffer: &mut PixelBuffer) {
        for draw in self.drawFuncs.iter() {
            draw(pixelBuffer, self);
        }
    }

    fn click(&self, x: u32, y: u32) -> bool {
        x < 100 && y < 100
    }
}

pub struct InputField {
    pub startX: u32,
    pub startY: u32,
    pub width: u32,
    pub height: u32,
    pub color: [u8; 4],
}

impl InputField {
    pub fn new(startX: u32, startY: u32, width: u32, height: u32, color: [u8; 4]) -> Self {
        Self {
            startX,
            startY,
            width,
            height,
            color,
        }
    }
}

impl BufferElem for InputField {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn add(&self, pixelBuffer: &mut PixelBuffer) {
        pixelBuffer.DrawFullRect(
            self.startX,
            self.startY,
            self.width,
            self.height,
            self.color,
        );
    }

    fn click(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}
