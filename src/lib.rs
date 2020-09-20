#[macro_use] extern crate log;
use std::char::from_u32;
use log::Level;

const RIGHT: &'static str = "right";
const LEFT: &'static str = "left";
const ASCII_BITS_PER_CHAR: usize = 8;

fn shift_binary_string(bin_str: String, i: i32, direction: &str) -> String {

    let i1 = i as usize;

    return if direction.to_lowercase().as_str() == RIGHT {
        let x: String = bin_str.chars().take(bin_str.len() - i1).collect();
        format!("{}{}", "0".repeat(i1), x)
    } else if direction.to_lowercase().as_str() == LEFT {
        format!("{}{}", bin_str, "0".repeat(i1))
    }
    else {
        println!("bad direction: {}", direction);
          bin_str
    };
}

fn binary_string_to_ascii(mut s: String) -> String {
    if s.len() % ASCII_BITS_PER_CHAR != 0 {
        let padded_zeros_cnt = ASCII_BITS_PER_CHAR - s.len() % ASCII_BITS_PER_CHAR;
        s = format!("{}{}", "0".repeat(padded_zeros_cnt).to_string(), s);
    }

    let chars: Vec<char> = (0..s.len())
        .step_by(ASCII_BITS_PER_CHAR)
        .map(|i| {
            let int = u8::from_str_radix(&s[i..i + ASCII_BITS_PER_CHAR], 2);
            let i1 = int.unwrap() as u32;
            let char = from_u32(i1).unwrap();
            return char;
        })
        .collect();

    let mut my_str = String::from("");

    for x in chars {
        my_str.push(x);
    }

    return my_str;
}

fn shift_to_ascii(bin_str: String, direction: &str, shift_cnt: i32) -> String {
    init();
    let string = shift_binary_string(bin_str, shift_cnt, direction);
    let string1 = binary_string_to_ascii(string);
    string1
}

fn init(){
}

#[cfg(test)]
mod tests {
    use crate::shift_to_ascii;

    #[test]
    fn a_test() {
        let mut bin_str = String::new();
        bin_str.push_str("1100001");
        let string1 = shift_to_ascii(bin_str.to_string(), "right", 0);

        assert_eq!("a", string1);
    }

    #[test]
    fn cap_a_test() {
        let mut bin_str = String::new();
        bin_str.push_str("1000001");
        let string1 = shift_to_ascii(bin_str.to_string(), "right", 0);

        assert_eq!("A", string1);
    }

    #[test]
    fn it_works() {
        let mut bin_str = String::new();
        bin_str.push_str(
            "00101000000010000010111101100001101001011101000101101110100001100011111101101010",
        );
        bin_str.push_str("1010001111000111101000110010000001000000010010001000011111011101001111101001110111101010");
        bin_str.push_str("0110011110010111100011011110001010000010001100110110000101011001110101010001011101001001");
        bin_str.push_str("0110000011011110001010110011001011111001110010011101100011110000110111111001000011010101");
        bin_str.push_str("0100000000101000111110101000111001111100111000010001000100110");

        for i in 0..20 {
            let string1 = shift_to_ascii(bin_str.to_string(), "right", i);

            println!("_____________'string' = '{}'_____________", string1);
        }
    }
}
