use std::fmt::Display;

use crate::{DATABASE_FILE_PATH, Result, error::Error};
use tokio::runtime::Runtime;

pub fn command<D: Display>(database_url: D) -> Result<()> {
    let Ok(runtime) = Runtime::new() else {
        return Err(Error::RuntimeInitializationFailed);
    };

    run_blocking(runtime, database_url.to_string())?;

    Ok(())
}

fn run_blocking(runtime: Runtime, database_url: String) -> Result<()> {
    runtime.block_on(async {
        // Load the workspace
        let workspace = shika_workspace::Workspace::load()?;

        // Load the database
        let database = shika_database::Database::fetch(
            &database_url,
            workspace
                .config
                .exclude_tables
                .clone()
                .unwrap_or(Vec::new()),
        )
        .await?;

        workspace.write(DATABASE_FILE_PATH, database)?;

        // Print a success message
        println!("Successfully pulled the latest changes from the database.");

        Ok(())
    })
}
