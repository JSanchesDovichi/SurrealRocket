use figment::Figment;
use figment::providers::Format;
use figment::providers::Toml;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use rocket::fairing::AdHoc;

use crate::conn_config::Connection;

async fn get_surreal_connection() -> Surreal<Client>{
	let Some(configuration) = Figment::new().merge(Toml::file("Database.toml")).extract::<Connection>().ok() else {
		println!("Failed to read database connection file. Check it and try again!");

		std::process::exit(0);
	};

	let host_address = format!("{}:{}", configuration.address.host, configuration.address.port);

	let Ok(db) = Surreal::new::<Ws>(host_address).await else {
		println!("Unable to connect to Database!");

        std::process::exit(0);
	};

	if let Err(e) = db.signin(Root {
		username: &configuration.user.username,
		password: &configuration.user.password,
	})
	.await {
		println!("DB Auth error: {e}");
	}

	if let Err(e) = db.use_ns(configuration.database.namespace).use_db(configuration.database.database).await {
		println!("DB Selection error: {}", e);
	}

	return db;
}

pub fn init_surreal_state_handler() -> AdHoc {
    AdHoc::on_ignite("Connection to SurrealDB", |rocket| async {
        rocket.manage(get_surreal_connection().await)
    })
}