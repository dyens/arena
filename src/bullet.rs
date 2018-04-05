use piston_window::*;
use graphics::math::Matrix2d;
use gfx_device_gl::Resources;

use object::{Object, Vec2};
use object::component::Component;

const BULLET_W: u32 = 30;
const BULLET_H: u32 = 30;

pub struct Bullet {
    pub bullet: Component,
    pub to_be_removed: bool,
}

impl Bullet {
    pub fn new(sprite: Option<&Texture<Resources>>,
               s_width: f64,
               s_height: f64) -> Self {
        let (width, height) = match sprite {
            Some(ref texture) => texture.get_size(),
            None => (BULLET_W, BULLET_H) // default value
        };
        println!("Bullet texture size: {:?}, {:?}", width, height);
        // scale
        let width = (width as f64) * 0.3;
        let height = (height as f64) * 0.3;

        Bullet {
            bullet: Component::new(sprite,
                                   s_width,
                                   s_height,
                                   width,
                                   height),
            to_be_removed: false,
        }
    }
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
        if self.bullet.on_border() == true {
            self.to_be_removed = true;
        }
    }
    fn render(&mut self, v: Matrix2d, g: &mut G2d) {
        self.bullet.render(v, g);
    }
}
