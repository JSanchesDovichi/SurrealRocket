use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use rocket::{delete, get, http::Status, post, put, routes, serde::json::Json, Route, State};
use surrealdb::sql::Thing;

pub fn routes() -> Vec<Route> {
    routes![get, post, put, delete]
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    company: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersonWithId {
    id: Thing,
    name: String,
    company: String,
}

static TABLE_NAME: &'static str = "People";

#[get("/")]
async fn get(database: &State<Surreal<Client>>) -> Json<Vec<PersonWithId>> {
    let query = format!("SELECT * FROM {TABLE_NAME}");

    let mut response = database.query(query).await.unwrap();

    let users: Vec<PersonWithId> = response.take(0).unwrap();

    Json(users)
}

#[post("/", data = "<new_person>")]
async fn post(database: &State<Surreal<Client>>, new_person: Json<Person>) -> Status {
    let query = format!("CREATE {TABLE_NAME} SET name = $name, company = $company");

    let Ok(mut results) = database.query(query)
    .bind(new_person.into_inner())
    .await else {
        return Status::InternalServerError;
    };

    if let Ok(Some(_)) = results.take::<Option<Person>>(0) {
        return Status::Created;
    } else {
        return Status::InternalServerError;
    }
}

#[put("/<record_id>", data = "<modification_data>")]
async fn put(
    database: &State<Surreal<Client>>,
    record_id: &str,
    modification_data: Json<Person>,
) -> Status {
    let search_query = format!("SELECT * FROM {TABLE_NAME}:{record_id}");

    let mut response = database.query(search_query).await.unwrap();

    let users: Vec<Person> = response.take(0).unwrap();

    if users.len() == 0 {
        return Status::NotFound;
    }

    let query = format!("UPDATE {TABLE_NAME}:{record_id} SET name = $name, company = $company");

    if let Err(e) = database
        .query(query)
        .bind(modification_data.into_inner())
        .await
    {
        println!("{e}");

        return Status::InternalServerError;
    }

    Status::Ok
}

#[delete("/<record_id>")]
async fn delete(database: &State<Surreal<Client>>, record_id: &str) -> Status {
    let search_query = format!("SELECT * FROM {TABLE_NAME}:{record_id}");

    let mut response = database.query(search_query).await.unwrap();

    let users: Vec<Person> = response.take(0).unwrap();

    if users.len() == 0 {
        return Status::NotFound;
    }

    let sql = format!("DELETE {TABLE_NAME}:{record_id}");

    if let Err(e) = database.query(sql).await {
        println!("{e}");

        return Status::InternalServerError;
    }

    Status::Gone
}
