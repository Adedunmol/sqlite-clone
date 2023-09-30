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

    pub fn execute_statement(&self, mut table: &mut Table) -> ExecuteResult {

        match self.statement_type {
            StatementType::StatementInsert => {
                println!("This is where we are doing an insert");
                self.execute_insert(&mut table)
            },
            StatementType::StatementSelect => {
                println!("This is where we are doing a select");
                self.execute_select(&mut table)
            },
            StatementType::None => ExecuteResult::None
        }
    }

    pub fn execute_insert(&self, mut table: &mut Table) -> ExecuteResult {
        if table.num_rows >= TABLE_MAX_ROWS {
            return ExecuteResult::ExecuteTableFull
        }
        let row_to_insert = &self.row_to_insert;
        let destination = table.row_slot(table.num_rows);

        let _ = row_to_insert.serialize_row(destination);
        table.num_rows += 1;

        ExecuteResult::ExecuteSuccess
    }

    pub fn execute_select(&self, mut table: &mut Table) -> ExecuteResult {
        println!("{}", table.num_rows);
        let mut row = Row::new(1, "", "").unwrap();
        for row_num in 0..table.num_rows {
            row.deserialize_row(table.row_slot(row_num));
            println!("{:#?}", row);
        }

        println!("{:?}", table.pages);

        ExecuteResult::ExecuteSuccess
    }
}