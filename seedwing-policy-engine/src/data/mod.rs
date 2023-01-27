use crate::runtime::RuntimeError;
use crate::value::RuntimeValue;
use serde_json::{Error, Value};
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub trait DataSource: Send + Sync + Debug {
    fn get(&self, path: String) -> Result<Option<RuntimeValue>, RuntimeError>;
}

#[derive(Debug)]
pub struct DirectoryDataSource {
    root: PathBuf,
}

impl DirectoryDataSource {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl DataSource for DirectoryDataSource {
    fn get(&self, path: String) -> Result<Option<RuntimeValue>, RuntimeError> {
        let target = self.root.join(path);

        if target.exists() {
            if target.is_dir() {
                Err(RuntimeError::FileUnreadable(target))
            } else if let Some(name) = target.file_name() {
                if name.to_string_lossy().ends_with(".json") {
                    // parse as JSON
                    if let Ok(file) = File::open(target.clone()) {
                        let json: Result<serde_json::Value, _> = serde_json::from_reader(file);
                        match json {
                            Ok(json) => Ok(Some(json.into())),
                            Err(e) => Err(RuntimeError::JsonError(target, e)),
                        }
                    } else {
                        Err(RuntimeError::FileUnreadable(target))
                    }
                } else if let Ok(mut file) = File::open(target.clone()) {
                    // just octets
                    let mut octets = Vec::new();
                    file.read_to_end(&mut octets);
                    Ok(Some(RuntimeValue::Octets(octets)))
                } else {
                    Err(RuntimeError::FileUnreadable(target))
                }
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
