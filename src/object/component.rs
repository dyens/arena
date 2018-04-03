use piston_window::*;
use gfx_device_gl::Resources;
use graphics::math::Matrix2d;

use object::transform::Transform;

pub struct Component {
    pub trans: Transform,
    pub sprite: Option<Texture<Resources>>,
}

impl Component {

    pub fn new(sprite: Option<&Texture<Resources>>,
               s_width: f64,
               s_height: f64) -> Component {

        match sprite {
            Some(texture) =>  {
                return Component {
                    trans: Transform::new(s_width, s_height),
                    sprite: Some(texture.clone()),
                }
            }
            None => {
                return Component {
                    trans: Transform::new(s_width, s_height),
                    sprite: None,
                }
            }
        }
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
