use super::{
    frequencies::Frequencies, CODE_VALUE_BITS, CODE_VALUE_FIRST_QUARTER, CODE_VALUE_HALF,
    CODE_VALUE_MAX, CODE_VALUE_THIRD_QUARTER, EOF_SYMBOL,
};

fn bit_iterator(data: &[u8]) -> impl Iterator<Item = bool> + '_ {
    data.iter()
        .flat_map(|x| (0..8).rev().map(move |i| (x >> i) & 1 == 1))
}

fn slide(iter: &mut Box<impl Iterator<Item = bool>>, code_value: &mut usize) {
    match iter.next() {
        Some(next_bit) => *code_value = 2 * *code_value + (next_bit as usize),
        None => *code_value *= 2,
    }
}

fn get_symbol_index(frequencies: &mut Frequencies, cum: usize) -> usize {
    let mut symbol_index = 1;
    while frequencies.low(symbol_index) > cum {
        symbol_index += 1;
    }
    symbol_index
}

fn next_symbol_index(
    iter: &mut Box<impl Iterator<Item = bool>>,
    frequencies: &mut Frequencies,
    low: &mut usize,
    high: &mut usize,
    code_value: &mut usize,
) -> usize {
    let range = *high - *low + 1;
    let total = frequencies.total();
    let cum = ((*code_value - *low + 1) * total - 1) / range;

    let symbol_index = get_symbol_index(frequencies, cum);
    let symbol_low = frequencies.low(symbol_index);
    let symbol_high = frequencies.high(symbol_index);
    *high = *low + range * symbol_high / total - 1;
    *low += range * symbol_low / total;

    loop {
        if *high < CODE_VALUE_HALF {
        } else if *low >= CODE_VALUE_HALF {
            *code_value -= CODE_VALUE_HALF;
            *low -= CODE_VALUE_HALF;
            *high -= CODE_VALUE_HALF;
        } else if *low >= CODE_VALUE_FIRST_QUARTER && *high < CODE_VALUE_THIRD_QUARTER {
            *code_value -= CODE_VALUE_FIRST_QUARTER;
            *low -= CODE_VALUE_FIRST_QUARTER;
            *high -= CODE_VALUE_FIRST_QUARTER;
        } else {
            break;
        }
        *low *= 2;
        *high = 2 * *high + 1;
        slide(iter, code_value);
    }

    symbol_index
}

pub fn decode(data: &[u8]) -> Vec<u8> {
    let mut encoded_data = Box::new(bit_iterator(data));
    let mut decoded_data = Vec::new();
    let mut frequencies = Frequencies::new();
    let mut low = 0;
    let mut high = CODE_VALUE_MAX;
    let mut code_value = 0;

    for _ in 0..CODE_VALUE_BITS {
        slide(&mut encoded_data, &mut code_value);
    }

    loop {
        let symbol_index = next_symbol_index(
            &mut encoded_data,
            &mut frequencies,
            &mut low,
            &mut high,
            &mut code_value,
        );
        if symbol_index == EOF_SYMBOL {
            break;
        }

        let char = frequencies.index_to_char[symbol_index];
        decoded_data.push(char as u8);
        frequencies.update(symbol_index);
    }
    decoded_data
}
