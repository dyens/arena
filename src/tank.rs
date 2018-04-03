use piston_window::*;
use gfx_device_gl::Resources;
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
}

impl Tank {

    pub fn new(sprite: Option<&Texture<Resources>>,
               s_width: f64,
               s_height: f64) -> Tank {

        let (width, height) = match sprite {
            Some(ref texture) => texture.get_size(),
            None => (100, 100) //default params of texture
        };
        Tank {
            tank: Component::new(sprite, s_width, s_height),
            health: 100,
            is_destroyed: false,
            collider: Cuboid2f::new(Vec2::new(width as f64 / 2.0, height as f64 / 2.0)),
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

    pub fn fire(&self, sprite: Option<&Texture<Resources>>,
                s_width: f64,
                s_height: f64) -> Bullet {
        let mut bul = Bullet {
            bullet: Component::new(sprite, s_width, s_height),
            to_be_removed: false,
        };
        bul.mov_to(self.tank.trans.pos);
        bul.rot_to(self.tank.trans.rot);
        bul.fwd(125.0);
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

    #[allow(unused_variables)]
    fn update(&mut self, dt: f64) {
//        self.tank.trans.pos = self.tank.trans.pos;
    }
    fn render(&mut self, v: Matrix2d, g: &mut G2d, ) {
        self.tank.render(v, g);
    }
}
