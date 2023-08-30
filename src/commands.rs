use crate::tables::Row;
pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

#[derive(Debug)]
pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
    PrepareSyntaxError
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
    None
}

pub struct Statement {
    pub statement_type: StatementType,
    pub row_to_insert: Row
}

impl Statement {

    pub fn execute_statement(&self) {

        match self.statement_type {
            StatementType::StatementInsert => {
                println!("This is where we'd do an insert");
            },
            StatementType::StatementSelect => {
                println!("This is where we'd do a select");
            }
            _ => ()
        }
    }
}