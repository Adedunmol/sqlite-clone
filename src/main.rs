use std::process;

use commands::Statement;
use repl::InputBuffer;

mod repl;
mod commands;

pub type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        input_buffer.buffer.clear();

        InputBuffer::print_prompt();
        
        input_buffer.read_input().unwrap_or_else(|err| {
            println!("An error occurred while reading from stdin: {}", err);
            process::exit(1);
        });

        if input_buffer.get_buffer().starts_with(".") {
            match input_buffer.do_meta_command() {
                commands::MetaCommandResult::MetaCommandSuccess => {
                    continue;
                },
                commands::MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized keyword at the start of \"{}\".", input_buffer.get_buffer());
                    continue;
                }
            }
        }

        let mut statement = Statement { statement_type: commands::StatementType::None };
        let result = input_buffer.prepare_statement(&mut statement);
        
        if let commands::PrepareResult::PrepareSuccess = result {
            // continue;
        } else if let commands::PrepareResult::PrepareUnrecognizedStatement = result {
            println!("Unrecognized keyword at the start of \"{}\".", input_buffer.get_buffer());
            continue;
        }

        statement.execute_statement();
        println!("Executed.");
    }
}
