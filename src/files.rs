use crate::http::Methods;
use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct ReqMetadata {
    pub method: Methods,
    pub endpoint: String,
    pub id: String,
}

#[derive(Debug)]
pub struct FileData {
    pub path: String,
    pub name: String,
}

#[derive(Debug)]
pub struct ReqInfo {
    pub metadata: ReqMetadata,
    pub file_data: FileData,
}

pub struct Targets {
    pub param: Option<String>,
    pub config: Option<String>,
}

impl ReqInfo {
    pub async fn read_file(&self) -> Result<String> {
        let file_content = tokio::fs::read_to_string(&self.file_data.path).await?;
        Ok(file_content)
    }

    pub async fn move_to_folder(&self, folder: &str) -> Result<()> {
        let mut new_path = self.file_data.path.replace(&self.file_data.name, "");

        new_path.push_str(folder);
        new_path.push_str(&self.file_data.name);

        tokio::fs::rename(&self.file_data.path, new_path).await?;

        Ok(())
    }
}

fn create_dirs(target: &String) -> Result<()> {
    fs::create_dir_all(target.to_string() + "/error")?;
    fs::create_dir_all(target.to_string() + "/success")?;
    Ok(())
}

fn select_target(targets: Targets) -> Result<String> {
    if let Some(target) = targets.param {
        if Path::new(&target).exists() {
            Ok(target)
        } else {
            Err(anyhow!("Could not find `{}`. Use a valid path.", target))
        }
    } else if let Some(target) = targets.config {
        if Path::new(&target).exists() {
            Ok(target)
        } else {
            Err(anyhow!("Could not find `{}`. Use a valid path.", target))
        }
    } else {
        Err(anyhow!("Could not find path. Use a valid path."))
    }
}

pub struct Files {
    target: String,
    bindinds: HashMap<String, String>,
}

impl Files {
    pub fn new(targets: Targets, bindinds: HashMap<String, String>) -> Result<Files> {
        let target = select_target(targets)?;
        create_dirs(&target)?;
        Ok(Files { target, bindinds })
    }

    pub fn list(&self) -> Result<Vec<ReqInfo>> {
        let files = fs::read_dir(&self.target)?;

        Ok(files
            .par_bridge()
            .filter_map(|file| -> Option<ReqInfo> {
                match file {
                    Ok(file) => {
                        if file.file_type().unwrap().is_file() {
                            let parsed_file = self.parse_file(&file);
                            match parsed_file {
                                Ok(file) => Some(file),
                                Err(e) => {
                                    log::error!(
                                        "{}: {}",
                                        file.file_name().to_str().unwrap_or("Unkown file name"),
                                        e
                                    );
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            })
            .collect())
    }

    fn parse_file(&self, file: &fs::DirEntry) -> Result<ReqInfo> {
        let name = file.file_name().to_str().unwrap().to_owned();
        self.validate_file_name(&name)?;

        let parameters = name.split('_').collect::<Vec<&str>>();
        self.validate_parameters(&parameters)?;

        let method = self.extract_method(parameters[0])?;
        let endpoint = self.extract_endpoint(parameters[1])?;

        let metadata = ReqMetadata {
            method,
            endpoint,
            id: self.extract_id(&parameters),
        };

        let file_data = FileData {
            path: file.path().to_str().unwrap().to_owned(),
            name: file.file_name().to_str().unwrap().to_owned(),
        };

        Ok(ReqInfo {
            metadata,
            file_data,
        })
    }

    fn validate_file_name(&self, name: &str) -> Result<()> {
        if name.ends_with(".json") {
            Ok(())
        } else {
            Err(anyhow!("Invalid file name `{}`", name))
        }
    }

    fn validate_parameters(&self, parameters: &Vec<&str>) -> Result<()> {
        if (3..=4).contains(&parameters.len()) {
            Ok(())
        } else {
            Err(anyhow!("Invalid parameters"))
        }
    }

    fn extract_method(&self, method: &str) -> Result<Methods> {
        match method {
            "GET" => Ok(Methods::GET),
            "POST" => Ok(Methods::POST),
            "PUT" => Ok(Methods::PUT),
            "DELETE" => Ok(Methods::DELETE),
            _ => Err(anyhow!("Invalid method `{}`", method)),
        }
    }

    fn extract_endpoint(&self, key: &str) -> Result<String> {
        if self.bindinds.contains_key(key) {
            Ok(self.bindinds.get(key).unwrap().to_owned())
        } else {
            Err(anyhow!("Invalid bindind `{}`", key))
        }
    }

    fn extract_id(&self, parameters: &Vec<&str>) -> String {
        let mut id = String::from("");

        if parameters.len() == 4 {
            id = parameters[2].to_string();
        };

        id
    }
}
