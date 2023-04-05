# SurrealDB + Rocket CRUD

A little project with the base implementation of a RESTful CRUD using SurrealDB and the Rocket Framework.

### Dependencies

* SurrealDB: [`SurrealDB`](https://github.com/surrealdb/surrealdb)
* Rocket: [`Rocket`](https://github.com/SergioBenitez/Rocket)

### Using

First open the SurrealDB instance with
- ```sh
    surreal start --log debug --user surreal_test --pass surreal_test file:surreal_test.db
    ```

Then build and run the API with:
- ```sh
    cargo run
    ```

And access the following address in a browser or request test framework:
- ```sh
    localhost:8080/people
    ```
    
The available HTTP methods for the endpoints are: GET, POST, PUT, DELETE.