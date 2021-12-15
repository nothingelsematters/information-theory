use super::{
    frequencies::Frequencies, CODE_VALUE_FIRST_QUARTER, CODE_VALUE_HALF, CODE_VALUE_MAX,
    CODE_VALUE_THIRD_QUARTER, EOF_SYMBOL,
};

pub struct BitWriter {
    pub data: Vec<u8>,
    bit_index: isize,
}

impl BitWriter {
    fn new() -> BitWriter {
        BitWriter {
            data: vec![0],
            bit_index: 7,
        }
    }

    fn write(&mut self, bit: bool) {
        if self.bit_index == -1 {
            self.data.push(0);
            self.bit_index = 7;
        }
        let len = self.data.len();
        self.data[len - 1] |= (bit as u8) << self.bit_index;
        self.bit_index -= 1;
    }
}

pub struct Encoder<'a> {
    pub writer: BitWriter,
    initial_data: &'a [u8],
    frequency_model: Frequencies,
    low: usize,
    high: usize,
    bits_to_follow: i32,
}

impl<'a> Encoder<'a> {
    pub fn new(initial_data: &[u8]) -> Encoder {
        Encoder {
            writer: BitWriter::new(),
            frequency_model: Frequencies::new(),
            low: 0,
            high: CODE_VALUE_MAX,
            bits_to_follow: 0,
            initial_data,
        }
    }

    fn encode_following_bit(&mut self, bit: bool) {
        self.writer.write(bit);
        while self.bits_to_follow > 0 {
            self.writer.write(!bit);
            self.bits_to_follow -= 1;
        }
    }

    fn encode_char(&mut self, char_index: usize) {
        let range = self.high - self.low + 1;
        let total = self.frequency_model.total();
        let char_low = self.frequency_model.low(char_index);
        let char_high = self.frequency_model.high(char_index);
        self.high = self.low + range * char_high / total - 1;
        self.low += range * char_low / total;

        loop {
            if self.high < CODE_VALUE_HALF {
                self.encode_following_bit(false);
            } else if self.low >= CODE_VALUE_HALF {
                self.encode_following_bit(true);
                self.low -= CODE_VALUE_HALF;
                self.high -= CODE_VALUE_HALF;
            } else if self.low >= CODE_VALUE_FIRST_QUARTER && self.high < CODE_VALUE_THIRD_QUARTER {
                self.bits_to_follow += 1;
                self.low -= CODE_VALUE_FIRST_QUARTER;
                self.high -= CODE_VALUE_FIRST_QUARTER;
            } else {
                break;
            }
            self.low *= 2;
            self.high = 2 * self.high + 1;
        }
    }

    pub fn encode(&mut self) {
        for i in self.initial_data {
            let char_index = self.frequency_model.char_to_index[*i as usize];
            self.encode_char(char_index);
            self.frequency_model.update(char_index);
        }
        self.encode_char(EOF_SYMBOL);
        self.bits_to_follow += 1;
        self.encode_following_bit(self.low >= CODE_VALUE_FIRST_QUARTER);
    }
}
