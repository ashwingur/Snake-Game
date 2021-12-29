use snake_engine::SnakeEngine;

mod snake_engine;
mod snake_grid;

fn main() {
    let mut snake_engine = SnakeEngine::new();
    snake_engine.start_game();
}
