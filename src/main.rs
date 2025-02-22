mod gen_code;
mod brain_fetch_interpreter;
use crate::brain_fetch_interpreter::BrainFetchInterpreter as BFInterpreter;


#[tokio::main]
async fn main() {
    let program_string = "";
    let mut interpreter = BFInterpreter::new(&*program_string);
    println!("{}", &program_string);
    interpreter.run().await;
}
