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
use brain::Brain;


const TANK_W: u32 = 150;
const TANK_H: u32 = 200;

#[derive(Debug)]
pub enum TankAction {
    FWD,
    ROT,
    UROT,
    FIRE,
    STOP
}

pub struct Tank {
    pub tank: Component,
    pub health: u32,
    pub is_destroyed: bool,
    collider: Cuboid2f,
    fire_counter: f64, // for slow fire
    pub brain: Option<Brain>,

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
            fire_counter: 0.0,
            brain: None,
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

    pub fn fire(&mut self,
                sprite: Option<&Texture<Resources>>) -> Bullet {
        self.fire_counter = 0.0;
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
        if let Some(ref brain) = self.brain {
            let mut players_data = game.players.iter()
                .filter(|x| x.tank.trans.pos != self.tank.trans.pos)
                .take(1) // we have 1 enemy
                .map(|x| {
                    let ex = x.tank.trans.pos.x;
                    let ey = x.tank.trans.pos.y;
                    let erot = x.tank.trans.rot;

                    let x = self.tank.trans.pos.x;
                    let y = self.tank.trans.pos.y;
                    let distance = ((x - ex).powi(2) +
                                    (y - ey).powi(2)).sqrt();
                    let can = self.can_fire() as u32 as f64;


                    let tg_alpha = (ex - x) / (ey -y);
                    let alpha = tg_alpha.atan();
                    let delta_alpha = alpha - self.tank.trans.rot;
                    let sin_delta_alpha = delta_alpha.sin();
                    [distance, sin_delta_alpha, ex, ey, erot, can]
                })
                .collect::<Vec<[f64; 6]>>();

            players_data.sort_by(|a, b| {
                a[0].partial_cmp(&b[0]).unwrap()});

            let mut bullets_data = game.bullets.iter()
                .filter(|x| {
                    let xb = x.bullet.trans.pos.x;
                    let yb = x.bullet.trans.pos.y;
                    let rb = x.bullet.trans.rot;

                    let xt = self.tank.trans.pos.x;
                    let yt = self.tank.trans.pos.y;

                    let dist = ((xb - xt).powi(2) +
                                (yb - yt).powi(2)).sqrt();



                    let sin_a = (xt - xb) / dist;
                    let cos_a = (yt - yb) / dist;

                    let v = rb.cos() * cos_a - rb.sin() * sin_a;
                    v > 0.99
                })
                .take(2) // take 2 bullet
                .map(|x| {
                    let bx = x.bullet.trans.pos.x;
                    let by = x.bullet.trans.pos.y;
                    let brot = x.bullet.trans.rot;

                    let x = self.tank.trans.pos.x;
                    let y = self.tank.trans.pos.y;
                    let rot = self.tank.trans.rot;
                    let can = self.can_fire() as u32 as f64;

                    let distance = ((x - bx).powi(2) +
                                    (y - by).powi(2)).sqrt();

                    let delta_alpha = brot - self.tank.trans.rot;
                    let sin_delta_alpha = delta_alpha.sin();
                    [distance, sin_delta_alpha, x, y, rot, can]
                })
                .collect::<Vec<[f64; 6]>>();
            bullets_data.sort_by(|a, b| {
                a[0].partial_cmp(&b[0]).unwrap()});


            let r = brain.calc(&players_data, &bullets_data);
            println!("action: {:?}", r);
            return r;
        }
        TankAction::STOP
    }

    pub fn create_brain(&mut self) {
        let brain = Brain::new(
            6, 5, 1,
            6, 5, 1,
            5, 1
        );
        self.brain = Some(brain);
    }

    pub fn can_fire(&self) -> bool {
        // this is ugly code(need compare time.)
        // but in training mode we have not time ;-(
        if self.fire_counter > 0.5 {
            return true;
        }
        false
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

    fn update(&mut self, dt: f64) {
        if self.fire_counter >= 100.0 {
            self.fire_counter = 50.0;
        } else {
            self.fire_counter += dt;
        }
    }

    fn render(&mut self, v: Matrix2d, g: &mut G2d, ) {
        self.tank.render(v, g);
    }
}
