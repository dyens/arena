use piston_window::*;
use gfx_device_gl::Resources;
use graphics::math::Matrix2d;
use nc::query::PointQuery;
use na::Isometry2;

use object::Pnt2;
use object::{Object, Cuboid2f, Vec2};
use object::component::Component;
use bullet::Bullet;
use game::Game;


const TANK_W: u32 = 150;
const TANK_H: u32 = 200;

pub enum TankAction {
    FWD,
    ROT,
    UROT,
    FIRE
}

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
            None => (TANK_W, TANK_H) // default value
        };
        //println!("Tank texture size: {:?}, {:?}", width, height);
        // scale
        let width = (width as f64) * 0.3;
        let height = (height as f64) * 0.3;

        Tank {
            tank: Component::new(sprite, s_width, s_height, width, height),
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

    pub fn fire(&self,
                sprite: Option<&Texture<Resources>>) -> Bullet {
        let mut bul = Bullet::new(sprite,
                                  self.tank.s_width,
                                  self.tank.s_height,);
        bul.mov_to(self.tank.trans.pos);
        bul.rot_to(self.tank.trans.rot);
        let (_width, height) = match sprite {
            Some(ref texture) => texture.get_size(),
            None => (100, 100) //default params of texture
        };
        let fwd = (height + 10) as f64;
        bul.fwd(fwd);
        bul
    }


    pub fn get_action(&self, game: &Game) -> TankAction {
        let players_data = game.players.iter()
            .filter(|x| x.tank.trans.pos != self.tank.trans.pos)
            .take(1) // we have 1 enemy
            .map(|x| (x.tank.trans.pos.x, x.tank.trans.pos.y, x.tank.trans.rot)).collect::<Vec<(f64, f64, f64)>>();

        let bullets_data = game.bullets.iter()
            .filter(|x| {
                let xb = x.bullet.trans.pos.x;
                let xt = self.tank.trans.pos.x;
                let yb = x.bullet.trans.pos.y;
                let yt = self.tank.trans.pos.y;

                let tg_alpha = (xt - xb) / (yt -yb);
                let alpha = tg_alpha.atan();

                let r = x.bullet.trans.rot;

                let v = alpha.cos() * r.cos() - alpha.sin() * r.sin();
                v > 0.99
            })
            .take(2) // take 2 bullet
            .map(|x| (x.bullet.trans.pos.x,
                      x.bullet.trans.pos.y,
                      x.bullet.trans.rot)
            ).collect::<Vec<(f64, f64, f64)>>();

        println!("{:?}", bullets_data.len());

        let mut data = Vec::with_capacity(12);
        data.push(self.tank.trans.pos.x);
        data.push(self.tank.trans.pos.y);
        data.push(self.tank.trans.rot);
        for p in players_data {
            data.push(p.0);
            data.push(p.1);
            data.push(p.2);
        }
        for b in bullets_data {
            data.push(b.0);
            data.push(b.1);
            data.push(b.2);
        }


        TankAction::FWD
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
        let pos = self.tank.trans.pos;
        self.tank.trans.fwd(d);
        if self.tank.on_border() == true {
            self.tank.trans.pos = pos;
        }
    }

    #[allow(unused_variables)]
    fn update(&mut self, dt: f64) {
//        self.tank.trans.pos = self.tank.trans.pos;
    }
    fn render(&mut self, v: Matrix2d, g: &mut G2d, ) {
        self.tank.render(v, g);
    }
}
