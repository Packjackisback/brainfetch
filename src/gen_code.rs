#[allow(dead_code)]
pub fn string_to_brainfuck(input: &str) -> String {
    let mut brainfuck_code = String::new();

    for c in input.chars() {
        let ascii_value = c as u8;

        if ascii_value > 0 {
            brainfuck_code.push_str(&"+" .repeat(ascii_value as usize));
        }
        brainfuck_code.push('>');
    }

    brainfuck_code
}

