use super::GCloudFactory;

pub mod table;

pub use table::{Table, TableMetadata};

pub type CrudResult<T> = Result<T, Box<dyn std::error::Error>>;


pub struct BigQueryClient {
    gcloud_factory: Box<GCloudFactory>,
    project_id: String,
}

impl BigQueryClient {

    pub fn new(gcloud_factory: Box<GCloudFactory>, project_id: &str) -> BigQueryClient {
       
        BigQueryClient{
            gcloud_factory: gcloud_factory,
            project_id: project_id.to_owned(),
        }
    }

    pub fn table(&self, dataset_id: &str, name: &str) -> Table {
        let gcloud = (self.gcloud_factory)();
        Table::new(gcloud, self.project_id.as_str(), dataset_id, name)
    }

}   