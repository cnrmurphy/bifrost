use mongodb::{bson::doc, options::ClientOptions, Client, Database};

pub struct DB {
    pub client: Database,
}

impl DB {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        let mongo_uri = std::env::var("MONGO_URL").expect("MONGO_URL must be set");

        let mut client_options = ClientOptions::parse(mongo_uri).await?;
        client_options.app_name = Some("bifrost".to_string());

        let client = match Client::with_options(client_options) {
            Ok(client) => client,
            Err(error) => panic!("error creating MongoDB client: {error:?}"),
        };

        tracing::info!("attempting to establish connection to MongoDB");

        match client
            .database("admin")
            .run_command(doc! { "ping": 1 }, None)
            .await
        {
            Ok(_) => tracing::info!("successfully established connection to MongoDB"),
            Err(error) => panic!("error establishing connection to MongoDB: {error:?}"),
        }

        let db = client.database("bifrost");

        Ok(Self { client: db })
    }
}
