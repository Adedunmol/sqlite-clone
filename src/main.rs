use std::process;

use commands::ExecuteResult;
use repl::InputBuffer;

use crate::{rows::Row, statement::Statement};

mod repl;
mod commands;
mod rows;
mod tables;
mod statement;

pub type Err = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Err>;


fn main() {
    let mut input_buffer = InputBuffer::new();
    let mut table = tables::Table::new();

    loop {
        input_buffer.buffer.clear();

        InputBuffer::print_prompt();
        
        input_buffer.read_input().unwrap_or_else(|err| {
            println!("An error occurred while reading from stdin: {}", err);
            process::exit(1);
        });

        if input_buffer.get_buffer().starts_with('.') {
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

        let test_row = Row::new(1, "", "").unwrap();
        let mut statement = Statement { statement_type: commands::StatementType::None, row_to_insert: test_row };
        let result = input_buffer.prepare_statement(&mut statement);

        if let commands::PrepareResult::PrepareSuccess = result {
            // println!("statement has been parsed successfully");
            // continue;
        } else if let commands::PrepareResult::PrepareUnrecognizedStatement = result {
            println!("Unrecognized keyword at the start of \"{}\".", input_buffer.get_buffer());
            continue;
        } else if let commands::PrepareResult::PrepareSyntaxError = result {
            println!("Syntax error. Could not parse statement.\r");
            continue;
        } else if let commands::PrepareResult::PrepareStringTooLong = result {
            println!("String too long.\r");
            continue;
        } else if let commands::PrepareResult::PrepareNegativeId = result {
            println!("ID must be positive.\r");
            continue;
        }

        match statement.execute_statement(&mut table) {
            ExecuteResult::ExecuteSuccess => {
                println!("Executed.\r");
                continue;
            },
            ExecuteResult::ExecuteTableFull => {
                println!("Error: Table full.\r");
                break;
            },
            ExecuteResult::None => ()
        }
        println!("Executed.");
    }
}
