use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

pub struct Targets<'a> {
    pub param: Option<String>,
    pub config: &'a Option<String>,
}

pub fn select_target(targets: Targets) -> Result<String> {
    if let Some(target) = targets.param {
        if Path::new(&target).exists() {
            Ok(target)
        } else {
            Err(anyhow!("Could not find `{}`. Use a valid path.", target))
        }
    } else if let Some(target) = &targets.config {
        if Path::new(&target).exists() {
            Ok(target.to_string())
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
