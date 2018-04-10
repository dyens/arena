use gfx_device_gl::Resources;
use piston_window::Button;
use piston_window::PistonWindow;
use piston_window::ButtonArgs;
use piston_window::ButtonState;
use piston_window::clear;
use piston_window::line;
use piston_window::text;
use piston_window::Context;
use piston_window::Flip;
use piston_window::G2d;
use piston_window::G2dTexture;
use piston_window::Key;
use piston_window::RenderArgs;
use piston_window::Texture;
use piston_window::TextureSettings;
use piston_window::Transformed;
use piston_window::UpdateArgs;
use piston_window::Glyphs;

use find_folder;



//use piston_window::D2;
//use gfx_core::texture::Kind::D2;

use object::{Object, Vec2};
use tank::Tank;
use tank::TankAction;
use bullet::Bullet;

use std::collections::HashMap;

pub const S_WIDTH: f64 = 800.0;
pub const S_HEIGHT: f64 = 600.0;







pub struct Game<'a> {
    pub players: Vec<Tank>,
    pub active_player: Option<usize>,
    up_d: bool,
    left_d: bool,
    right_d: bool,
    scx: f64,
    scy: f64,
    pub bullets: Vec<Bullet>,
    sprites: HashMap<&'a str, Texture<Resources> >,
    glyphs: Option<Glyphs>,
    pub message: String,
    pub state: String,
}


impl<'a> Game<'a> {
    pub fn new() -> Self {
        Game {
            players: Vec::new(), // TODO number of players
            active_player: None,
            up_d: false,
            left_d: false,
            right_d: false,
            scx: S_WIDTH / 2.0,
            scy: S_HEIGHT /2.0,
            bullets: Vec::new(),
            sprites: HashMap::new(),
            glyphs: None,
            message: "started".to_string(),
            state: "started".to_string(),
        }
    }

    pub fn on_load(&mut self, w: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        let sprite = assets.join("mtank.png");
        let sprite: G2dTexture = Texture::from_path(&mut w.factory,
                                                    &sprite,
                                                    Flip::None,
                                                    &TextureSettings::new())
            .expect("couldn't find tank sprite");
        self.sprites.insert("tank_sprite", sprite);


        let sprite = assets.join("bullet.png");
        let sprite: G2dTexture = Texture::from_path(&mut w.factory,
                                                    &sprite,
                                                    Flip::None,
                                                    &TextureSettings::new())
            .expect("couldn't find tank sprite");
        self.sprites.insert("bullet_sprite", sprite);



        let ref font = assets.join("font.ttf");
        let glyphs = Glyphs::new(font,
                                 w.factory.clone(),
                                 TextureSettings::new())
            .unwrap();

        self.glyphs = Some(glyphs);

        self.generate_map();
        self.active_player = Some(0 as usize);
    }

    pub fn generate_map(&mut self) {
        // TODO: normal generation
        let mut player = Tank::new(
            self.sprites
                .get("tank_sprite"),
            S_WIDTH,
            S_HEIGHT);
        player.create_brain();
        player.mov_to(Vec2::new(
            S_WIDTH / 4.0, S_HEIGHT / 2.0));
        self.players.push(player);

        let mut player = Tank::new(
            self.sprites
                .get("tank_sprite"),
            S_WIDTH,
            S_HEIGHT);
        player.create_brain();
        player.mov_to(Vec2::new(
            S_WIDTH / 4.0 * 3.0, S_HEIGHT / 2.0));
        self.players.push(player);

    }

    pub fn on_update(&mut self, upd: &UpdateArgs) {

        // player moves
        match self.active_player {
            Some(player_index) => {
                if self.up_d {
                    //self.player.mov(0.0, -150.0 * upd.dt);
                    self.players[player_index].fwd(150.0 * upd.dt);
                }
                if self.left_d {
                    //self.player.mov(-150.0 * upd.dt, 0.0);
                    self.players[player_index].rot(-1.0 * upd.dt);
                }
                if self.right_d {
                    //self.player.mov(150.0 * upd.dt, 0.0);
                    self.players[player_index].rot(1.0 * upd.dt);
                }
            }
            None => {}
        }


        // bots moves

        for index in 0..self.players.len() {
            if let Some(player_index) = self.active_player {
                if index == player_index {
                    continue;
                }
            }

            let mut action = self.players[index]
                .get_action(&self);
            match action  {
                TankAction::FWD => {
                    self.players[index].fwd(150.0 * upd.dt);
                }
                TankAction::ROT => {
                    self.players[index].rot(1.0 * upd.dt);
                }

                TankAction::UROT => {
                    self.players[index].rot(-1.0 * upd.dt);
                }
                TankAction::FIRE => {
                    if self.players[index].can_fire() == true {
                        self.bullets.push(
                            self.players[index].fire(
                                self.sprites.get("bullet_sprite")));
                    }
                }
                TankAction::STOP => {}
            }
        }

        for player in &mut self.players {
            player.update(upd.dt);
            if let Some(ref mut brain) = player.brain {
                brain.mutate();
            }
        }

        for bullet in &mut self.bullets {
            for player in &mut self.players {
                if player.collides(&bullet){
                    player.shooted();
                    bullet.to_be_removed = true;
                }
            }
            bullet.update(upd.dt);
        }

        self.bullets.retain(|ref bul| bul.to_be_removed == false);
        self.players.retain(|ref player| player.is_destroyed == false);
//        println!("players: {:?}", self.players.len());
//        println!("bullets: {:?}", self.bullets.len());
    }

    pub fn on_draw(&mut self, c: &Context, g: &mut G2d, ren: &RenderArgs) {
        clear([0.8, 0.8, 0.8, 1.0], g);
        self.scx = (ren.width / 2) as f64  - S_WIDTH / 2.0;
        self.scy = (ren.height / 2) as f64 - S_HEIGHT / 2.0;

        let center = c.transform.trans(self.scx, self.scy);

        line([1.0, 0.4, 0.0, 1.0],
             3.0,
             [0.0, 0.0, 0.0, S_HEIGHT], // rectangle
                  center, g);

        line([1.0, 0.4, 0.0, 1.0],
             3.0,
             [0.0, 0.0, S_WIDTH, 0.0], // rectangle
             center, g);

        line([1.0, 0.4, 0.0, 1.0],
             3.0,
             [S_WIDTH, 0.0, S_WIDTH, S_HEIGHT], // rectangle
             center, g);

        line([1.0, 0.4, 0.0, 1.0],
             3.0,
             [0.0, S_HEIGHT, S_WIDTH, S_HEIGHT], // rectangle
             center, g);

        for bul in &mut self.bullets {
            bul.render(center, g);
        }

        for player in &mut self.players {
            player.render(center, g);
        }

        let _glyphs = &mut self.glyphs;
        match _glyphs {
            &mut Some(ref mut glyphs) => {
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15).draw(
                    &self.message,
                    glyphs,
                    &c.draw_state,
                    center.trans(10.0, 20.0), g
                ).unwrap();
            }
            &mut None => {}
        }
    }

    pub fn on_input(&mut self, button_args: &ButtonArgs) {
        match self.active_player {
            Some(player_index) => {
                if let Button::Keyboard(Key::Space) =
                    button_args.button {
                        if self.players[player_index].can_fire() == true {
                            self.bullets.push(
                                self.players[player_index]
                                    .fire(
                                        self.sprites.get("bullet_sprite"),
                                    ));
                        }
                    }
            }
            None => {}
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

