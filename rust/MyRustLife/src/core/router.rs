pub struct Router {
    segments: Vec<String>,
    index: u8,
}

impl Router {
    fn parse_segments(path: &str) -> Vec<String> {
        path.trim_matches('/')
            .split('/')
            .map(|s| s.to_string())
            .collect()
    }

    pub fn new(path: &str) -> Self {
        Self {
            segments: Self::parse_segments(path),
            index: 0,
        }
    }

    pub fn path(&self) -> String {
        format!("/{}", self.segments.join("/"))
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn current(&self) -> Option<&str> {
        self.segments.get(self.index as usize).map(|s| s.as_str())
    }

    pub fn enter(&mut self) {
        self.index += 1;
    }

    pub fn exit(&mut self) {
        if (self.index > 0) {
            self.index -= 1;
        }
    }

    // pub fn navigate(&mut self, path: &str, index: u8) {
    //     self.segments = Self::parse_segments(path);
    //     self.index = index;
    // }

    pub fn push(&mut self, segment: &str) {
        // Remove everything after current index first
        self.segments.truncate(self.index as usize + 1);
        self.segments.push(segment.to_string());
        // self.enter();
    }

    // pub fn pop(&mut self) {
    //     if self.index > 0 {
    //         self.segments.truncate(self.index as usize);
    //         self.exit();
    //     }
    // }

    // fn next(&mut self) -> Option<&str> {
    //     let seg = self.segments.get(self.index as usize)?;
    //     self.index += 1;
    //     Some(seg)
    // }
}
