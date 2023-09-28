use crate::{rows::Row, tables::{Table, TABLE_MAX_ROWS}};
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

pub enum ExecuteResult {
    ExecuteTableFull,
    ExecuteSuccess,
    None
}

impl Statement {

    pub fn execute_statement(&self, table: Table) -> ExecuteResult {

        match self.statement_type {
            StatementType::StatementInsert => {
                // println!("This is where we'd do an insert");
                self.execute_insert(table)
            },
            StatementType::StatementSelect => {
                ExecuteResult::None
            },
            StatementType::None => ExecuteResult::None
        }
    }

    pub fn execute_insert(&self, mut table: Table) -> ExecuteResult {
        if table.num_rows >= TABLE_MAX_ROWS {
            return ExecuteResult::ExecuteTableFull
        }
        let row_to_insert = &self.row_to_insert;
        let destination = table.row_slot(table.num_rows);

        println!("{:?}", destination.len());
        let _ = row_to_insert.serialize_row(destination);
        table.num_rows += 1;

        ExecuteResult::ExecuteSuccess
    }
}