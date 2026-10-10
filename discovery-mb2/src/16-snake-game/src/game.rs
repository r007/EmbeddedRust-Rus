mod coords;
mod movement;
mod rng;
mod snake;

use crate::Rng;

pub use coords::Coords;
pub use movement::{Direction, GameStatus, StepOutcome, Turn};
pub use rng::Prng;
pub use snake::Snake;

use heapless::index_set::FnvIndexSet;

/// Структура для хранения состояния игры и связанного с ним поведения
pub struct Game {
    pub status: GameStatus,
    rng: Prng,
    snake: Snake,
    food_coords: Coords,
    speed: u8,
    score: u8,
}

impl Game {
    pub fn new(rng: &mut Rng) -> Self {
        let mut rng = Prng::seeded(rng);
        let snake = Snake::make_snake();
        let food_coords = Coords::random(&mut rng, Some(&snake.coord_set));
        Self {
            rng,
            snake,
            food_coords,
            speed: 1,
            status: GameStatus::Ongoing,
            score: 0,
        }
    }

    /// Сбросить состояние игры, чтобы начать новую игру.
    pub fn reset(&mut self) {
        self.snake = Snake::make_snake();
        self.place_food();
        self.speed = 1;
        self.status = GameStatus::Ongoing;
        self.score = 0;
    }

    /// Случайным образом разместить еду на сетке.
    fn place_food(&mut self) -> Coords {
        let coords = Coords::random(&mut self.rng, Some(&self.snake.coord_set));
        self.food_coords = coords;
        coords
    }

    /// «Сквозной переход» для координат за пределами сетки (например, координаты левее
    /// левого края сетки появятся в крайнем правом столбце). Предполагается, что
    /// координаты выходят за границы только по одному измерению.
    fn wraparound(&self, coords: Coords) -> Coords {
        if coords.row < 0 {
            Coords { row: 4, ..coords }
        } else if coords.row >= 5 {
            Coords { row: 0, ..coords }
        } else if coords.col < 0 {
            Coords { col: 4, ..coords }
        } else {
            Coords { col: 0, ..coords }
        }
    }

    /// Определить следующую клетку, на которую переместится змейка (без
    /// фактического перемещения).
    fn get_next_move(&self) -> Coords {
        let head = &self.snake.head;
        let next_move = match self.snake.direction {
            Direction::Up => Coords {
                row: head.row - 1,
                col: head.col,
            },
            Direction::Down => Coords {
                row: head.row + 1,
                col: head.col,
            },
            Direction::Left => Coords {
                row: head.row,
                col: head.col - 1,
            },
            Direction::Right => Coords {
                row: head.row,
                col: head.col + 1,
            },
        };
        if next_move.is_out_of_bounds() {
            self.wraparound(next_move)
        } else {
            next_move
        }
    }

    /// Оценить следующий ход змейки и вернуть результат. Состояние игры при этом
    /// фактически не обновляется.
    fn get_step_outcome(&self) -> StepOutcome {
        let next_move = self.get_next_move();
        if self.snake.coord_set.contains(&next_move) {
            // Змейку мы ещё не двигали, поэтому если следующий ход приходится на конец
            // хвоста, столкновения на самом деле не будет (хвост к тому моменту
            // уже сдвинется, когда голова заедет на клетку)
            if next_move != *self.snake.tail.peek().unwrap() {
                StepOutcome::Collision
            } else {
                StepOutcome::Move(next_move)
            }
        } else if next_move == self.food_coords {
            if self.snake.tail.len() == 23 {
                StepOutcome::Full
            } else {
                StepOutcome::Eat(next_move)
            }
        } else {
            StepOutcome::Move(next_move)
        }
    }

    /// Обработать результат шага, обновив внутреннее состояние игры.
    fn handle_step_outcome(&mut self, outcome: StepOutcome) {
        self.status = match outcome {
            StepOutcome::Collision => GameStatus::Lost,
            StepOutcome::Full => GameStatus::Won,
            StepOutcome::Eat(c) => {
                self.snake.move_snake(c, true);
                self.place_food();
                self.score += 1;
                if self.score.is_multiple_of(5) {
                    self.speed += 1
                }
                GameStatus::Ongoing
            }
            StepOutcome::Move(c) => {
                self.snake.move_snake(c, false);
                GameStatus::Ongoing
            }
        }
    }

    pub fn step(&mut self, turn: Turn) {
        self.snake.turn(turn);
        let outcome = self.get_step_outcome();
        self.handle_step_outcome(outcome);
    }

    /// Вычислить время ожидания между шагами игры в миллисекундах.
    /// Обычно оно уменьшается по мере роста счёта игрока, но нужно
    /// следить, чтобы оно не стало отрицательным.
    pub fn step_len_ms(&self) -> u32 {
        let result = 1000 - (200 * ((self.speed as i32) - 1));
        if result < 200 {
            200u32
        } else {
            result as u32
        }
    }

    /// Вернуть массив, описывающий состояние игры; его можно использовать, чтобы вывести
    /// состояние на светодиодной матрице micro:bit. Каждый параметр `_brightness` должен быть
    /// значением от 0 до 9.
    pub fn game_matrix(
        &self,
        head_brightness: u8,
        tail_brightness: u8,
        food_brightness: u8,
    ) -> [[u8; 5]; 5] {
        let mut values = [[0u8; 5]; 5];
        values[self.snake.head.row as usize][self.snake.head.col as usize] = head_brightness;
        for t in &self.snake.tail {
            values[t.row as usize][t.col as usize] = tail_brightness
        }
        values[self.food_coords.row as usize][self.food_coords.col as usize] = food_brightness;
        values
    }

    /// Вернуть массив, описывающий счёт игры; его можно использовать, чтобы вывести
    /// счёт на светодиодной матрице micro:bit (зажигая соответствующее количество
    /// светодиодов, слева направо и сверху вниз).
    pub fn score_matrix(&self) -> [[u8; 5]; 5] {
        let mut values = [[0u8; 5]; 5];
        let full_rows = (self.score as usize) / 5;
        #[allow(clippy::needless_range_loop)]
        for r in 0..full_rows {
            values[r] = [1; 5];
        }
        #[allow(clippy::needless_range_loop)]
        for c in 0..(self.score as usize) % 5 {
            values[full_rows][c] = 1;
        }
        values
    }
}
