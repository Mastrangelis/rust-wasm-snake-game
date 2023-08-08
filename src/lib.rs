use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rand.ts")]
extern {
    fn randomRangeIdx(min: usize, max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    length: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut length: Vec<SnakeCell> = vec!();

        for i in 0..size {
            length.push(SnakeCell(spawn_index - i));
        }
        Snake {
            length,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    state: Option<GameStatus>,
    points: usize,
}

#[allow(private_in_public)]
#[allow(dead_code)]
#[wasm_bindgen]
impl World {
    pub fn new(width: Option<usize>, spawn_index: Option<usize>) -> World {
        let world_width: usize = width.unwrap_or(4); // default value
        let world_size: usize = world_width * world_width;
        let snake: Snake = Snake::new(spawn_index.unwrap_or(0), 3);

        let reward_cell = World::gen_next_reward_cell(
            world_size - 1,
            &snake.length
        );

        World {
            width: world_width,
            size: world_size,
            snake,
            next_cell: None,
            reward_cell,
            state: None,
            points: 0,
        }
    }

    fn gen_next_reward_cell(
        max: usize,
        snake_body: &Vec<SnakeCell>
    ) -> Option<usize> {
        let mut reward_cell;

        loop {
            reward_cell = randomRangeIdx(0, max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }

        Some(reward_cell)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_head(&self) -> usize {
        self.snake.length[0].0
    }

    pub fn snake_length(&self) -> usize {
        self.snake.length.len()
    }

    pub fn snake_tail(&self) -> usize {
        self.snake.length[self.snake.length.len() - 1].0
    }

    pub fn start_game(&mut self) {
        self.state = Some(GameStatus::Played);
    }

    pub fn game_status(&self) -> Option<GameStatus> {
        self.state
    }

    pub fn game_status_text(&self) -> String {
        match self.state {
            Some(GameStatus::Won) => "You won!".to_string(),
            Some(GameStatus::Lost) => "You lost!".to_string(),
            Some(GameStatus::Played) => "Game is on!".to_string(),
            None => "Game is not started!".to_string(),
        }
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn set_snake_direction(&mut self, direction: &str) {
        let next_direction = match direction {
            "ArrowUp" => Direction::Up,
            "ArrowRight" => Direction::Right,
            "ArrowDown" => Direction::Down,
            "ArrowLeft" => Direction::Left,
            _ => panic!("Invalid direction"),
        };

        let next_cell: SnakeCell = self.gen_next_snake_cell(&next_direction);

        if self.snake.length[1].0 == next_cell.0 {
            return;
        }

        self.next_cell = Some(next_cell);

        self.snake.direction = next_direction;
    }

    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.length.as_ptr()
    }

    pub fn step(&mut self) {
        match self.state {
            Some(GameStatus::Played) => {
                let temp: Vec<SnakeCell> = self.snake.length.clone();

                match self.next_cell {
                    Some(next_cell) => {
                        self.snake.length[0] = next_cell;
                        self.next_cell = None;
                    }
                    None => {
                        self.snake.length[0] = self.gen_next_snake_cell(
                            &self.snake.direction
                        );
                    }
                }

                for i in 1..self.snake_length() {
                    self.snake.length[i] = SnakeCell(temp[i - 1].0);
                }

                if
                    self.snake.length[1..self.snake_length()].contains(
                        &self.snake.length[0]
                    )
                {
                    self.state = Some(GameStatus::Lost);
                }

                if self.reward_cell == Some(self.snake_head()) {
                    if self.snake_length() < self.size {
                        self.points += 1;
                        self.reward_cell = World::gen_next_reward_cell(
                            self.size - 1,
                            &self.snake.length
                        );
                    } else {
                        self.reward_cell = None;
                        self.state = Some(GameStatus::Won);
                    }

                    self.snake.length.push(SnakeCell(self.snake.length[1].0));
                }
            }
            _ => (),
        }
    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx: usize = self.snake_head();
        let row = snake_idx / self.width;

        return match self.snake.direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            }
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            }
            Direction::Up => {
                let treshold = snake_idx - row * self.width;
                if snake_idx == treshold {
                    SnakeCell(self.size - self.width + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            }
            Direction::Down => {
                let treshold = snake_idx + (self.width - row) * self.width;
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - (row + 1) * self.width)
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            }
        };
    }
}
