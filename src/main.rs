mod people;
mod database;
mod conn_config;

#[rocket::main]
pub async fn main() {
	let database_handler = database::init_surreal_state_handler();

    let server_instance = rocket::build()
		.attach(database_handler)
		.mount("/people", people::routes());

    if let Err(e) = server_instance.launch().await {
        println!("Server initialization failed: {e}");
    }
}