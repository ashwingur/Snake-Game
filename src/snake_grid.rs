use rand::Rng;
use sdl2::controller::GameController;

pub const GRID_HEIGHT: usize = 25;
pub const GRID_WIDTH: usize = 25;

#[derive(Clone, Copy, Debug)]
pub enum SnakeCell {
    Air,
    Food,
    Body(Option<Direction>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum GameResult {
    Win,
    Lose,
    Continue,
}

pub struct SnakeGrid {
    pub grid: [[SnakeCell; GRID_WIDTH]; GRID_HEIGHT],
    snake_size: usize,
    head_location: (usize, usize),
    tail_location: (usize, usize),
    pub travel_direction: Direction,
    pub previous_direction: Direction,
}

impl SnakeGrid {
    pub fn new() -> SnakeGrid {
        let mut grid = [[SnakeCell::Air; GRID_WIDTH]; GRID_HEIGHT];
        let head_location = ((GRID_WIDTH / 4), (GRID_WIDTH / 4));
        let travel_direction = Direction::Right;
        grid[head_location.0][head_location.1] = SnakeCell::Body(None);
        // The 2 pixel long snake has the tail on the left of the head
        let tail_location = (head_location.0, head_location.1 - 1);
        grid[tail_location.0][tail_location.1] = SnakeCell::Body(Some(Direction::Right));

        SnakeGrid {
            grid,
            snake_size: 0,
            head_location,
            tail_location,
            travel_direction,
            previous_direction: travel_direction,
        }
    }

    pub fn generate_fruit(&mut self) {
        let y: usize = rand::thread_rng().gen_range(0..GRID_HEIGHT);
        let x: usize = rand::thread_rng().gen_range(0..GRID_WIDTH);

        if let SnakeCell::Air = self.grid[y][x] {
            self.grid[y][x] = SnakeCell::Food;
        } else {
            self.generate_fruit();
        }
    }

    pub fn update_direction(&mut self, direction: Direction) {
        if direction == SnakeGrid::opposite_direction(self.previous_direction) {
            // The player tried to make a 180 degree turn before the next tick occurred,
            // so do not allow a direction change
            return;
        }
        match direction {
            Direction::Up => {
                if self.travel_direction != Direction::Down {
                    self.travel_direction = direction;
                }
            }
            Direction::Left => {
                if self.travel_direction != Direction::Right {
                    self.travel_direction = direction;
                }
            }
            Direction::Right => {
                if self.travel_direction != Direction::Left {
                    self.travel_direction = direction;
                }
            }
            Direction::Down => {
                if self.travel_direction != Direction::Up {
                    self.travel_direction = direction;
                }
            }
        }
    }

    pub fn tick(&mut self) -> GameResult {
        // Move the head forward
        let new_coords = SnakeGrid::update_coordinates(self.head_location, self.travel_direction);

        match self.grid[new_coords.0][new_coords.1] {
            SnakeCell::Air => {
                // Set the next block to be the new head, remove the tail
                self.grid[new_coords.0][new_coords.1] = SnakeCell::Body(None);
                self.grid[self.head_location.0][self.head_location.1] =
                    SnakeCell::Body(Some(self.travel_direction));
                self.head_location = new_coords;
                let old_tail = self.tail_location;
                match self.grid[old_tail.0][old_tail.1] {
                    SnakeCell::Body(direction) => {
                        if let Some(d) = direction {
                            self.tail_location = SnakeGrid::update_coordinates(old_tail, d);
                        }
                    }
                    _ => (),
                }
                self.grid[old_tail.0][old_tail.1] = SnakeCell::Air;
                GameResult::Continue
            }
            SnakeCell::Food => {
                // Set this cell to become the new head
                let new_head =
                    SnakeGrid::update_coordinates(self.head_location, self.travel_direction);
                self.grid[self.head_location.0][self.head_location.1] =
                    SnakeCell::Body(Some(self.travel_direction));
                self.grid[new_coords.0][new_coords.1] = SnakeCell::Body(None);
                self.head_location = new_head;
                self.snake_size += 1;
                self.generate_fruit();
                GameResult::Continue
            }
            SnakeCell::Body(d) => GameResult::Lose,
        }
    }

    fn update_coordinates(mut coords: (usize, usize), direction: Direction) -> (usize, usize) {
        match direction {
            Direction::Up => {
                if coords.0 == 0 {
                    coords.0 += GRID_HEIGHT;
                }
                (coords.0 - 1, coords.1)
            }
            Direction::Down => ((coords.0 + 1) % GRID_HEIGHT, coords.1),
            Direction::Left => {
                if coords.1 == 0 {
                    coords.1 += GRID_WIDTH;
                }
                (coords.0, coords.1 - 1)
            }
            Direction::Right => (coords.0, (coords.1 + 1) % GRID_WIDTH),
        }
    }

    pub fn opposite_direction(direction: Direction) -> Direction {
        match direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
