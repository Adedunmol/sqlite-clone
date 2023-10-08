pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand
}

#[derive(Debug)]
pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
    PrepareSyntaxError,
    PrepareStringTooLong,
    PrepareNegativeId
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
