mod yaml;

use std::{fs::File, io::BufReader, path::Path};

type ResultDataParsing = Result<crate::data::RawData, String>;

enum SupportedFiletypes {
    Yaml
}

pub fn parse_file_as_raw_data(path: &Path) -> ResultDataParsing {
    let extension: &std::ffi::OsStr = path.extension()
        .ok_or("Could not determine file's extension.".to_string())?;

    let extension_as_utf8 = extension.to_str()
        .ok_or("Extension is not proper UTF-8.".to_string())?;

    let filetype = match extension_as_utf8 {
        "yaml" | "yml" => { SupportedFiletypes::Yaml }
        _ => { Err("File's extension does not match any supported filetype.".to_string())? }
    };

    let file = File::open(path)
        .or(Err("Error while reading the file.".to_string()))?;

    let mut file_buffer = BufReader::new(file);

    match filetype {
        SupportedFiletypes::Yaml => yaml::parse_as_raw_data(&mut file_buffer)
    }
}
