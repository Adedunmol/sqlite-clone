use crate::{commands::{StatementType, ExecuteResult}, rows::Row, tables::{Table, TABLE_MAX_ROWS}};




pub struct Statement {
    pub statement_type: StatementType,
    pub row_to_insert: Row
}

impl Statement {

    pub fn execute_statement(&self, table: &mut Table) -> ExecuteResult {

        match self.statement_type {
            StatementType::StatementInsert => {
                // println!("This is where we are doing an insert");
                self.execute_insert(table)
            },
            StatementType::StatementSelect => {
                // println!("This is where we are doing a select");
                self.execute_select(table)
            },
            StatementType::None => ExecuteResult::None
        }
    }

    pub fn execute_insert(&self, table: &mut Table) -> ExecuteResult {
        if table.num_rows >= TABLE_MAX_ROWS {
            return ExecuteResult::ExecuteTableFull
        }
        let row_to_insert = &self.row_to_insert;
        let destination = table.row_slot(table.num_rows);

        let _ = row_to_insert.serialize_row(destination);
        table.num_rows += 1;

        ExecuteResult::ExecuteSuccess
    }

    pub fn execute_select(&self, table: &mut Table) -> ExecuteResult {

        let mut row = Row::new(1, "", "").unwrap();
        for row_num in 0..table.num_rows {
            let source = table.row_slot(row_num);
            let new_row = row.deserialize_row(source).unwrap();
            println!("{}", new_row);
        }

        // println!("{:?}", table.pages);

        ExecuteResult::ExecuteSuccess
    }
}