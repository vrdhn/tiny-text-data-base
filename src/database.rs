use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::table::Table;
use crate::value::Value;

use crate::consts::*;

use crate::storage;

#[derive(Debug)]
pub struct Database {
    path: PathBuf,
    tables: HashMap<String, Table>,
}

impl Database {
    /// if path doesn't exists, the directory will be created.
    /// if path exists, the transactions files will be parsed.
    ///
    pub fn new(path: &PathBuf) -> Result<Database, String> {
        // let's make sure the path exists ...
        if !path.exists() {
            match fs::create_dir_all(path) {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            };
        } else if !path.is_dir() {
            return Err(ERROR_NOT_DIRECTORY.to_string());
        }
        // read any transactions which are there
        Ok(Database {
            path: path.clone(),
            tables: storage::read_db(path),
        })
    }

    pub fn table(&mut self, table_name: &str, columns: &[&str]) -> Result<(), &'static str> {
        let mut cols = vec![];
        if self.tables.contains_key(table_name) {
            return Err(ERROR_TABLE_DUP);
        }
        cols.push(PRIMARY_KEY_NAME.to_string());
        for column in columns.iter() {
            if column == &PRIMARY_KEY_NAME {
                return Err(ERROR_COLUMN_ID);
            }
            if cols.iter().any(|i| i == column) {
                return Err(ERROR_COLUMN_DUP);
            }
            cols.push(column.to_string());
        }
        self.tables
            .insert(table_name.to_string(), Table::new(&cols));

        Ok(())
    }

    pub fn insert(
        &mut self,
        table_name: &str,
        values: &[(usize, Value)],
    ) -> Result<(), &'static str> {
        let table = self.tables.get_mut(table_name);
        match table {
            None => Err(ERROR_TABLE_NOTFOUND),
            Some(t) => t.insert(
                Value::PrimaryKey {
                    key: Uuid::new_v4(),
                },
                values,
            ),
        }
    }

    pub fn get_indices(
        &self,
        table_name: &str,
        columns: &[&str],
    ) -> Result<Vec<usize>, &'static str> {
        let table = self.tables.get(table_name);
        match table {
            None => Err(ERROR_TABLE_NOTFOUND),
            Some(t) => t.column2indices(columns),
        }
    }

    pub fn rows(&self, table_name: &str) -> Result<&Vec<Vec<Value>>, &'static str> {
        let table = self.tables.get(table_name);
        match table {
            None => Err(ERROR_TABLE_NOTFOUND),
            Some(t) => t.rows(),
        }
    }
}
