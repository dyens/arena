use piston_window::*;

pub use graphics::math::Matrix2d;

use na::Vector2;
use na::Point2;
use nc::shape::Cuboid2;

pub type Vec2 = Vector2<f64>;
pub type Pnt2 = Point2<f64>;
pub type Cuboid2f = Cuboid2<f64>;

pub mod transform;
pub mod component;

pub trait Object {
    fn mov(&mut self, pos: Vec2);
    fn mov_to(&mut self, pos: Vec2);
    fn rot(&mut self, r: f64);
    fn rot_to(&mut self, r: f64);
    fn fwd(&mut self, d: f64);
    fn update(&mut self, dt: f64);
    fn render(&mut self, v: Matrix2d, g: &mut G2d);
}