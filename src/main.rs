use std::process;

use commands::{Statement, ExecuteResult};
use repl::InputBuffer;

use crate::rows::Row;

mod repl;
mod commands;
mod rows;
mod tables;

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
            println!("working");
            // continue;
        } else if let commands::PrepareResult::PrepareUnrecognizedStatement = result {
            println!("Unrecognized keyword at the start of \"{}\".", input_buffer.get_buffer());
            continue;
        } else if let commands::PrepareResult::PrepareSyntaxError = result {
            println!("Syntax error. Could not parse statement.\r");
            continue;
        }

        let table = tables::Table::new();

        let statement_result = statement.execute_insert(table);

        if let ExecuteResult::ExecuteSuccess = statement_result {
            println!("Executed\r\n");
            break;
        }

        // match statement.execute_statement() {
        //     ExecuteResult::ExecuteSuccess => {
        //         println!("Executed\r\n");
        //         break;
        //     },
        //     ExecuteResult::ExecuteTableFull => {

        //     }
        // }
        // println!("Executed.");
    }
}
