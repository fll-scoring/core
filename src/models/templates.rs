use serde::Serialize;

#[derive(Serialize)]
pub struct Service {
  pub name: String,
  pub base_url: String,
}
