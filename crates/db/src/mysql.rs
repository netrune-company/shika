use sqlx::{MySql, Pool};

use crate::Database;

pub async fn connect(url: &str) -> anyhow::Result<Pool<MySql>> {
    let pool = Pool::<MySql>::connect(url).await?;

    Ok(pool)
}

pub async fn get(_pool: &Pool<MySql>) -> anyhow::Result<Database> {
    todo!()
}
