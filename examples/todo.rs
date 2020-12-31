use std::path::PathBuf;
use structopt::StructOpt;
use uuid::Uuid;

use tiny_text_data_base::Database;
use tiny_text_data_base::Value;

#[derive(StructOpt)]
#[structopt(about = "Example of a todo")]
enum Cmd {
    List {},
    Add { text: String },
    //Edit { part_uuid: String, new_text: String },
}

#[derive(StructOpt)]
#[structopt(about = "Example of a todo")]
struct Opt {
    #[structopt(parse(from_os_str), default_value = "todo-db", short, long)]
    path: PathBuf,
    #[structopt(subcommand)]
    cmd: Cmd,
}

pub fn main() {
    let opt = Opt::from_args();

    const TABLE: &str = "todolist";

    let mut db = Database::new(&opt.path).unwrap();

    // db.table is declaration that these columns should exist.
    // this is like create table + migration rolled in one.
    // 'id' is automatically added and maintained.
    db.table(TABLE, &vec!["text"]).unwrap();

    // get index of each column we are intereted.
    let indices = db.get_indices(TABLE, &vec!["id", "text"]).unwrap();
    let (id, text) = (indices[0], indices[1]);

    let listfn = |db: &Database| {
        let rows: Vec<(Uuid, String)> = db
            .rows(TABLE)
            .unwrap()
            .iter()
            .map(|row| (row[id].as_pkey().unwrap(), row[text].as_string().unwrap()))
            .collect();
        for row in rows {
            println!("{} -- {}", row.0, row.1);
        }
    };

    let addfn = |db: &mut Database, val: &str| {
        db.insert(TABLE, &vec![(text, Value::new_string(val))])
            .unwrap();
    };

    match opt.cmd {
        Cmd::List {} => listfn(&db),
        Cmd::Add { text } => addfn(&mut db, &text),
        //Cmd::Edit { part_uuid, new_text, } => (),
    }
}
