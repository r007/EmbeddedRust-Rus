use super::Prng;

use heapless::index_set::FnvIndexSet;

/// Одна точка на сетке.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coords {
    // Знаковые целые, чтобы допускать отрицательные значения (удобно при проверке того, не вышли ли мы
    // за верхний или левый край сетки)
    pub row: i8,
    pub col: i8,
}

impl Coords {
    /// Получить случайные координаты внутри сетки. `exclude` — необязательное множество
    /// координат, которые нужно исключить из результата.
    pub fn random(rng: &mut Prng, exclude: Option<&FnvIndexSet<Coords, 32>>) -> Self {
        let mut coords = Coords {
            row: ((rng.random_u32() as usize) % 5) as i8,
            col: ((rng.random_u32() as usize) % 5) as i8,
        };
        while exclude.is_some_and(|exc| exc.contains(&coords)) {
            coords = Coords {
                row: ((rng.random_u32() as usize) % 5) as i8,
                col: ((rng.random_u32() as usize) % 5) as i8,
            }
        }
        coords
    }

    /// Находится ли точка за границами сетки.
    pub fn is_out_of_bounds(&self) -> bool {
        self.row < 0 || self.row >= 5 || self.col < 0 || self.col >= 5
    }
}
