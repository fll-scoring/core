use handlebars::{Handlebars, Registry};

pub fn init_handlebars() -> Result<Registry, String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".html", "./static/templates")?;

    Ok(handlebars.clone())

}
