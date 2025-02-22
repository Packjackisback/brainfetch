mod gen_code;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write, Read};
use reqwest;
use reqwest::header::{HeaderMap, HeaderName};
use csv::Writer;
use crate::gen_code::string_to_brainfuck;

struct BrainfuckInterpreter {
    memory: HashMap<i64, u8>,
    current: i64,
    instruction_current: usize,
    program: Vec<char>,
    bracket_pairs: HashMap<usize, usize>,
}

impl BrainfuckInterpreter {
    fn new(program: &str) -> Self {
        let program: Vec<char> = program.chars().collect();
        let bracket_pairs = Self::build_bracket_pairs(&program);
        Self {
            memory: HashMap::new(),
            current: 0,
            instruction_current: 0,
            program,
            bracket_pairs,
        }
    }

    fn build_bracket_pairs(program: &[char]) -> HashMap<usize, usize> {
        let mut bracket_pairs = HashMap::new();
        let mut stack = Vec::new();

        for (instruction_current, &cmd) in program.iter().enumerate() {
            match cmd {
                '[' => stack.push(instruction_current),
                ']' => {
                    if let Some(start) = stack.pop() {
                        bracket_pairs.insert(start, instruction_current);
                        bracket_pairs.insert(instruction_current, start);
                    } else {
                        panic!("unmatched ] at {}", instruction_current);
                    }
                }
                _ => {}
            }
        }

        if !stack.is_empty() {
            panic!("unmatched [ at {:?}", stack);
        }

        bracket_pairs
    }

    fn print_memory(&self) {
        println!("Memory Contents:");
        println!("{:<10} | {:<15} | {}", "Index", "Value (ASCII)", "Character");
        println!("{:-<10}+-{:-<15}-+{:-<15}", "", "", "");

        let mut indices: Vec<i64> = self.memory.keys().copied().collect();
        indices.sort();

        for index in indices {
            let value = self.memory.get(&index).unwrap_or(&0);
            println!("{:<10} | {:<15} | {}", index, value, *value as char);
        }

        let start_index = 0;
        let end_index = self.current + 10;

        for i in start_index..end_index {
            if !self.memory.contains_key(&i) {
                println!("{:<10} | {:<15} | {}", i, 0, ' ');
            }
        }

    }

    async fn fetch_from_api(&mut self, url_cell: i64) {
        let url_pair = self.read_string_from_memory(url_cell);
        let url = url_pair.0;
        let headers = self.read_headers_from_memory(url_pair.1+2);

        if url.is_empty() {
            eprintln!("Invalid URL in memory cell {}", url_cell);
            return;
        }
        println!("URL: {}", url);
        println!("Headers: {:?}", headers);
        let client = reqwest::Client::new();
        let mut request = client.get(&url);
        let headermap = Self::vec_to_headermap(headers);
        request = request.headers(headermap.clone());

        match request.send().await {
            Ok(response) => {
                if let Ok(body) = response.text().await {
                    self.store_string_in_memory(body);
                }
            }
            Err(e) => eprintln!("Failed to fetch URL {}: {}", url, e),
        }
    }

    fn vec_to_headermap(vec: Vec<(String, String)>) -> HeaderMap {
        let mut headers = HeaderMap::new();

        for (key, value) in vec {
            if let Ok(header_name) = key.parse::<HeaderName>() {
                if let Ok(header_value) = value.parse() {
                    headers.insert(header_name, header_value);
                }
            }
        }

        headers
    }

    fn read_string_from_memory(&self, start: i64) -> (String, i64) {
        let mut s = String::new();
        let mut i = start;
        while let Some(&byte) = self.memory.get(&i) {
            if byte == 0 {
                break;
            }
            s.push(byte as char);
            i += 1;
        }
        (s, i)
    }

    fn read_headers_from_memory(&self, start: i64) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        let mut i = start;

