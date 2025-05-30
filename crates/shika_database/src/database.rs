use std::{fs::File, path::Path};

use futures::TryStreamExt;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, query_as};

use crate::{Result, error::Error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub tables: Vec<Table>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub name: String,
    pub kind: String,
    pub required: bool,
    pub referenced_by: Vec<Reference>,
    pub references: Option<Reference>,
    pub is_primary_key: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Reference {
    pub table: String,
    pub column: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct TableMetadata {
    name: String,
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct ColumnMetadata {
    name: String,
    kind: String,
    is_primary_key: bool,
    optional: bool,
}

#[derive(sqlx::FromRow, Debug, Clone)]
struct ColumnReferenceMetadata {
    column: String,
    table: String,
}

impl Database {
    pub async fn fetch(database_url: &str, ignore: Vec<String>) -> Result<Self> {
        let connection = Pool::<Postgres>::connect(database_url)
            .await
            .map_err(Error::Connection)?;

        let mut tables_stream = query_as::<_, TableMetadata>(
            r#"
                SELECT
                    "table_name" AS "name"
                FROM "information_schema"."tables"
                WHERE
                    "table_type" = 'BASE TABLE'
                    AND "table_schema" = $1
                    AND NOT ("table_name" = ANY($2))
            "#,
        )
        .bind("public")
        .bind(ignore)
        .fetch(&connection);

        let mut tables: Vec<Table> = Vec::new();
        while let Some(table) = tables_stream.try_next().await? {
            let mut columns_stream = query_as::<_, ColumnMetadata>(
                r#"
                    SELECT
                        C."column_name" AS "name",
                        C."data_type" AS "kind",
                        CASE
                            WHEN C."is_nullable" = 'YES' THEN TRUE
                        ELSE
                            FALSE
                        END AS "optional",
                        CASE
                            WHEN TC."constraint_type" = 'PRIMARY KEY' THEN TRUE
                        ELSE
                            FALSE
                        END AS "is_primary_key",
                        TC.*
                    FROM "information_schema"."columns" AS C
                    LEFT JOIN "information_schema"."key_column_usage" AS KCU
                        ON C."column_name" = KCU."column_name"
                        AND C."table_name" = KCU."table_name"
                    LEFT JOIN "information_schema"."table_constraints" AS TC
                        ON TC.table_name = KCU.table_name
                        AND TC.constraint_catalog = KCU.constraint_catalog
                        AND TC.constraint_schema = KCU.constraint_schema
                        AND TC.constraint_name = KCU.constraint_name
                    WHERE
                        (TC."constraint_type" IS NULL OR TC."constraint_type" != 'FOREIGN KEY')
                        AND C."table_name" = $1
                "#,
            )
            .bind(&table.name)
            .fetch(&connection);

            let mut columns: Vec<Column> = Vec::new();
            while let Some(column) = columns_stream.try_next().await? {
                // Query for all columns referencing this column.
                let referenced_by = query_as::<_, ColumnReferenceMetadata>(
                    r#"
                        SELECT
                            REFBY."column_name" AS "column",
                            REFBY."table_name" AS "table"
                        FROM "information_schema"."referential_constraints" AS RC
                        INNER JOIN "information_schema"."key_column_usage" AS REFBY
                            ON RC."constraint_name" = REFBY."constraint_name"
                        INNER JOIN "information_schema"."key_column_usage" AS REFTO
                            ON RC."unique_constraint_name" = REFTO."constraint_name"
                        WHERE REFTO."column_name" = $1 AND REFTO."table_name" = $2
                    "#,
                )
                .bind(&column.name)
                .bind(&table.name)
                .fetch_all(&connection)
                .await?;

                // Query for all columns referenced by this column.
                let references = query_as::<_, ColumnReferenceMetadata>(
                    r#"
                        SELECT
                            REFTO."column_name" AS "column",
                            REFTO."table_name" AS "table"
                        FROM "information_schema"."referential_constraints" AS RC
                        INNER JOIN "information_schema"."key_column_usage" AS REFBY
                            ON RC."constraint_name" = REFBY."constraint_name"
                        INNER JOIN "information_schema"."key_column_usage" AS REFTO
                            ON RC."unique_constraint_name" = REFTO."constraint_name"
                        WHERE REFBY."column_name" = $1 AND REFBY."table_name" = $2
                    "#,
                )
                .bind(&column.name)
                .bind(&table.name)
                .fetch_optional(&connection)
                .await?;

                columns.push(Column {
                    name: column.name,
                    kind: column.kind,
                    is_primary_key: column.is_primary_key,
                    required: !column.optional,
                    referenced_by: referenced_by
                        .into_iter()
                        .map(|r| Reference {
                            table: r.table,
                            column: r.column,
                        })
                        .collect(),
                    references: references.map(|r| Reference {
                        table: r.table,
                        column: r.column,
                    }),
                });
            }

            tables.push(Table {
                name: table.name.clone(),
                columns,
            });
        }

        Ok(Database { tables })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Option<Self>> {
        let Ok(file) = File::open(path).map_err(Error::IO) else {
            return Ok(None);
        };

        let database = serde_yml::from_reader(file).map_err(Error::InvalidDatabaseFile)?;

        Ok(database)
    }
}
