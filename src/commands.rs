
pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
    None
}

pub struct Statement {
    pub statement_type: StatementType 
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