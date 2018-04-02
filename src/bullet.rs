use piston_window::*;
use graphics::math::Matrix2d;

use object::{Object, Vec2};
use object::component::Component;

pub struct Bullet {
    pub bullet: Component,
    pub to_be_removed: bool,
}

impl Object for Bullet {
    fn mov(&mut self, pos: Vec2) {
        self.bullet.trans.mov(pos);
    }
    fn mov_to(&mut self, pos: Vec2) {
        self.bullet.trans.mov_to(pos);
    }
    fn rot(&mut self, r: f64) {
        self.bullet.trans.rot(r);
    }
    fn rot_to(&mut self, r: f64) {
        self.bullet.trans.rot_to(r);
    }
    fn fwd(&mut self, d: f64) {
        self.bullet.trans.fwd(d);
    }
    fn update(&mut self, dt: f64) {
        let bullet_speed = 200.0;
        self.fwd(bullet_speed * dt);
    }
    fn render(&mut self, v: Matrix2d, g: &mut G2d) {
        self.bullet.render(v, g);
    }
}
