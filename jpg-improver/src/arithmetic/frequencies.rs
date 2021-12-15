use super::{MAX_FREQUENCY, NUMBER_OF_CHARS, NUMBER_OF_SYMBOLS};

pub struct Frequencies {
    pub char_to_index: Vec<usize>,
    pub index_to_char: Vec<i32>,
    frequencies: Vec<usize>,
    cumulative: Vec<usize>,
}

impl Frequencies {
    pub fn new() -> Frequencies {
        let mut model = Frequencies {
            char_to_index: (0..NUMBER_OF_CHARS).map(|i| i + 1).collect(),
            index_to_char: (0..NUMBER_OF_SYMBOLS + 1).map(|i| (i as i32) - 1).collect(),
            frequencies: vec![1; NUMBER_OF_SYMBOLS + 1],
            cumulative: (0..NUMBER_OF_SYMBOLS + 1)
                .map(|i| NUMBER_OF_SYMBOLS - i)
                .collect(),
        };
        model.frequencies[0] = 0;
        model
    }

    pub fn total(&self) -> usize {
        self.cumulative[0]
    }

    pub fn low(&self, symbol_index: usize) -> usize {
        self.cumulative[symbol_index]
    }

    pub fn high(&self, symbol_index: usize) -> usize {
        self.cumulative[symbol_index - 1]
    }

    pub fn update(&mut self, symbol_index: usize) {
        // halve if exceeded
        if self.total() == MAX_FREQUENCY {
            let mut cum = 0;
            for i in (0..=NUMBER_OF_SYMBOLS).rev() {
                self.frequencies[i] = (self.frequencies[i] + 1) / 2;
                self.cumulative[i] = cum;
                cum += self.frequencies[i];
            }
        }

        // update symbol index if needed
        let mut new_symbol_index = symbol_index;
        while self.frequencies[new_symbol_index] == self.frequencies[(new_symbol_index - 1)] {
            new_symbol_index -= 1;
        }

        if new_symbol_index < symbol_index {
            let new_char = self.index_to_char[new_symbol_index];
            let old_char = self.index_to_char[symbol_index];

            self.index_to_char[new_symbol_index] = old_char;
            self.index_to_char[symbol_index] = new_char;

            self.char_to_index[new_char as usize] = symbol_index;
            self.char_to_index[old_char as usize] = new_symbol_index;
        }

        // update frequencies
        self.frequencies[new_symbol_index] += 1;
        let mut index_to_update = new_symbol_index;
        while index_to_update > 0 {
            index_to_update -= 1;
            self.cumulative[index_to_update] += 1;
        }
    }
}
