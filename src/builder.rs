use failure;
use input_desc;
use output_desc;

pub fn build(home: input_desc::Home) -> Result<output_desc::Home, failure::Error> {
    Ok(output_desc::Home { desc: format!("{:?}", home) })
}
