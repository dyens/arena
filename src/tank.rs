use piston_window::*;
use gfx_device_gl::Resources;
use std::f64::consts::PI;
use graphics::math::Matrix2d;
use nc::query::PointQuery;
use na::Isometry2;

use object::Pnt2;
use object::{Object, Cuboid2f, Vec2};
use object::component::Component;
use bullet::Bullet;

pub struct Tank {
    pub tank: Component,
    pub health: u32,
    pub is_destroyed: bool,
    collider: Cuboid2f,
    point_to: Vec2,
}

impl Tank {
    pub fn new() -> Tank {
        Tank {
            tank: Component::new(),
            health: 100,
            is_destroyed: false,
            collider: Cuboid2f::new(Vec2::new(38.0, 65.0)),
            point_to: Vec2::new(0.0, 0.0),
        }
    }

    pub fn collides(&mut self, b: &Bullet) -> bool {
        let bpnt = Pnt2::new(b.bullet.trans.pos.x, b.bullet.trans.pos.y);
        self.collider
            .contains_point(&Isometry2::new(self.tank.trans.pos, 0.0), &bpnt)
    }

    pub fn shooted(&mut self) {
        if self.is_destroyed == false {
            self.health -= 10;
        }
        if self.health <= 0 {
            self.is_destroyed = true
        }
    }

    pub fn fire(&self, sprite: Texture<Resources>) -> Bullet {
        let mut bul = Bullet {
            bullet: Component::new(),
            to_be_removed: false,
        };
        bul.mov_to(self.tank.trans.pos);
        bul.rot_to(self.tank.trans.rot);
        bul.fwd(125.0);
        bul.bullet.set_sprite(sprite);
        bul
    }
}
impl Object for Tank {
    fn mov(&mut self, pos: Vec2) {
        self.tank.trans.mov(pos);
    }
    fn mov_to(&mut self, pos: Vec2) {
        self.tank.trans.mov_to(pos);
    }
    fn rot(&mut self, r: f64) {
        self.tank.trans.rot(r);
    }
    fn rot_to(&mut self, r: f64) {
        self.tank.trans.rot_to(r);
    }
    fn fwd(&mut self, d: f64) {
        self.tank.trans.fwd(d);
    }
    fn update(&mut self, dt: f64) {
        self.tank.trans.pos = self.tank.trans.pos;
    }
    fn render(&mut self, v: Matrix2d, g: &mut G2d, ) {
        self.tank.render(v, g);
    }
}
