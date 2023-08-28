use std::process;

use repl::InputBuffer;

mod repl;

pub type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        InputBuffer::print_prompt();
        
        input_buffer.buffer.clear();

        input_buffer.read_input().unwrap_or_else(|err| {
            println!("An error occurred while reading from stdin: {}", err);
            process::exit(1);
        });

        if input_buffer.get_buffer() == ".exit" {

            process::exit(0);
        } else {
            println!("Unrecognized command \"{}\".\n", input_buffer.get_buffer());
        }
    }
}
