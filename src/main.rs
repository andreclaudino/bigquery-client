mod gcloud;

use gcloud::{GCloud, bigquery::BigQueryClient};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub (crate) struct ScalarMetric {
    pub name: String,
    pub task: String,
    pub scope: String,
    
    #[serde(with = "ts_seconds")]
    pub written_at: DateTime<Utc>,
    pub step: i64,
    pub substep: i64,
    pub value: f64,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
    
    let gcloud_factory = Box::new(||{GCloud::default()});

    let client = BigQueryClient::new(gcloud_factory, project_id.as_str());
    let table = client.table("_temporary", "metrics_sidecar_test");
    
    let row = &ScalarMetric {
        name: "teste".to_owned(),
        task: "task".to_owned(),
        scope: "test".to_owned(),
        written_at: Utc::now(),
        step: 120,
        substep: 0,
        value: 10.0
    };

    let _ = table.insert(row).await;

    println!("Done");
    
    Ok(())
}

