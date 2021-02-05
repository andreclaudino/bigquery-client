use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct InsertAll {
    kind: String,
    skip_invalid_rows: bool,
    ignore_unknown_values: bool,
    template_suffix: Option<String>,
    rows: Vec<Row>,
    trace_id: Uuid,
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Row {
    insert_id: Uuid,
    json: serde_json::Value
}

impl Row {
    pub fn new_option(entity: &impl Serialize) -> Option<Row> {
        match serde_json::to_value(entity) {
            Ok(json) => {
                let row = Row {
                    insert_id: Uuid::new_v4(),
                    json
                };

                Some(row)
            },
            _ => None
        }
    }
}

impl InsertAll {
    pub fn new(entities: &Vec<&impl Serialize>) -> InsertAll {
        let rows = 
            entities.iter().filter_map(|entity|{
                Row::new_option(entity)
            }).collect();

        InsertAll {
            kind: "bigquery#tableDataInsertAllRequest".to_owned(),
            skip_invalid_rows: true,
            ignore_unknown_values: true,
            template_suffix: None,
            rows: rows,
            trace_id: Uuid::new_v4(),
        }
    }
}