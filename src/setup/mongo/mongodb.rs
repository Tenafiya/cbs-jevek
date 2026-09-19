use config::Config;
use mongodb::{Client, Collection, options::ClientOptions};
use std::time::Duration;

use crate::app::{
    amls::mongo_model::AmlActionTriggerModel,
    transactions::mongo_model::DepositTransactionMongoModel,
};

#[derive(Clone, Debug)]
pub struct MongoDatabase {
    pub aml_actions: Collection<AmlActionTriggerModel>,
    pub mongo_transactions: Collection<DepositTransactionMongoModel>,
}

impl MongoDatabase {
    pub async fn connector(settings: &Config) -> Self {
        let app_name = settings.get::<String>("mongo.app_name").unwrap();
        let connect_timeout = settings.get::<u64>("mongo.connect_timeout").unwrap();
        let server_selection_timeout = settings
            .get::<u64>("mongo.server_selection_timeout")
            .unwrap();
        let max_pool_size = settings.get::<u32>("mongo.max_pool_size").unwrap();
        let min_pool_size = settings.get::<u32>("mongo.min_pool_size").unwrap();
        let max_idle_time = settings.get::<u64>("mongo.max_idle_time").unwrap();
        let database_name = settings.get::<String>("mongo.database").unwrap();

        let db_env_check = settings.get::<String>("app.environment").unwrap();

        let mongo_uri = match db_env_check.as_str() {
            "TEST" => std::env::var("MONGO_URI_TEST").expect("Cannot Find MONGO URI"),
            _ => std::env::var("MONGO_URI_PROD").expect("Cannot Find MONGO URI"),
        };

        let mut client_options = ClientOptions::parse(mongo_uri)
            .await
            .expect("Failed to parse MongoDB URI");

        client_options.app_name = Some(app_name);
        client_options.connect_timeout = Some(Duration::new(connect_timeout, 0));
        client_options.server_selection_timeout = Some(Duration::new(server_selection_timeout, 0));
        client_options.max_pool_size = Some(max_pool_size);
        client_options.min_pool_size = Some(min_pool_size);
        client_options.max_idle_time = Some(Duration::new(max_idle_time, 0));

        let client = Client::with_options(client_options).expect("Failed to init MongoDB client");

        let db = client.database(&database_name);

        let aml_actions: Collection<AmlActionTriggerModel> = db.collection("aml_actions");
        let mongo_transactions: Collection<DepositTransactionMongoModel> =
            db.collection("transactions");

        MongoDatabase {
            aml_actions,
            mongo_transactions,
        }
    }
}
