use std::path::PathBuf;
use uuid::Uuid;

use crate::Database;
use crate::Value;

// not exported to users of library, but needed for testing.
use crate::consts::*;

#[test]
fn test_database() {
    let mut db = Database::new(&PathBuf::from("/tmp/ttdb-test")).unwrap();

    assert_eq!(
        db.table("table", &vec!["col1", PRIMARY_KEY_NAME]),
        Err(ERROR_COLUMN_ID)
    );
    assert_eq!(
        db.table("table", &vec!["col1", "col1"]),
        Err(ERROR_COLUMN_DUP)
    );
    db.table("table", &vec!["col1", "col2"]).unwrap();
    assert_eq!(
        db.table("table", &vec!["col1", "col2"]),
        Err(ERROR_TABLE_DUP)
    );

    let indices = db
        .get_indices("table", &vec!["col2", "id", "col1"])
        .unwrap();
    let (col2, id, col1) = (indices[0], indices[1], indices[2]);
    // although 0,1,2 is not guaranteed, this check is easier.
    assert_eq!(0, id);
    assert_eq!(1, col1);
    assert_eq!(2, col2);
}

#[test]
fn test_insert() {
    let mut db = Database::new(&PathBuf::from("/tmp/ttdb-test")).unwrap();
    db.table("table", &vec!["col1", "col2"]).unwrap();
    let indices = db
        .get_indices("table", &vec!["col2", "id", "col1"])
        .unwrap();
    let (_col2, id, _col1) = (indices[0], indices[1], indices[2]);

    assert_eq!(
        db.insert("non_existing_table", &vec![(0 as usize, Value::Null)]),
        Err(ERROR_TABLE_NOTFOUND)
    );
    assert_eq!(
        db.insert("table", &vec![(id, Value::Null)]),
        Err(ERROR_COLUMN_ID)
    );
    assert_eq!(
        db.insert("table", &vec![(3, Value::Null)]),
        Err(ERROR_COLUMN_NOTFOUND)
    );
}

#[test]
fn test_select() {
    let mut db = Database::new(&PathBuf::from("/tmp/ttdb-test")).unwrap();
    db.table("table", &vec!["col1", "col2"]).unwrap();
    let indices = db
        .get_indices("table", &vec!["col2", "id", "col1"])
        .unwrap();
    let (col2, id, col1) = (indices[0], indices[1], indices[2]);
    db.insert(
        "table",
        &vec![
            (col1, Value::new_integer(100)),
            (col2, Value::new_string("hello")),
        ],
    )
    .unwrap();
    db.insert("table", &vec![(col1, Value::new_integer(200))])
        .unwrap();
    db.insert("table", &vec![(col2, Value::new_string("hello"))])
        .unwrap();

    let mut rowref: Vec<(Uuid, Option<i64>)> = db
        .rows("table")
        .unwrap()
        .iter()
        .filter(|&row| row[col2].string_equals("hello"))
        .map(|row| (row[id].as_pkey().unwrap(), row[col1].as_int()))
        .collect();
    rowref.sort_by_key(|v| v.1);

    assert_eq!(2, rowref.len());
    assert_eq!(None, rowref[0].1);
    assert_eq!(Some(100), rowref[1].1);
    eprint!("\nDB is {:#?}\n", rowref);
}
