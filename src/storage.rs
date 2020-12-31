/// bridges the in-memory data structures to on-disk transaction files.
/// panics if any corruption is detected
/// as trying to recover will lead to loss of data.
use log::warn;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::table::Table;
use crate::transaction::Statement;
use crate::transaction::Transaction;
use crate::value::Value;

fn read_txns_sorted(path: &PathBuf) -> Vec<Transaction> {
    let mut transactions = vec![];

    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if let Some(extn) = entry.path().extension() {
                // skip ., .. etc.
                if extn != "txn" {
                    warn!("Unknown file ignored : {}", entry.path().to_str().unwrap());
                } else {
                    let file = File::open(path).unwrap();
                    let reader = BufReader::new(file);
                    let txn: Transaction = serde_json::from_reader(reader).unwrap();
                    transactions.push(txn);
                    //TODO:
                    // if txn.sort_key() in check_dupkey,
                    //   throw error of duplicate key
                    // add key to check_dupkey
                }
            }
        }
    }

    transactions.sort_unstable_by_key(|t| t.sort_key());
    transactions
}

fn column_create(tables: &mut HashMap<String, Table>, table: &str, columns: &[String]) {
    match tables.get_mut(table) {
        Some(tref) => tref.add_columns(&columns),
        None => {
            let t = Table::new(columns);
            tables.insert(table.to_string(), t);
        }
    }
}

fn row_upsert(
    tables: &mut HashMap<String, Table>,
    table: &str,
    values: &HashMap</*column_name: */ String, /*value: */ Value>,
) {
    match tables.get_mut(table) {
        Some(tref) => tref.upsert(&values),
        None => panic!("insert/update in non-existing table"),
    }
}

fn row_delete(tables: &mut HashMap<String, Table>, table: &str, key: &uuid::Uuid) {
    match tables.get_mut(table) {
        Some(tref) => tref.delete(&key),
        None => panic!("delete in non-existing table"),
    }
}

fn reduce(tables: &mut HashMap<String, Table>, statement: &Statement) {
    match statement {
        Statement::ColumnCreate { table, columns } => column_create(tables, table, &columns),
        Statement::RowUpsert { table, values } => row_upsert(tables, table, &values),
        Statement::RowDelete { table, key } => row_delete(tables, table, key),
    }
}

pub fn read_db(path: &PathBuf) -> HashMap<String, Table> {
    let txns = read_txns_sorted(path);

    let mut tables = HashMap::new();

    for txn in txns.iter() {
        for stmt in txn.statements() {
            reduce(&mut tables, stmt);
        }
    }

    tables
}
