use tera::Context;
use crate::models::templates::*;
use crate::config::{load_config, get_service_config_value};

pub fn setup_base_template_context(service_name: String) -> Context {

  let config = load_config().unwrap();

  let mut context = Context::new();
  let mut services: Vec<Service> = Vec::new();

  for (k, v) in config {
    if k != "fll-scoring" {
      services.push(Service { name: k.clone().split("_").collect::<Vec<&str>>().first().unwrap().to_string().clone(), base_url: get_service_config_value(&k, "base-url", true).unwrap()});
    }
  }
  context.insert("services", &services);

  context
}
