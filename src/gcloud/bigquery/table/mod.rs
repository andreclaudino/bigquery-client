mod insert_entities;
mod create_entities;

use serde::Serialize;
use super::CrudResult;
use crate::gcloud::{GCloud, client::Endpoint};

pub use create_entities::TableMetadata;

use insert_entities::InsertAll;
pub struct Table {
    gcloud_client: GCloud,
    project_id: String,
    dataset_id: String,
    name: String,
    table_metadata: Option<create_entities::TableMetadata>,
}


impl Endpoint for Table {}

impl Table {

    pub async fn insert_many(&self, entities: &Vec<&impl Serialize>) -> CrudResult<()> {
        let header_value = self.gcloud_client.header_value();
        let resource = format!("bigquery/v2/projects/{project_id}/datasets/{dataset_id}/tables/{table_id}/insertAll",
                                     project_id=self.project_id, dataset_id=self.dataset_id, table_id=self.name);
        let endpoint = self.endpoint(resource.as_str());

        let body = InsertAll::new(entities);

        let request_client = reqwest::Client::new();
        let response = request_client
            .post(endpoint.as_str())
            .json(&body)
            .header("Authorization", header_value)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err)),
        }
        
    }

    pub async fn insert(&self, entity: &impl Serialize) -> CrudResult<()> {
        let entities = &vec![entity];
        self.insert_many(entities).await
    }

    pub fn new(gcloud_client: GCloud, project_id: &str, dataset_id: &str, name: &str) -> Table {
        Table {
            project_id: project_id.to_owned(),
            dataset_id: dataset_id.to_owned(),
            name: name.to_owned(),
            gcloud_client,
            table_metadata: None,
        }
    }
}
