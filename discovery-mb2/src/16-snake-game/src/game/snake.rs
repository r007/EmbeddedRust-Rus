use super::{Coords, Direction, FnvIndexSet, Turn};

use heapless::spsc::Queue;

pub struct Snake {
    /// Координаты головы змейки.
    pub head: Coords,
    /// Очередь координат остального тела змейки. Конец хвоста находится
    /// в начале очереди.
    pub tail: Queue<Coords, 32>,
    /// Множество всех координат, которые сейчас занимает змейка (для быстрой
    /// проверки столкновений).
    pub coord_set: FnvIndexSet<Coords, 32>,
    /// Направление, в котором сейчас движется змейка.
    pub direction: Direction,
}

impl Snake {
    pub fn make_snake() -> Self {
        let head = Coords { row: 2, col: 2 };
        let initial_tail = Coords { row: 2, col: 1 };
        let mut tail = Queue::new();
        tail.enqueue(initial_tail).unwrap();
        let mut coord_set: FnvIndexSet<Coords, 32> = FnvIndexSet::new();
        coord_set.insert(head).unwrap();
        coord_set.insert(initial_tail).unwrap();
        Self {
            head,
            tail,
            coord_set,
            direction: Direction::Right,
        }
    }

    /// Переместить змейку на клетку с заданными координатами. Если `extend` ложно,
    /// хвост змейки освобождает самую заднюю клетку.
    pub fn move_snake(&mut self, coords: Coords, extend: bool) {
        // Место головы становится началом хвоста
        self.tail.enqueue(self.head).unwrap();
        // Голова перемещается на новые координаты
        self.head = coords;
        self.coord_set.insert(coords).unwrap();
        if !extend {
            let back = self.tail.dequeue().unwrap();
            self.coord_set.remove(&back);
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn(&mut self, direction: Turn) {
        match direction {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
            Turn::None => (),
        }
    }
}
