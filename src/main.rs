extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate graphics;
extern crate nalgebra as na;
extern crate ncollide as nc;

mod object;
mod tank;
mod bullet;

use gfx_device_gl::Resources;
use piston_window::*;

use object::{Object, Vec2};
use tank::Tank;
use bullet::Bullet;

use std::collections::HashMap;

struct Game<'a> {
    players: Vec<Tank>,
    active_player: usize,
    up_d: bool,
    left_d: bool,
    right_d: bool,
    scx: f64,
    scy: f64,
    bullets: Vec<Bullet>,
    sprites: HashMap<&'a str, Texture<Resources>>,
}

impl<'a> Game<'a> {
    fn new() -> Self {
        Game {
            players: Vec::new(), // TODO number of players
            active_player: 0 as usize,
            up_d: false,
            left_d: false,
            right_d: false,
            scx: 300.0,
            scy: 300.0,
            bullets: Vec::new(),
            sprites: HashMap::new()
        }
    }
    fn on_load(&mut self, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let tank_sprite = assets.join("E-100_Base.png");
        let tank_sprite: G2dTexture = Texture::from_path(&mut w.factory,
                                                         &tank_sprite,
                                                         Flip::None,
                                                         &TextureSettings::new())
            .expect("couldn't find tank sprite");
        self.sprites.insert("tank_sprite", tank_sprite);

        let tank_dest_sprite = assets.join("E-100_Base_destroyed.png");
        let tank_dest_sprite = Texture::from_path(&mut w.factory,
                                                  &tank_dest_sprite,
                                                  Flip::None,
                                                  &TextureSettings::new())
            .expect("couldn't find tank dest sprite");
        self.sprites.insert("tank_dest_sprite", tank_dest_sprite);

        let bullet_sprite = assets.join("Bullet.png");
        let bullet_sprite = Texture::from_path(&mut w.factory,
                                               &bullet_sprite,
                                               Flip::None,
                                               &TextureSettings::new())
            .expect("couldn't find bullet sprite");
        self.sprites.insert("bullet_sprite", bullet_sprite);

        for i in 0..5 {
            let mut player = Tank::new();
            player.tank.set_sprite(self.sprites.get("tank_sprite").unwrap().clone());
            player.mov_to(Vec2::new(100.0 + (i as f64) * 10.0, 100.0 + (i as f64) * 10.0));
            self.players.push(player);
        }
    }

    fn on_update(&mut self, upd: &UpdateArgs) {
        if self.up_d {
            //self.player.mov(0.0, -150.0 * upd.dt);
            self.players[self.active_player].fwd(150.0 * upd.dt);
        }
        if self.left_d {
            //self.player.mov(-150.0 * upd.dt, 0.0);
            self.players[self.active_player].rot(-1.0 * upd.dt);
        }
        if self.right_d {
            //self.player.mov(150.0 * upd.dt, 0.0);
            self.players[self.active_player].rot(1.0 * upd.dt);
        }

        for bullet in &mut self.bullets {
            for player in &mut self.players {
                if player.collides(&bullet){
                    player.shooted();
                    bullet.to_be_removed = true;
                }
                player.update(upd.dt);
            }
            bullet.update(upd.dt);
        }

        self.bullets.retain(|ref bul| bul.to_be_removed == false);
    }

    fn on_draw(&mut self, c: &Context, g: &mut G2d, ren: &RenderArgs) {
        clear([0.8, 0.8, 0.8, 1.0], g);
        self.scx = (ren.width / 2) as f64;
        self.scy = (ren.height / 2) as f64;
        let center = c.transform.trans(self.scx, self.scy);
        for bul in &mut self.bullets {
            bul.render(center, g);
        }

        for player in &mut self.players {
            player.render(center, g);
        }
    }

    fn on_input(&mut self, button_args: &ButtonArgs) {
        if let Button::Keyboard(Key::Space) = button_args.button {
            self.bullets.push(
                self.players[self.active_player]
                    .fire(self.sprites.get("bullet_sprite").unwrap().clone()));
        }
        match button_args.state {
            ButtonState::Press => {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Right => self.right_d = true,
                        Key::Left => self.left_d = true,
                        Key::Up => self.up_d = true,
                        _ => {}
                    }
                }
            }
            ButtonState::Release => {
                if let Button::Keyboard(key) = button_args.button {
                    match key {
                        Key::Right => self.right_d = false,
                        Key::Left => self.left_d = false,
                        Key::Up => self.up_d = false,
                        _ => {}
                    }
                }
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Areana!", [600, 600])
        .vsync(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();
    game.on_load(&mut window);
    while let Some(e) = window.next() {
        if let Some(uargs) = e.update_args() {
            game.on_update(&uargs);
        }
        if let Some(render_args) = e.render_args() {
            window.draw_2d(&e, |c, g| game.on_draw(&c, g, &render_args));
        }
        if let Some(input) = e.button_args() {
            game.on_input(&input);
        }
    }
}
