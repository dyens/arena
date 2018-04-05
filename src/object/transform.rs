use object::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub pos: Vec2,
    pub scale: Vec2,
    pub rot: f64,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vec2::new(0.0, 0.0),
            scale: Vec2::new(0.3, 0.3),
            rot: 0.0,
        }
    }

    pub fn mov(&mut self, v: Vec2) {
        self.pos = self.pos + v;
    }
    pub fn mov_to(&mut self, v: Vec2) {
        self.pos = v;
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
    }
}
