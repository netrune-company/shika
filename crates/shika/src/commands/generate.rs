use crate::{DATABASE_FILE_PATH, Result, error::Error};
use shika_database::Database;
use shika_renderer::Renderer;
use shika_workspace::Workspace;

pub fn command(template_name: Option<String>) -> Result<()> {
    // Load the workspace
    let workspace = Workspace::load()?;
    let config = workspace.config.clone();
    let renderer = Renderer::new(&workspace)?;

    let Some(database) = workspace.read::<Database, _>(DATABASE_FILE_PATH)? else {
        return Err(Error::DatabaseNotPulled);
    };

    if let Some(template_name) = template_name {
        let Some(template) = config.templates.get(&template_name) else {
            return Err(crate::error::Error::TemplateNotFound(template_name));
        };

        match renderer.render(&template.path, &database.into()) {
            Ok(output) => {
                if let Err(error) = workspace.write(&template.output, output) {
                    eprintln!("Could not write template to file: {error}");
                }
            }
            Err(error) => {
                eprintln!("Could not render template at {}: {}", template.path, error);
            }
        }
    } else {
        config.templates.iter().for_each(|(_, template)| {
            let database = database.clone();

            match renderer.render(&template.path, &database.into()) {
                Ok(output) => {
                    if let Err(error) = workspace.write_file(&template.output, &output) {
                        eprintln!("Could not write template to file: {error}");
                    }
                }
                Err(error) => {
                    eprintln!("Could not render template at {}: {}", template.path, error);
                }
            }
        });
    }

    // Print a success message
    println!("Successfully generated the project from the template.");

    Ok(())
}
