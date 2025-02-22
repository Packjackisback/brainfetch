use std::collections::HashMap;
use std::io::{self, Read};
use reqwest;
use reqwest::header::{HeaderMap, HeaderName};

pub struct BrainFetchInterpreter {
    memory: HashMap<i64, u8>,
    current: i64,
    instruction_current: usize,
    program: Vec<char>,
    bracket_pairs: HashMap<usize, usize>,
}

impl BrainFetchInterpreter {
    pub fn new(program: &str) -> Self {
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
    pub async fn run(&mut self) -> String {
        let mut output = String::new();
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
                '.' => { output.push(*self.memory.get(&self.current).unwrap_or(&0) as char); }
                ',' => { let mut input = [0; 1]; io::stdin().read_exact(&mut input).expect("bad input"); self.memory.insert(self.current, input[0]); }
                '@' => { self.fetch_from_api(self.current).await; }
                '[' => { if *self.memory.get(&self.current).unwrap_or(&0) == 0 { self.instruction_current = self.bracket_pairs[&self.instruction_current]; } }
                ']' => { if *self.memory.get(&self.current).unwrap_or(&0) != 0 { self.instruction_current = self.bracket_pairs[&self.instruction_current]; } }
                '|' => {self.current -= *self.memory.get(&self.current).unwrap_or(&0) as i64}
                '*' => {self.current += *self.memory.get(&self.current).unwrap_or(&0) as i64}
                '#' => {self.current = 0}
                _ => {}
            }
            self.instruction_current += 1;
        }
        output
    }
}
