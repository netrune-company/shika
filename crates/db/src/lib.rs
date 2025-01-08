use serde::{Deserialize, Serialize};

pub mod mysql;
pub mod postgres;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum DatabaseKind {
    #[serde(rename = "postgres")]
    Postgres,
    #[serde(rename = "mysql")]
    MySql,
    #[serde(rename = "sqlite")]
    SqLite,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Database {
    #[serde(rename = "type")]
    pub kind: DatabaseKind,
    pub tables: Vec<Table>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub is_primary_key: bool,
    pub optional: bool,
    pub referenced_by: Vec<Reference>,
    pub references: Option<Reference>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Reference {
    table: String,
    column: String,
}
