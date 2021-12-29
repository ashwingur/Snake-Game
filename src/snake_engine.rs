use core::time;
use std::thread;

use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use crate::snake_grid::Direction;

use super::snake_grid::{SnakeCell, SnakeGrid, GRID_HEIGHT, GRID_WIDTH};

const PIXEL_SCALE: u32 = 20;

pub struct SnakeEngine {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    snake_grid: SnakeGrid,
}

enum KeyboardResult {
    Normal,
    Exit,
    Restart,
}

impl SnakeEngine {
    pub fn new() -> SnakeEngine {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "Snake",
                GRID_WIDTH as u32 * PIXEL_SCALE,
                GRID_HEIGHT as u32 * PIXEL_SCALE,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        SnakeEngine {
            canvas,
            event_pump,
            snake_grid: SnakeGrid::new(),
        }
    }

    pub fn start_game(&mut self) {
        let mut freq = 0;
        self.snake_grid.generate_fruit();
        'running: loop {
            match self.read_input() {
                KeyboardResult::Exit => break 'running,
                _ => (),
            }
            if freq == 0 {
                // Update the previous direction now that a tick has been made
                self.snake_grid.previous_direction = self.snake_grid.travel_direction;
                self.snake_grid.tick();
                self.draw_frame();
                freq = 20;
            }
            thread::sleep(time::Duration::from_millis(5));
            freq -= 1;
        }
    }

    fn read_input(&mut self) -> KeyboardResult {
        self.event_pump.pump_events();

        let mut code = KeyboardResult::Normal;
        for s in self.event_pump.keyboard_state().pressed_scancodes() {
            code = match s {
                Scancode::Right | Scancode::D => {
                    self.snake_grid.update_direction(Direction::Right);
                    KeyboardResult::Normal
                }
                Scancode::Left | Scancode::A => {
                    self.snake_grid.update_direction(Direction::Left);
                    KeyboardResult::Normal
                }
                Scancode::Up | Scancode::W => {
                    self.snake_grid.update_direction(Direction::Up);
                    KeyboardResult::Normal
                }
                Scancode::Down | Scancode::S => {
                    self.snake_grid.update_direction(Direction::Down);
                    KeyboardResult::Normal
                }
                Scancode::Escape => KeyboardResult::Exit,
                _ => KeyboardResult::Normal,
            };
        }
        code
    }

    fn draw_frame(&mut self) {
        // Set the whole background to black
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Draw the grid with green and dark green tiles alternating
        // Draw the snake as blue, and the food as red
        for row in 0..GRID_HEIGHT {
            for col in 0..GRID_WIDTH {
                match self.snake_grid.grid[row][col] {
                    SnakeCell::Air => {
                        if row % 2 == 0 {
                            if col % 2 == 0 {
                                self.canvas.set_draw_color(Color::RGB(101, 204, 90));
                            } else {
                                self.canvas.set_draw_color(Color::RGB(71, 189, 58));
                            }
                        } else {
                            if col % 2 == 0 {
                                self.canvas.set_draw_color(Color::RGB(71, 189, 58));
                            } else {
                                self.canvas.set_draw_color(Color::RGB(101, 204, 90));
                            }
                        }
                    }
                    SnakeCell::Food => self.canvas.set_draw_color(Color::RGB(255, 25, 33)),
                    SnakeCell::Body(_) => self.canvas.set_draw_color(Color::RGB(52, 118, 224)),
                }

                self.canvas
                    .fill_rect(Rect::new(
                        (col as u32 * PIXEL_SCALE) as i32,
                        (row as u32 * PIXEL_SCALE) as i32,
                        PIXEL_SCALE,
                        PIXEL_SCALE,
                    ))
                    .unwrap_or_else(|_| ());
            }
        }
        self.canvas.present();
    }
}
