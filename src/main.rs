

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate ggez;
//extern crate rand;
use ggez::conf;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2, Rect};
use ggez::timer;
//use ggez::graphics::{Vector2, Point2};
use ggez::nalgebra as na;

//use std::env;
//use std::path;

use glass::Glass;

use figures::Figure;

mod figures;
mod glass;

struct MainState {
    screen_width: u32,
    screen_height: u32,
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    glass: Glass,
    block_size: f32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (33, 55, 122, 255).into());
        let mut glass = Glass::new(12, 26);
        let figure = Figure::LeftZig;
        let figure_map = figure.draw();

        let row = 0;
        let col = (glass.width as isize - 4) / 2 - 1;
        glass.place(figure_map, (row, col));

        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;
        let block_size = screen_height as f32 * 3.0/4.0 / glass.height as f32;
        Ok(MainState {
            screen_width,
            screen_height,
            x: screen_width as f32 / 2.0,
            y: screen_height as f32 / 2.0,
            dx: 0.0,
            dy: 0.0,
            glass,
            block_size,
        })
    }

    fn draw_glass(&self, ctx: &mut Context) -> GameResult<()> {
        let w = self.glass_width();
        let x = self.glass_x();
        let h = self.glass_height();
        let y = self.glass_y();
        graphics::rectangle(ctx, DrawMode::Line(1.0), Rect { x, y, w, h })?;
        Ok(())
    }

    fn glass_width(&self) -> f32 {
        self.block_size * self.glass.width as f32
    }

    fn glass_x(&self) -> f32 {
        let w = self.glass_width();
        (self.screen_width as f32 - w) / 2.0
    }

    fn glass_height(&self) -> f32 {
        self.block_size * self.glass.height as f32
    }

    fn glass_y(&self) -> f32 {
        let h = self.glass_height();
        (self.screen_height as f32 - h) / 2.0
    }

    fn draw_figure(&self, ctx: &mut Context) -> GameResult<()> {
        let figure = self.glass.figure.unwrap(); //TODO: FIX!
        for row in 0..4 {
            for col in 0..4 {
                if figure.figure[row][col] {
                    let w = self.block_size;
                    let (f_row, f_col) = figure.position;
                    let x = self.glass_x() + (f_col as f32 + col as f32) * w;
                    let y = self.glass_y() + (f_row as f32 + row as f32) * w;

                    graphics::rectangle(ctx, DrawMode::Line(1.0), Rect { x, y, w, h: w })?;
                }
            }
        }
        Ok(())
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.x += self.dx;
        self.y += self.dy;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, (135, 55, 5, 255).into());

        graphics::circle(ctx,
                         DrawMode::Line(8.0),
                         Point2::new(self.x, self.y),
                         100.0,
                         0.10)?;

        self.draw_glass(ctx);

        self.draw_figure(ctx);

        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }
        match keycode {
            Keycode::Right => self.dx += 10.0,
            Keycode::Left => self.dx -= 10.0,
            Keycode::Up => self.dy -= 10.0,
            Keycode::Down => self.dy += 10.0,
            _ => {}
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }
        match keycode {
            Keycode::Right => self.dx -= 10.0,
            Keycode::Left => self.dx += 10.0,
            Keycode::Up => self.dy += 10.0,
            Keycode::Down => self.dy -= 10.0,
            _ => {}
        }
    }
}


fn main() {
    println!("Tetris!");

//    let glass = Glass::new(15, 30);

    let mut cb = ContextBuilder::new("tetris", "ygree")
        .window_setup(conf::WindowSetup::default()
            .title("Tetris!")
        )
        .window_mode(conf::WindowMode::default()
            .dimensions(640, 480)
        );

    let ctx = &mut cb.build().unwrap();

    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx, game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
    }

}
