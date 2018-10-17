use piston_window::*;
use object::component::Component;
use gfx_device_gl::Resources;

const ROCKET_W: u32 = 150;
const ROCKET_H: u32 = 200;



#[derive(Debug)]
pub struct Rocket {
    pub rocket: Component,
    pub state: i8,
    sprites: Vec<Texture<Resources>>,
}

impl Rocket {
    pub fn new(
        sprites: Vec<&Texture<Resources>>,
        s_width: f64,
        s_height: f64,
    ) -> Rocket {
        let (width, height) = match sprite {
            Some(ref texture) => texture.get_size(),
            None => (ROCKET_W, ROCKET_H) // default value
        };
        println!("Rocket texture size: {:?}, {:?}", width, height);
        // scale
        let width = (width as f64) * 0.3;
        let height = (height as f64) * 0.3;
        Rocket {
            rocket: Component::new(sprites[0],
                                   s_width,
                                   s_height,
                                   width,
                                   height),
            state: 0,
        }
    }
}


impl Object for Rocket {
    fn mov(&mut self, pos: Vec2) {
        self.rocket.trans.mov(pos);
    }
    fn mov_to(&mut self, pos: Vec2) {
        self.rocket.trans.mov_to(pos);
    }
    fn rot(&mut self, r: f64) {
        self.rocket.trans.rot(r);
    }
    fn rot_to(&mut self, r: f64) {
        self.rocket.trans.rot_to(r);
    }

    fn update(&mut self, dt: f64) {
        if self.fire_counter >= 100.0 {
            self.fire_counter = 50.0;
        } else {
            self.fire_counter += dt;
        }
    }

    fn render(&mut self, v: Matrix2d, g: &mut G2d, ) {
        self.rocket.render(v, g);
    }
}
