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

fn encode_following_bit(writer: &mut BitWriter, bits_to_follow: &mut i32, bit: bool) {
    writer.write(bit);
    while *bits_to_follow > 0 {
        writer.write(!bit);
        *bits_to_follow -= 1;
    }
}

fn encode_char(
    writer: &mut BitWriter,
    frequencies: &mut Frequencies,
    low: &mut usize,
    high: &mut usize,
    bits_to_follow: &mut i32,
    char_index: usize,
) {
    let range = *high - *low + 1;
    let total = frequencies.total();
    let char_low = frequencies.low(char_index);
    let char_high = frequencies.high(char_index);
    *high = *low + range * char_high / total - 1;
    *low += range * char_low / total;

    loop {
        if *high < CODE_VALUE_HALF {
            encode_following_bit(writer, bits_to_follow, false);
        } else if *low >= CODE_VALUE_HALF {
            encode_following_bit(writer, bits_to_follow, true);
            *low -= CODE_VALUE_HALF;
            *high -= CODE_VALUE_HALF;
        } else if *low >= CODE_VALUE_FIRST_QUARTER && *high < CODE_VALUE_THIRD_QUARTER {
            *bits_to_follow += 1;
            *low -= CODE_VALUE_FIRST_QUARTER;
            *high -= CODE_VALUE_FIRST_QUARTER;
        } else {
            break;
        }
        *low *= 2;
        *high = 2 * *high + 1;
    }
}

pub fn encode(data: &[u8]) -> Vec<u8> {
    let mut writer = BitWriter::new();
    let mut frequencies = Frequencies::new();
    let mut low = 0;
    let mut high = CODE_VALUE_MAX;
    let mut bits_to_follow = 0;

    for i in data {
        let char_index = frequencies.char_to_index[*i as usize];
        encode_char(
            &mut writer,
            &mut frequencies,
            &mut low,
            &mut high,
            &mut bits_to_follow,
            char_index,
        );
        frequencies.update(char_index);
    }

    encode_char(
        &mut writer,
        &mut frequencies,
        &mut low,
        &mut high,
        &mut bits_to_follow,
        EOF_SYMBOL,
    );
    bits_to_follow += 1;

    encode_following_bit(
        &mut writer,
        &mut bits_to_follow,
        low >= CODE_VALUE_FIRST_QUARTER,
    );
    writer.data
}
