#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate uuid;

mod consts;

mod storage;
mod transaction;
mod value;

mod database;
mod table;

pub use crate::database::Database;
pub use crate::value::Value;

#[cfg(test)]
mod tests;
