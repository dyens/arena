extern crate piston_window;
extern crate gfx_device_gl;
extern crate find_folder;
extern crate graphics;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate image;
extern crate argparse;
extern crate rand;

use argparse::ArgumentParser;
use argparse::StoreTrue;
use piston_window::PistonWindow;
use piston_window::OpenGL;
use piston_window::WindowSettings;
use piston_window::UpdateArgs;
use piston_window::UpdateEvent;
use piston_window::ButtonEvent;
use piston_window::RenderEvent;


mod game;
mod object;
mod tank;
mod bullet;
mod brain;

use game::Game;
use game::S_WIDTH;
use game::S_HEIGHT;


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
            WindowSettings::new("Arena", [S_WIDTH as u32, S_HEIGHT as u32])
                .opengl(OpenGL::V3_3)
                .srgb(false)
                .resizable(false)
                .decorated(true)
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
