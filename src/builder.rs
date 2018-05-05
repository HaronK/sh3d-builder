use failure;
use model;

pub fn build(home: model::Home) -> Result<String, failure::Error> {
    Ok(format!("{:?}", home))
}
