use super::{
    frequencies::Frequencies, CODE_VALUE_BITS, CODE_VALUE_FIRST_QUARTER, CODE_VALUE_HALF,
    CODE_VALUE_MAX, CODE_VALUE_THIRD_QUARTER, EOF_SYMBOL,
};

fn bit_iterator(data: &[u8]) -> impl Iterator<Item = bool> + '_ {
    data.iter()
        .flat_map(|x| (0..8).rev().map(move |i| (x >> i) & 1 == 1))
}

pub struct Decoder<'a> {
    encoded_data: Box<dyn Iterator<Item = bool> + 'a>,
    pub decoded_data: Vec<u8>,
    friequencies: Frequencies,
    low: usize,
    high: usize,
    code_value: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(encoded_data: &'a [u8]) -> Decoder<'a> {
        let mut writer = Decoder {
            encoded_data: Box::new(bit_iterator(encoded_data)),
            decoded_data: Vec::new(),
            friequencies: Frequencies::new(),
            low: 0,
            high: CODE_VALUE_MAX,
            code_value: 0,
        };

        for _ in 0..CODE_VALUE_BITS {
            writer.slide();
        }
        writer
    }

    fn slide(&mut self) {
        match self.encoded_data.next() {
            Some(next_bit) => self.code_value = 2 * self.code_value + (next_bit as usize),
            None => self.code_value *= 2,
        }
    }

    fn get_symbol_index(&self, cum: usize) -> usize {
        let mut symbol_index = 1;
        while self.friequencies.get_low(symbol_index) > cum {
            symbol_index += 1;
        }
        symbol_index
    }

    fn next_symbol_index(&mut self) -> usize {
        let range = self.high - self.low + 1;
        let total = self.friequencies.get_total();
        let cum = ((self.code_value - self.low + 1) * total - 1) / range;

        let symbol_index = self.get_symbol_index(cum);
        let symbol_low = self.friequencies.get_low(symbol_index);
        let symbol_high = self.friequencies.get_high(symbol_index);
        self.high = self.low + range * symbol_high / total - 1;
        self.low += range * symbol_low / total;

        loop {
            if self.high < CODE_VALUE_HALF {
            } else if self.low >= CODE_VALUE_HALF {
                self.code_value -= CODE_VALUE_HALF;
                self.low -= CODE_VALUE_HALF;
                self.high -= CODE_VALUE_HALF;
            } else if self.low >= CODE_VALUE_FIRST_QUARTER && self.high < CODE_VALUE_THIRD_QUARTER {
                self.code_value -= CODE_VALUE_FIRST_QUARTER;
                self.low -= CODE_VALUE_FIRST_QUARTER;
                self.high -= CODE_VALUE_FIRST_QUARTER;
            } else {
                break;
            }
            self.low *= 2;
            self.high = 2 * self.high + 1;
            self.slide();
        }

        symbol_index
    }

    pub fn decode(&mut self) -> &Vec<u8> {
        loop {
            let symbol_index = self.next_symbol_index();
            if symbol_index == EOF_SYMBOL {
                break;
            }

            let char = self.friequencies.index_to_char(symbol_index);
            self.decoded_data.push(char as u8);
            self.friequencies.update(symbol_index);
        }
        &self.decoded_data
    }
}
