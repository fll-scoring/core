use super::config::get_global_value;
use bson::doc;
use mongodb::{options::ClientOptions, Client, Collection, Database};
use std::env;

/// Returns a mongo client according to the configured mongo url
pub async fn get_mongo_client() -> Result<Client, Box<dyn std::error::Error>> {
    let mongo_url = get_global_value("mongo-url", true)?;
    let client_options = ClientOptions::parse(&mongo_url).await?;

    let client = Client::with_options(client_options)?;

    Ok(client)
}

/// Returns either the Mongo database from the client returned by get_mongo_client() or the Error.
pub async fn get_mongo_database() -> Result<Database, Box<dyn std::error::Error>> {
    let client = get_mongo_client().await?;

    let mongo_database = get_global_value("mongo-database", true)?;

    let database = client.database(&mongo_database);

    Ok(database)
}

/// Gets the next number in a mongo sequence
pub async fn get_next_mongo_sequence_number(
    name: String,
    counter_collection: Collection,
) -> Result<i32, Box<dyn std::error::Error>> {
    let sequence_document = match counter_collection
        .find_one_and_update(
            doc! {"_id": name.clone()},
            doc! {"$inc": {"sequence_value": 1}},
            None,
        )
        .await
    {
        Ok(opt) => match opt {
            Some(doc) => doc,
            None => {
                counter_collection
                    .insert_one(doc! {"_id": name, "sequence_value": 0}, None)
                    .await?;
                doc! {"sequence_value": 0}
            }
        },
        Err(err) => {
            return Err(Box::new(err));
        }
    };
    let sequence_value = sequence_document.get_i32("sequence_value")?;
    Ok(sequence_value)
}
