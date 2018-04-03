use object::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vec2,
    pub scale: Vec2,
    pub rot: f64,

    s_widht: f64,
    s_height: f64,
}

impl Transform {
    pub fn new(s_widht: f64, s_height: f64) -> Self {
        Transform {
            pos: Vec2::new(0.0, 0.0),
            scale: Vec2::new(0.75, 0.75),
            rot: 0.0,
            s_widht: s_widht,
            s_height: s_height,
        }
    }

    fn correct_pos(&mut self) {
        if self.pos.x > self.s_widht {
            self.pos.x = self.s_widht;
        }
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
        }
        if self.pos.y > self.s_height {
            self.pos.y = self.s_height;
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        }
    }

    pub fn on_border(&self) -> bool {
        if self.pos.x == 0.0 ||
            self.pos.x == self.s_widht ||
            self.pos.y == 0.0 ||
            self.pos.y == self.s_height {
                return true;
            }
        false
    }

    pub fn mov(&mut self, v: Vec2) {
        self.pos = self.pos + v;
        self.correct_pos();
    }
    pub fn mov_to(&mut self, v: Vec2) {
        self.pos = v;
        self.correct_pos();
    }
    pub fn rot(&mut self, r: f64) {
        self.rot += r;
    }
    pub fn rot_to(&mut self, r: f64) {
        self.rot = r;
    }
    pub fn fwd(&mut self, d: f64) {
        self.pos.x += d * (-self.rot.sin());
        self.pos.y += d * self.rot.cos();
        self.correct_pos();
    }
}
