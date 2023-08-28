use std::{io::{self, Write}, process};
use crate::{Result};

pub struct InputBuffer {
    pub buffer: String,
    buffer_length: usize,
    input_length: usize,
}

impl InputBuffer {

    pub fn new() -> InputBuffer {

        InputBuffer {
            buffer: String::new(),
            buffer_length: 0,
            input_length: 0
        }
    }

    pub fn print_prompt() {
        print!("db > ");
        io::stdout().flush().unwrap();
    }

    pub fn read_input(&mut self) -> Result<()> {
        let bytes_read = io::stdin().read_line(&mut self.buffer)?;

        if bytes_read <= 0 {
            eprintln!("Error reading input\n");
            process::exit(1);
        }

        self.input_length = bytes_read - 1;

        Ok(())
    }

    pub fn get_buffer(&self) -> &str {

        &self.buffer.trim()
    }

}