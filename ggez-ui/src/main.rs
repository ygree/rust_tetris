
extern crate ggez;
//extern crate rand;
use ggez::conf;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics;
use ggez::graphics::{DrawMode, Rect};
use ggez::timer;
//use ggez::nalgebra as na;

use glass::Glass;

//use figures::Figure;
use glass::MoveDirection;

use core::glass::{Glass, MoveDirection};

struct MainState {
    screen_width: u32,
    screen_height: u32,
    glass: Glass,
    block_size: f32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();
        graphics::set_background_color(ctx, (33, 55, 122, 255).into());
        let mut glass = Glass::new(12, 26);
        glass.next_figure();

        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;
        let block_size = screen_height as f32 * 3.0/4.0 / glass.height as f32;
        let main_state = MainState {
            screen_width,
            screen_height,
            glass,
            block_size,
        };

        Ok(main_state)
    }

    fn draw_glass(&self, ctx: &mut Context) -> GameResult<()> {
        let w = self.glass_width();
        let x = self.glass_x();
        let h = self.glass_height();
        let y = self.glass_y();
        graphics::rectangle(ctx, DrawMode::Line(1.58), Rect { x, y, w, h })?;
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
        if self.glass.figure.is_some() {
            let figure = self.glass.figure.unwrap(); //TODO: FIX!

            for &(col, row) in figure.figure.blocks.iter() {
                let w = self.block_size;
                let (f_row, f_col) = figure.position;
                let x = self.glass_x() + (f_col as f32 + col as f32) * w;
                let y = self.glass_y() + (f_row as f32 + row as f32) * w;

                graphics::rectangle(ctx, DrawMode::Fill, Rect { x, y, w, h: w })?;
                graphics::rectangle(ctx, DrawMode::Line(1.0), Rect { x, y, w, h: w })?;
            }
        }
        Ok(())
    }

    fn draw_content(&self, ctx: &mut Context) -> GameResult<()> {
        let w = self.block_size;
        let x0 = self.glass_x();
        let y0 = self.glass_y();
        for row in 0 .. self.glass.height {
            for col in 0 .. self.glass.width {
                if self.glass[row][col] {
                    let x = x0 + col as f32 * w;
                    let y = y0 + row as f32 * w;
                    graphics::rectangle(ctx, DrawMode::Fill, Rect { x, y, w, h: w })?;
                }
            }
        }
        Ok(())
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.glass.clean_filled_rows();
        while timer::check_update_time(ctx, 1) {
            if !self.glass.relocate_figure(MoveDirection::Down) {
                self.glass.freeze_figure();
                self.glass.next_figure();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        graphics::set_color(ctx, (133, 123, 55, 255).into())?;
        self.draw_content(ctx)?;

        graphics::set_color(ctx, (135, 55, 5, 255).into())?;
        self.draw_glass(ctx)?;

        graphics::set_color(ctx, (133, 123, 55, 64).into())?;
        self.draw_figure(ctx)?;

        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
//            return;
        }
        match keycode {
            Keycode::Right => {
                self.glass.relocate_figure(MoveDirection::Right);
            },
            Keycode::Left => {
                self.glass.relocate_figure(MoveDirection::Left);
            },
            Keycode::Up => {
                self.glass.rotate_figure();
            },
            Keycode::Down => {
                while self.glass.relocate_figure(MoveDirection::Down) {
                }
//                self.glass.freeze_figure();
//                self.glass.next_figure();
            },
            _ => {}
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if repeat {
            return;
        }
        match keycode {
            _ => {}
        }
    }
}


fn main() {
    println!("Tetris!");

    let cb = ContextBuilder::new("tetris", "ygree")
        .window_setup(conf::WindowSetup::default()
            .title("Tetris!")
        )
        .window_mode(conf::WindowMode::default()
            .dimensions(800, 600)
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
