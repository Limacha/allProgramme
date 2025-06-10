use std::any::Any;

pub trait BufferElem: Any {
    //besoin car rust sait pas que BufferElem pour etre cast en any
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn add(&self);
    fn click(&self, x: u32, y: u32) -> bool;
}

pub struct Button {
    pub width: u32,
    pub height: u32,
}

impl Button {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl BufferElem for Button {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn add(&self) {
        println!("Button ajouté avec taille {}x{}", self.width, self.height);
    }

    fn click(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}

pub struct InputField {
    pub width: u32,
    pub height: u32,
}

impl InputField {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl BufferElem for InputField {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn add(&self) {
        println!("Button ajouté avec taille {}x{}", self.width, self.height);
    }

    fn click(&self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }
}
