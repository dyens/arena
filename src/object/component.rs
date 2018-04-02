use piston_window::*;
use gfx_device_gl::Resources;
use graphics::math::Matrix2d;

use object::transform::Transform;

pub struct Component {
    pub trans: Transform,
    pub sprite: Option<Texture<Resources>>,
}

impl Component {
    pub fn new() -> Component {
        Component {
            trans: Transform::new(),
            sprite: None,
        }
    }
    pub fn render(&mut self,v: Matrix2d, g: &mut G2d) {
        let t: Transform = self.trans;
        if let Some(ref sprite) = self.sprite {
            let (spritex, spritey) = sprite.get_size();
            let (ocx, ocy) = (spritex / 2, spritey / 2);
            image(sprite,
                  v.trans(t.pos.x, t.pos.y)
                      .scale(t.scale.x, t.scale.y)
                      .rot_rad(t.rot)
                      .trans(-(ocx as f64), -(ocy as f64)),
                  g);
        }

    }
    pub fn set_sprite(&mut self, texture: Texture<Resources>) {
        self.sprite = Some(texture);
    }
}