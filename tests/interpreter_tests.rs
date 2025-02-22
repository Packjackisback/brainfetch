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
