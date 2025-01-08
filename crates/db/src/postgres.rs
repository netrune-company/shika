use anyhow::anyhow;
use futures::TryStreamExt;
use sqlx::{query_as, FromRow, Pool, Postgres};

use crate::{Column, Database, DatabaseKind, Reference, Table};

pub async fn connect(url: &str) -> anyhow::Result<Pool<Postgres>> {
    Pool::<Postgres>::connect(url)
        .await
        .map_err(|error| anyhow!(error))
}

#[derive(FromRow, Debug)]
struct TableQuery {
    pub name: String,
}

#[derive(FromRow, Debug)]
struct ColumnQuery {
    name: String,
    kind: String,
    is_primary_key: bool,
    optional: bool,
}

#[derive(FromRow, Debug)]
struct ColumnReferenceQuery {
    column: String,
    table: String,
}

pub async fn get(pool: &Pool<Postgres>, ignore: Vec<String>) -> anyhow::Result<Database> {
    let mut tables_stream = query_as::<_, TableQuery>(
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
    .fetch(pool);

    let mut tables: Vec<Table> = Vec::new();
    while let Some(table) = tables_stream.try_next().await? {
        let mut columns_stream = query_as::<_, ColumnQuery>(
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
        .fetch(pool);

        let mut columns: Vec<Column> = Vec::new();
        while let Some(column) = columns_stream.try_next().await? {
            // Query for all columns referencing this column.
            let referenced_by = query_as::<_, ColumnReferenceQuery>(
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
            .fetch_all(pool)
            .await?;

            // Query for all columns referenced by this column.
            let references = query_as::<_, ColumnReferenceQuery>(
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
            .fetch_optional(pool)
            .await?;

            columns.push(Column {
                name: column.name,
                kind: column.kind,
                is_primary_key: column.is_primary_key,
                optional: column.optional,
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

    Ok(Database {
        kind: DatabaseKind::Postgres,
        tables,
    })
}
