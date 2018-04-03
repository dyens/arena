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

use piston_window::Button;
use piston_window::ButtonArgs;
use piston_window::ButtonEvent;
use piston_window::ButtonState;
use piston_window::clear;
use piston_window::Context;
use piston_window::Flip;
use piston_window::G2d;
use piston_window::G2dTexture;
use piston_window::Key;
use piston_window::OpenGL;
use piston_window::PistonWindow;
use piston_window::RenderArgs;
use piston_window::RenderEvent;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Transformed;
use piston_window::UpdateArgs;
use piston_window::UpdateEvent;
use piston_window::WindowSettings;

extern crate image;



extern crate argparse;
use argparse::ArgumentParser;
use argparse::StoreTrue;


//use piston_window::D2;
//use gfx_core::texture::Kind::D2;

use object::{Object, Vec2};
use tank::Tank;
use bullet::Bullet;

use std::collections::HashMap;

const S_WIDHT: f64 = 640.0;
const S_HEIGHT: f64 = 480.0;

struct Game<'a> {
    players: Vec<Tank>,
    active_player: usize,
    up_d: bool,
    left_d: bool,
    right_d: bool,
    scx: f64,
    scy: f64,
    bullets: Vec<Bullet>,
    sprites: HashMap<&'a str, Texture<Resources> >,
}


impl<'a> Game<'a> {
    fn new() -> Self {
        Game {
            players: Vec::new(), // TODO number of players
            active_player: 0 as usize,
            up_d: false,
            left_d: false,
            right_d: false,
            scx: S_WIDHT / 2.0,
            scy: S_HEIGHT /2.0,
            bullets: Vec::new(),
            sprites: HashMap::new()
        }
    }

    fn on_load(&mut self, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

//        let im = image::open(assets.join("tanks.png")).unwrap();
//        let sprite = Texture::from_image(&mut w.factory,
//                                         &im,
//                                         &TextureSettings::new()).unwrap();

        let sprite = assets.join("mtank.png");
        let sprite: G2dTexture = Texture::from_path(&mut w.factory,
                                                    &sprite,
                                                    Flip::None,
                                                    &TextureSettings::new())
            .expect("couldn't find tank sprite");
        self.sprites.insert("tank_sprite", sprite);


        let sprite = assets.join("Bullet.png");
        let sprite: G2dTexture = Texture::from_path(&mut w.factory,
                                                    &sprite,
                                                    Flip::None,
                                                    &TextureSettings::new())
            .expect("couldn't find tank sprite");
        self.sprites.insert("bullet_sprite", sprite);
        self.generate_map()

    }

    fn generate_map(&mut self) {
        // TODO: normal generation
        for i in 0..5 {
            let mut player = Tank::new(
                self.sprites
                    .get("tank_sprite"),
                S_WIDHT,
                S_HEIGHT
            );
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
        self.players.retain(|ref player| player.is_destroyed == false);
//        println!("players: {:?}", self.players.len());
//        println!("bullets: {:?}", self.bullets.len());
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
                    .fire(
                        self.sprites.get("bullet_sprite"),
                        S_WIDHT,
                        S_HEIGHT
                    ));
            println!("{:?}", self.bullets.len());
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

    let mut train = false;
    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut train)
            .add_option(&["-t", "--train"], StoreTrue, "train");
        parser.parse_args_or_exit();
    }

    if train == false {
        let mut game = Game::new();
        let mut window: PistonWindow = PistonWindow::new(
            OpenGL::V3_3,
            0,
            WindowSettings::new("Arena", [S_WIDHT as u32, S_HEIGHT as u32])
                .opengl(OpenGL::V3_3)
                .srgb(false)
                .build()
                .unwrap(),
        );

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

    } else {
        let mut game = Game::new();
        game.generate_map();
        loop {
            let update_args = UpdateArgs{dt: 0.0083333};
            game.on_update(&update_args);
        }
    }

}
