use std::collections::HashMap;
/// A transaction has a date stamp, and list of statements
/// T1he uuid is only to generate a uniqud name.
/// although when reading, we don't check if filename matches the uuid
///
/// We need surprisingly few types of statements.
use uuid::Uuid;

use crate::value::Value;

/// A statement is a DDL/DML operation.

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Statement {
    /// Table is created if not already exists.
    /// This doesn't enforce ( or knows about) the id field
    ColumnCreate { table: String, columns: Vec<String> },

    /// if 'id' is present, it's a update, else it's a insert.
    RowUpsert {
        table: String,
        values: HashMap</*column_name: */ String, /*value: */ Value>,
    },

    /// delete row by the id.
    RowDelete {
        table: String,
        key: Uuid, // or CellValue, with enforced PrimaryKey?
    },
}

/// Transaction file has a UUID, which is the name of file
/// and several independent, Transactions.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Transaction {
    uuid: Uuid,
    epoch: u64,
    statements: Vec<Statement>,
}

impl Transaction {
    pub fn sort_key(&self) -> u64 {
        self.epoch
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}
