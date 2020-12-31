use crate::consts::*;
use crate::value::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug)]
pub struct Table {
    schema: Vec<String>,
    schema_index: HashMap<String, usize>,
    value2d: Vec<Vec<Value>>,
}

impl Table {
    pub fn new(columns: &[String]) -> Table {
        let mut schema_index = HashMap::new();

        for (index, element) in columns.iter().enumerate() {
            schema_index.insert(element.clone(), index);
        }
        // is assured by Database, so panic okay here.
        assert_eq!(schema_index["id"], 0 as usize);

        Table {
            schema: columns.to_vec(),
            schema_index: schema_index,
            value2d: vec![],
        }
    }

    /// gets indices of columns, disallow duplicates , allow "id"
    pub fn column2indices(&self, columns: &[&str]) -> Result<Vec<usize>, &'static str> {
        let mut failed = false;
        let mut done = HashSet::new();

        let res: Vec<usize> = columns
            .iter()
            .map(|&c| match self.schema_index.get(c) {
                None => {
                    failed = true;
                    usize::MAX
                }
                Some(v) => {
                    done.insert(*v);
                    *v
                }
            })
            .collect();
        if failed {
            Err(ERROR_COLUMN_NOTFOUND)
        } else if done.len() != res.len() {
            Err(ERROR_COLUMN_DUP)
        } else {
            Ok(res)
        }
    }

    pub fn insert(
        &mut self,
        primary_key: Value,
        values: &[(usize, Value)],
    ) -> Result<(), &'static str> {
        if !primary_key.is_primary_key() {
            return Err(INTERNAL_INVALID_PRIMARY_KEY);
        }

        let mut row: Vec<Value> = self.schema.iter().map(|_| Value::Null).collect();

        for (colidx, value) in values {
            // PRIMARY_KEY_NAME
            let colidx = *colidx;
            if colidx == 0 as usize {
                return Err(ERROR_INSERT_ID);
            }
            if colidx >= row.len() {
                return Err(ERROR_COLUMN_NOTFOUND);
            }
            if !row[colidx].is_null() {
                return Err(ERROR_INSERT_DUPCOLUMN);
            }
            row[colidx] = value.clone();
        }
        row[PRIMARY_KEY_IDX] = Value::PrimaryKey {
            key: Uuid::new_v4(),
        };
        self.value2d.push(row);
        Ok(())
    }
    pub fn rows(&self) -> Result<&Vec<Vec<Value>>, &'static str> {
        Ok(&self.value2d)
    }

    /// Add column(s), and add Null as value for that column
    /// panics on dup column, because it's a programming bug.
    pub fn add_columns(&mut self, columns: &[String]) {
        for column in columns {
            if self.has_column(column) {
                panic!("duplicate columns in adding new column")
            }
        }
        let sz = columns.len();
        self.schema.extend_from_slice(columns);

        let nulls = vec![Value::Null; sz];

        for row in &mut self.value2d {
            row.extend_from_slice(&nulls);
        }
    }

    pub fn upsert(&mut self, values: &HashMap</*column_name: */ String, /*value: */ Value>) {
        if values.contains_key(PRIMARY_KEY_NAME) {
        } else {
        }
    }
    pub fn delete(&mut self, key: &uuid::Uuid) {}

    fn has_column(&self, column: &str) -> bool {
        self.schema.contains(&column.to_string())
    }
}
