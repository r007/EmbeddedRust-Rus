use crate::Rng;

/// Простой псевдослучайный генератор чисел.
pub struct Prng {
    value: u32,
}

impl Prng {
    pub fn seeded(rng: &mut Rng) -> Self {
        Self::new(rng.random_u32())
    }

    pub fn new(seed: u32) -> Self {
        Self { value: seed }
    }

    /// Простая функция xorshift PRNG: см. <https://en.wikipedia.org/wiki/Xorshift>
    fn xorshift32(mut input: u32) -> u32 {
        input ^= input << 13;
        input ^= input >> 17;
        input ^= input << 5;
        input
    }

    /// Вернуть псевдослучайное u32.
    pub fn random_u32(&mut self) -> u32 {
        self.value = Self::xorshift32(self.value);
        self.value
    }
}
