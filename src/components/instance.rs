use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstanceToml {
    pub instance: Instance,
}

#[derive(Debug, Deserialize)]
pub struct Instance {
    pub name: String,
    pub slogan: String,
    pub url: String,
    pub api_version: String,
    pub donation_url: String,
    pub source_code_url: String,
    pub documentation_url: String,
    pub logo_name: String,
    pub no_logo_text: bool,
    pub logo_text_name: String,
    pub logo_width: i32,
    pub logo_height: i32,
    pub favicon_name: String,
}

pub fn get_instance_details() -> Result<Instance, Box<dyn std::error::Error>> {
    let toml_str = include_str!("../../Instance.toml");
    let instance_toml: InstanceToml = toml::from_str(toml_str)?;
    Ok(instance_toml.instance)
}
