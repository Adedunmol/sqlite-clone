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

pub enum ExecuteResult {
    ExecuteTableFull,
    ExecuteSuccess,
    None
}
