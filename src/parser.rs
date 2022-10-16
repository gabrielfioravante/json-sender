use crate::file::{FileData, FileToSend, RequestData};
use crate::http::Methods;
use anyhow::{anyhow, Result};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Targets {
    pub param: Option<String>,
    pub config: Option<String>,
}

pub struct FileParser {
    target: String,
    bindinds: HashMap<String, String>,
}

impl FileParser {
    pub fn new(targets: Targets, bindinds: HashMap<String, String>) -> Result<FileParser> {
        let target = select_target(targets)?;
        create_dirs(&target)?;
        Ok(FileParser { target, bindinds })
    }

    pub fn list_files(&self) -> Result<Vec<FileToSend>> {
        let files = fs::read_dir(&self.target)?;

        Ok(files
            .par_bridge()
            .filter_map(|file| -> Option<FileToSend> {
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

    fn parse_file(&self, file: &fs::DirEntry) -> Result<FileToSend> {
        let name = file.file_name().to_str().unwrap().to_owned();
        self.validate_file_name(&name)?;

        let parameters = name.split('_').collect::<Vec<&str>>();
        self.validate_parameters(&parameters)?;

        let method = self.extract_method(parameters[0])?;
        let endpoint = self.extract_endpoint(parameters[1])?;

        let request_data = RequestData {
            method,
            endpoint,
            id: self.extract_id(&parameters),
        };

        let data = self.extract_file_data(file)?;

        Ok(FileToSend { request_data, data })
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

    fn extract_file_data(&self, file: &fs::DirEntry) -> Result<FileData> {
        let binding = file.path();
        let path = binding
            .to_str()
            .ok_or_else(|| anyhow!("Unable to get file path"))?;

        let binding = file.file_name();
        let name = binding
            .to_str()
            .ok_or_else(|| anyhow!("Unable to get file name"))?;

        Ok(FileData {
            path: path.to_string(),
            name: name.to_string(),
        })
    }
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

fn create_dirs(target: &String) -> Result<()> {
    fs::create_dir_all(target.to_string() + "/error")?;
    fs::create_dir_all(target.to_string() + "/success")?;
    Ok(())
}
