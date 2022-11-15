// Tools for using decorator
// procedureal macros are being used
#![feature(proc_macro_hygiene, decl_macro)]

// All the macros and decorators from rocket shall be imported into this project
// imports the rocket macros globally and can be used anywhere in our application
#[macro_use] extern crate rocket;
use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;
use rusqlite::Connection;


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

    // Rocket is multi threaded and will not panic if panic occurs on one thread. Only 
    // that particular thread will crash if panic occurs 
    // so ok to use unwrap. But we want to handle errors so we can handle the response
    // to user
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    // Once we get a database connection, we can use it to query the database
    let mut statement = match db_connection.prepare("select id, item from todo_list") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare a query".into())
    };

    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        // Checking to see if ? operator returns Ok(T) or T back with this row.get(0)? command
        // println!("The row.get(0) is {}", row.get(0)?);
        Ok(ToDoItem {
            // the ? will return an error to propagate if there was an issue with the reading of database
            // also ? will return an error if the types do not match that is Rust know id is an integer but
            // if sql returns a string an error is propagated back.
            id: row.get(0)?, 
            item: row.get(1)?
        })
    });

    // results will be an iterator per rusqlite documentation
    // for result in results {
    //     println!("id and item in rows are {} and {}", result.get(0)?, result.get(1)?);
    // }

    // Since match is the last block here and without a semicolon so we are 
    // returning here.
    match results {
        Ok(rows) => {
            // Vec<ToDoItem> because in the above we said the rows returned are mapped to the ToDoItem struct
            // Take all the rows collected and put it into a vector of ToDoItems using the collect() function
            // Since results are Result<> type, the collect() function can return a Result<Collection<T>>. T in this case we are saying is 
            // ToDoItem struct
            let collection: rusqlite::Result<Vec<ToDoItem>> = rows.collect();

            // vector of ToDoItem is the ToDoList we defined. So we are take the items which in this case will be a vector
            // of ToDoItems and obtain the ToDoList, which we will convert/serialize using the Json function on it. 
            match collection {
                Ok(items) => Ok(Json(ToDoList {items})),
                Err(_) => Err("Could not collect items".into()) 
            }
        }
        Err(_) => Err("Failed to fetch ToDo Items".into())
    }


    // into() function if implemented on the type will return the Type required per the 
    // Return type set on this function. Which in this case if Error occurs shall be
    // a String
    // Err("Unknown Error".into())
}

// format says in what format we are expecting the Post request made in
// data field specifies the variable name we want to use to receive the data sent
#[post("/todo", format = "json", data = "<item>")]
// Rocket will automatically respond with the return type to the client
fn add_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare(
        "insert into todo_list (id, item) values (null, $1)") 
    {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare a query".into())
    };

    // add item to the database table
    // The &[&item.0] - The first "&" is saying that we are passing a reference to a 
    // string slice. The second & is referencing the item.0 value. We are just borrowing
    // the value here
    let results = statement.execute(&[&item.0]);

    match results {
        // the variable rows_added can be named with any name. It just represents the value in Ok(T).
        // That is it represents T which the Result got when the result was successfull and there 
        // were no errors
        Ok(rows_added) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_added),
        })),
        Err(_) => Err("Failed to insert ToDo Item".into())
    }

}

#[delete("/todo/<id>")]
// Rocket will automatically respond with the return type to the client
fn remove_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare(
        "delete from todo_list where id = $1;") 
    {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare a query".into())
    };

    let results = statement.execute(&[&id]);

    match results {
        // the variable rows_added can be named with any name. It just represents the value in Ok(T).
        // That is it represents T which the Result got when the result was successfull and there 
        // were no errors
        Ok(rows_deleted) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted", rows_deleted),
        })),
        Err(_) => Err("Failed to delete ToDo Item".into())
    }

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
    
    // add the function names in the routes! macro to let Rocket open the endpoints
    rocket::ignite().mount("/", routes![
        index, 
        fetch_all_todo_items, 
        add_todo_item,
        remove_todo_item
        ]).launch();
}