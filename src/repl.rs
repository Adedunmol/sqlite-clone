use std::{io::{self, Write}, process};
use crate::{Result, commands::{MetaCommandResult, Statement, StatementType, PrepareResult}};

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

        if bytes_read == 0 {
            eprintln!("Error reading input\n");
            process::exit(1);
        }

        self.input_length = bytes_read - 1;

        Ok(())
    }

    pub fn get_buffer(&self) -> &str {

        self.buffer.trim()
    }

    pub fn do_meta_command(&self) -> MetaCommandResult {
        if self.get_buffer() == ".exit" {

            process::exit(0);
        } else {

            MetaCommandResult::MetaCommandUnrecognizedCommand
        }
    }

    pub fn prepare_statement(&self, statement: &mut Statement) -> PrepareResult {

        if self.get_buffer().starts_with("insert") {
            statement.statement_type = StatementType::StatementInsert;
            let input = self.get_buffer();

            match sscanf::sscanf!(input, "insert {usize} {str} {str}") {
                Ok((id, username, email)) => {
                    statement.row_to_insert.id = id as u32;
                    // statement.row_to_insert.username = username.to_string();
                    match statement.row_to_insert.set_username(username.to_string()) {
                        Ok(()) => (),
                        Err(err) => { 
                            println!("Error: {}", err);
                            return PrepareResult::PrepareSyntaxError
                        }
                    }

                    // statement.row_to_insert.email = email.to_string();
                    match statement.row_to_insert.set_email(email.to_string()) {
                        Ok(()) => (),
                        Err(err) => { 
                            println!("Error: {}", err);
                            return PrepareResult::PrepareSyntaxError
                        }
                    }
                    
                },
                Err(_err) => { 
                    return PrepareResult::PrepareSyntaxError
                }
            } 

            return PrepareResult::PrepareSuccess;
        }
        if self.get_buffer() == "select" {
            statement.statement_type = StatementType::StatementSelect;

            return PrepareResult::PrepareSuccess;
        }

        PrepareResult::PrepareUnrecognizedStatement      
    }
}