        while let Some(&byte) = self.memory.get(&i) {
            if byte == 0 {
                break;
            }
            let key = self.read_string_from_memory(i).0;
            i += key.len() as i64 + 1; // Move past the key
            let value = self.read_string_from_memory(i).0;
            i += value.len() as i64 + 1; // Move past the value
            headers.push((key, value));
        }
        headers
    }

    fn store_string_in_memory(&mut self, data: String) {
        for (i, byte) in data.bytes().enumerate() {
            self.memory.insert(self.current + i as i64, byte);
        }
        self.memory.insert(self.current + data.len() as i64, 0);

    }
    fn dump_memory_to_csv(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = Writer::from_writer(file);
        writer.write_record(&["Index", "Value (Decimal)", "Character"])?;
        let mut indices: Vec<i64> = self.memory.keys().copied().collect();
        indices.sort();
        for index in indices {
            let value = self.memory.get(&index).unwrap_or(&0);
            let character = if *value >= 32 && *value <= 126 {
                (*value as char).to_string()
            } else {
                "".to_string()
            };
            writer.write_record(&[
                index.to_string(),
                value.to_string(),
                character,
            ])?;
        }

        writer.flush()?;
        Ok(())
    }


    async fn run(&mut self) {
        while self.instruction_current < self.program.len() {
            // println!(
            //     "Instruction: {:?}, Current Index: {}, Value: {}, Memory: {:?}",
            //     self.program[self.instruction_current],
            //     self.current,
            //     self.memory.get(&self.current).unwrap_or(&0),
            //     self.memory
            // );
            match self.program[self.instruction_current] {
                '>' => { self.current += 1; }
                '<' => { self.current -= 1; }
                '+' => { *self.memory.entry(self.current).or_insert(0) = self.memory.get(&self.current).unwrap_or(&0).wrapping_add(1); }
                '-' => { *self.memory.entry(self.current).or_insert(0) = self.memory.get(&self.current).unwrap_or(&0).wrapping_sub(1); }
                '.' => { print!("{}", *self.memory.get(&self.current).unwrap_or(&0) as char); }
                '%' => { println!("{}", *self.memory.get(&self.current).unwrap_or(&0)); }
                ',' => { let mut input = [0; 1]; io::stdin().read_exact(&mut input).expect("bad input"); self.memory.insert(self.current, input[0]); }
                '@' => { self.fetch_from_api(self.current).await; }
                '[' => { if *self.memory.get(&self.current).unwrap_or(&0) == 0 { self.instruction_current = self.bracket_pairs[&self.instruction_current]; } }
                ']' => { if *self.memory.get(&self.current).unwrap_or(&0) != 0 { self.instruction_current = self.bracket_pairs[&self.instruction_current]; } }
                '~' => {self.print_memory()}
                '_' => {print!("\nCurrent index is: {}\n", self.current)}
                '|' => {self.current -= *self.memory.get(&self.current).unwrap_or(&0) as i64}
                '*' => {self.current += *self.memory.get(&self.current).unwrap_or(&0) as i64}
                '#' => {self.current = 0}
                '!' => {
                    if let Err(e) = self.dump_memory_to_csv("memory_dump.csv") {
                        eprintln!("Failed to dump memory to CSV: {}", e);
                    } else {
                        println!("Memory dumped to \"memory_dump.csv\"");
                    }
                }
                _ => {}
            }
            self.instruction_current += 1;
        }
    }
}

#[tokio::main]
async fn main() {
    let program_string = format!(r#"
    {}
    >>
    {}
    >
    {}
    #
    ~
    @
    #
    [.>]
    ~

    >>
    ++++++++
    [<++++++++++++++++>-]<
    *
    {}
    >>>>>>{}
    <_|_
    +++++++++++++
    [<+++++++++++++>-]<
    %|<
    _
    %
    .
    <  now at 23
    _
    %
    [-]
    %
    <
    _
    %
    -
    [>++++++++<-]>+++++++ now at 23 with what should be 255
    * jumps to 278
    >>>>>>>>>>>>>>>>>>>>>>>>>>>>> 307
    +++++++++++++++
    [<+++++++++++++++++>-]< 306 should now hold 255 as well so we can jump back with it
    %
    _
    |
    ~
    _ 51 now need to go to 24 so 27
    <<<< now at 47 which holds 32
    | jump back to 15 need to go to 24
    >>>>>>>>> now at 24 ready to copy
    [<*>>>>>>>>>>>>>>>>>+>>>>>>>>>>>|<<<<|>>>>>>>>>-] should in theory move 8 to in the correct spot
    > onto 25
    [<<*>>>>>>>>>>>>>>>>>>+>>>>>>>>>>|<<<<|>>>>>>>>>>-] should move the number in 25
    >
    [<<<*>>>>>>>>>>>>>>>>>>>+>>>>>>>>>|<<<<|>>>>>>>>>>>-] should move the number in 26
    > onto 27 yippee
    [<<<<*>>>>>>>>>>>>>>>>>>>>+>>>>>>>>|<<<<|>>>>>>>>>>>>-] should move the number in 27
    > onto 28 yippee 2 left guys
    [<<<<<*>>>>>>>>>>>>>>>>>>>>>+>>>>>>>|<<<<|>>>>>>>>>>>>>-] should move the number in 28
    > onto 29 last one
    [<<<<<<*>>>>>>>>>>>>>>>>>>>>>>+>>>>>>|<<<<|>>>>>>>>>>>>>>-]
    wait we are done just need to remove the far jump and then fetch and then repeat everything again
    <<<<<<* to 278
    >>>>>>>>>>>>>>>>>>>>>>>>>>>> _ should be at 306
    [-] set to zero time to add headers yay
    >>
    {}
    >
    {}
    <<<<<<<<<| back to 251
    >>>>>>
    ~
    @
    ~!
    "#, string_to_brainfuck("http://security.mercurywork.shop/api"), string_to_brainfuck("AUTHENTICATION"), string_to_brainfuck("bob"), string_to_brainfuck("http://security.mercurywork.shop/api/"),string_to_brainfuck(".json"), string_to_brainfuck("AUTHENTICATION"), string_to_brainfuck("bob"));
    let mut interpreter = BrainfuckInterpreter::new(&*program_string);
    interpreter.run().await;
}
