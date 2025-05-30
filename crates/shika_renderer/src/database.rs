use serde::Serialize;

#[derive(Serialize)]
pub struct Database {
    pub tables: Vec<Table>,
}

#[derive(Serialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Serialize)]
pub struct Column {
    pub name: String,
    pub kind: String,
    pub required: bool,
    pub referenced_by: Vec<Reference>,
    pub references: Option<Reference>,
    pub is_primary_key: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct Reference {
    table: String,
    column: String,
}

impl From<shika_database::Database> for Database {
    fn from(db: shika_database::Database) -> Self {
        let tables = db
            .tables
            .into_iter()
            .map(|t| Table {
                name: t.name,
                columns: t
                    .columns
                    .into_iter()
                    .map(|c| Column {
                        name: c.name,
                        kind: from_kind(&c.kind, c.required),
                        is_primary_key: c.is_primary_key,
                        required: c.required,
                        referenced_by: c
                            .referenced_by
                            .into_iter()
                            .map(|r| Reference {
                                table: r.table,
                                column: r.column,
                            })
                            .collect(),
                        references: c.references.map(|r| Reference {
                            table: r.table,
                            column: r.column,
                        }),
                    })
                    .collect(),
            })
            .collect();

        Database { tables }
    }
}

fn from_kind(kind: &str, is_required: bool) -> String {
    let mut output = String::new();

    if !is_required {
        output.push_str("Option<");
    }

    let kind = match kind {
        "int4" => "i32".to_string(),
        "int8" => "i64".to_string(),
        "float4" => "f32".to_string(),
        "float8" => "f64".to_string(),
        "numeric" => "f64".to_string(),
        "date" => "chrono::NaiveDate".to_string(),
        "timestamp" | "timestamptz" | "timestamp with time zone" => {
            "chrono::DateTime<chrono::FixedOffset>".to_string()
        }
        "varchar" | "text" => "String".to_string(),
        "bool" => "bool".to_string(),
        "uuid" => "uuid::Uuid".to_string(),
        _ => kind.to_string(),
    };

    output.push_str(&kind);

    if !is_required {
        output.push('>');
    }

    output
}
