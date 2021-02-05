use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TableMetadata {
	pub schema: Option<Vec<SchemaField>>,
	pub time_partitioning: TimePartitioning,
	clustering: Clustering
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaField {
	name: String,
	
	#[serde(rename="type")]
	_type: FieldType,
	mode: Option<FieldMode>,
	description: String,
	policy_tags: Vec<String>,
	fields: Option<Vec<SchemaField>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE", untagged)]
pub enum FieldMode {
	Nullable,
	Required,
	Repeated,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE", untagged)]
pub enum FieldType {
	String,
	Bytes,
	Integer,
	Float,
	Boolean,
	Timestamp,
	Date,
	Time,
	DateTime,
	Geography,
	Numeric,
	BigNumeric,
	Struct
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimePartitioning {
	#[serde(rename="type")]
	_type: String,
	expirationMs: String,
	field: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clustering {
	fields: Vec<String>,
}
