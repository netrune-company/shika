mod filter;
mod model;

pub use model::{Column, Database, Table};

use tera::{Context, Tera};

pub fn render(
    path: &str,
    tera: &Tera,
    database: &db::Database,
    table: Option<&db::Table>,
) -> anyhow::Result<String> {
    let mut context = Context::new();
    context.insert("database", &database);

    if let Some(table) = table {
        context.insert("table", &table);
    }

    Ok(tera.render(&format!("{path}.shika.tera"), &context)?)
}

pub fn create() -> anyhow::Result<Tera> {
    let mut tera = Tera::new(".shika/templates/**/*.tera")?;

    tera.register_filter("not_primary", filter::not_primary);
    tera.register_filter("primary", filter::primary);
    tera.register_filter("foreign", filter::foreign);
    tera.register_filter("snake_to_pascal", filter::snake_to_pascal);

    Ok(tera)
}

pub fn render_name(
    name: &String,
    tera: &mut Tera,
    database: &db::Database,
    table: &db::Table,
) -> anyhow::Result<String> {
    let mut context = Context::new();
    context.insert("database", &database);
    context.insert("table", &table);

    let output = tera.render_str(name, &context)?;

    Ok(output)
}
