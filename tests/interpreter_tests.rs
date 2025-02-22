use brainfetch_lib::BrainFetchInterpreter;
use std::io::{self, Write, Cursor};
use std::sync::{Arc, Mutex};
use std::io::Read;
#[tokio::test]
async fn test_interpreter1() {
    let program = "++++++[>++++++++<-]>.";
    let mut interpreter = BrainFetchInterpreter::new(program);
    assert_eq!(interpreter.run().await, "0");
}

#[tokio::test]
async fn test_input1() {
    let input_data = "Hello";
    let input_cursor = Cursor::new(input_data.as_bytes());
    let stdin = Arc::new(Mutex::new(input_cursor));
    let mut interpreter = BrainFetchInterpreter::new(",[.,]").with_stdin(stdin.clone());
}
