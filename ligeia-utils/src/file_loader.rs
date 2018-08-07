use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::Read;

use FileError;

pub struct FileLoader {
    _file: File
}

impl FileLoader {
    pub fn open(filename: &str) -> Result<Self, FileError> {
        match File::open(filename) {
            Ok(file) => Ok(Self {_file: file}),
            Err(_)   => Err(FileError::FileDataLoadError)
        }
    }

    pub fn as_json_obj<T>(self) -> Result<T, FileError>
        where for<'de> T: Deserialize<'de>
    {
        match serde_json::from_reader(self._file) {
            Ok(val) => Ok(val),
            Err(_)  => Err(FileError::FileMalformedError)
        }
    }
}