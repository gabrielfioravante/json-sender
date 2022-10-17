use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

pub struct Targets<'a> {
    pub param: Option<String>,
    pub config: &'a Option<String>,
}

pub fn select_target<'a>(param_target: &'a Option<String>, config_target: &'a Option<String>) -> Result<&String> {
    if let Some(target) = param_target {
        if Path::new(&target).exists() {
            Ok(target)
        } else {
            Err(anyhow!("Could not find `{}`. Use a valid path.", target))
        }
    } else if let Some(target) = config_target {
        if Path::new(&target).exists() {
            Ok(target)
        } else {
            Err(anyhow!("Could not find `{}`. Use a valid path.", target))
        }
    } else {
        Err(anyhow!("Could not find path. Use a valid path."))
    }
}

pub fn create_dirs(target: &String) -> Result<()> {
    fs::create_dir_all(format!("{}/success", target))?;
    fs::create_dir_all(format!("{}/error", target))?;
    fs::create_dir_all(format!("{}/response", target))?;
    Ok(())
}
