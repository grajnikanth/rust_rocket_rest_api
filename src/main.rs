// Tools for using decorator
// procedureal macros are being used
#![feature(proc_macro_hygiene, decl_macro)]

// All the macros and decorators from rocket shall be imported into this project
// imports the rocket macros globally and can be used anywhere in our application
#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;
use rusqlite::Connection


// serialize by serde library will allow you to convert a struct to a json
// deserialize will allow you to convert a json back to this struct
// derive macro gives the struct on which it acts implementation functions on this 
// struct which are pre-generated for us. So it eliminates our writing of these 
// implementation functions ourselves
#[derive(Serialize)]
struct ToDoItem {
    id: i64, // i64 compatible with sqlite integers
    item: String
}

#[derive(Serialize)]
struct ToDoList {
    items: Vec<ToDoItem>
}

// used for sending messages to user
#[derive(Serialize)]
struct StatusMessage {
    message: String
}


// we are using the get() function provided by rocket with the argument "/"
// the function index
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/todo")]
// In this function we will take care of error handling instead of just using unwrap and panic
// For this function, we are going to return error as a String as implied by the 
// second argument in the Result. 
// First one is Json from Rocket_contrib in Result OK()
fn fetch_all_todo_items() -> Result<Json<ToDoList>, String> {
    
}

fn main() {

    // sqlite database initialization is kept in a code block so that at the end
    // of the code block the variables associated with the database are dropped
    {
        // Create a database using rusqlite library
        let db_connection = Connection::open("data.sqlite").unwrap();

        // using sql connection create a table
        db_connection.execute("create table if not exists todo_list
            (
                id integer primary key,
                item varchar(64) not null
            );", 
            rusqlite::NO_PARAMS)
            .unwrap();
    }
    

    rocket::ignite().mount("/", routes![index]).launch();
}