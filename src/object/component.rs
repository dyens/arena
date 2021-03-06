use piston_window::*;
use gfx_device_gl::Resources;
use graphics::math::Matrix2d;

use object::transform::Transform;

pub struct Component {
    pub trans: Transform,
    pub sprite: Option<Texture<Resources>>,

    pub width: f64,
    pub height: f64,
    pub s_width: f64,
    pub s_height: f64,
}

impl Component {

    pub fn new(sprite: Option<&Texture<Resources>>,
               s_width: f64, s_height: f64,
               t_width: f64, t_height: f64,
    ) -> Component {


        match sprite {
            Some(texture) =>  {
                return Component {
                    trans: Transform::new(),
                    sprite: Some(texture.clone()),
                    s_width: s_width,
                    s_height: s_height,
                    width: t_width,
                    height: t_height,
                }
            }
            None => {
                return Component {
                    trans: Transform::new(),
                    sprite: None,
                    s_width: s_width,
                    s_height: s_height,
                    width: t_width,
                    height: t_height,
                }
            }
        }
    }

    pub fn on_border(&mut self) -> bool {
        let x = self.trans.pos.x;
        let y = self.trans.pos.y;

        let half_w = self.width / 2.0;
        let half_h = self.height / 2.0;

        if x + half_w >= self.s_width ||
            x - half_w <= 0.0 ||
            y + half_h >= self.s_height ||
            y - half_h <= 0.0 {
                return true;
            }
        false
    }

    pub fn render(&mut self, v: Matrix2d, g: &mut G2d) {
        match &self.sprite {
            &Some(ref texture) => {
                let t: Transform = self.trans;
                let (spritex, spritey) = texture.get_size();
                let (ocx, ocy) = (spritex / 2, spritey / 2);
                image(texture,
                      v.trans(t.pos.x, t.pos.y)
                      .scale(t.scale.x, t.scale.y)
                      .rot_rad(t.rot)
                      .trans(-(ocx as f64), -(ocy as f64)),
                      g);
            }
            &None => {}
        }
    }
}
