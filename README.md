# tiny-text-data-base


# What is it

A table-column oriented, flat text files database, which ensures
that files are never modified. 

This design permits
* version control: never conflicts
* easy merge: just copy directory over another.
* backup / storage: just copy directory on to other.

# Features
* Has tables, columns, transactions
* Very ACIDic
* Loads complete database in memory.
* Compact database to reduce number of files


# Design

Database is stored as a series of transactions, each in an file.
On startup, all transactions are read, sorted, and reduced
to build the database.

Queries work directly from the in-memory data structure.
Any changes are done in a transaction, which is written to 
a temporary file, and on commit, moved in as dbtxn file.

Transaction files are dumps of simple data structures, in 
yaml, although it can be json, xml or anything else.

Database compaction involved merging several transactions in a single
transaction, and deleting those files. However even if the 
extra files are not deleted, they will be ignored.


# Current limitations
* no high level SQL like thing ; use rust functionalities.



# Example
Look in `src/example/todo.rs` for a simple example.

```
alias todo='cargo run --example todo -- '
## on machine 1, there is a todo list
todo --path p1  list
todo --path p1  add 'task 1 in p2'
todo --path p1  list

## on machine 2, another todo list
todo --path p2  list
todo --path p2  add 'task 1 in p1'
todo --path p2  list

## to merge, just copy all the files over ..
cp p2/* p1/
todo --path p1  list

```



