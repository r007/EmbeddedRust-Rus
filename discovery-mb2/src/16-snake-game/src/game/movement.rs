use super::Coords;

/// Определяем направления, в которых может двигаться змейка.
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// В каком направлении должна повернуть змейка.
#[derive(Debug, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
    None,
}

/// Текущий статус игры.
pub enum GameStatus {
    Won,
    Lost,
    Ongoing,
}

/// Результат одного хода (шага).
pub enum StepOutcome {
    /// Сетка заполнена (игрок выигрывает)
    Full,
    /// Змейка врезалась в себя (игрок проигрывает)
    Collision,
    /// Змейка съела еду
    Eat(Coords),
    /// Змейка переместилась (и больше ничего не произошло)
    Move(Coords),
}
