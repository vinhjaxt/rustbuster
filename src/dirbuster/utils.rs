use std::{fs, fs::File, io::Write, path::Path, str};

use super::result_processor::SingleDirScanResult;

use regex::Regex;
use lazy_static::lazy_static;

pub fn build_urls(
    wordlist_path: &str,
    url: &str,
    extensions: Vec<String>,
    append_slash: bool,
    append_ext: bool,
) -> Vec<hyper::Uri> {
    debug!("building urls");
    lazy_static! {
        static ref RE_EXT : Regex = Regex::new(r"(?i)%ext%").unwrap();
    }
    let mut urls: Vec<hyper::Uri> = Vec::new();
    let wordlist =
        fs::read_to_string(wordlist_path).expect("Something went wrong reading the wordlist file");
    let urls_iter = wordlist
        .lines()
        .filter(|word| !word.starts_with('#') && !word.starts_with(' '))
        .map(|word| {
            if url.ends_with("/") {
                format!("{}{}", url, word)
            } else {
                format!("{}/{}", url, word)
            }
        });

    for url in urls_iter {
        if append_slash {
            if !url.ends_with("/") {
                match format!("{}/", url).parse::<hyper::Uri>() {
                    Ok(v) => {
                        urls.push(v);
                    }
                    Err(e) => {
                        trace!("URI: {}", e);
                    }
                }
            }
        }

        if append_ext || extensions.len() == 0 {
            match url.parse::<hyper::Uri>() {
                Ok(v) => {
                    urls.push(v);
                }
                Err(e) => {
                    trace!("URI: {}", e);
                }
            }
        }

        for extension in extensions.iter() {
            if append_slash {
                match format!("{}{}/", url, extension).parse::<hyper::Uri>() {
                    Ok(v) => {
                        urls.push(v);
                    }
                    Err(e) => {
                        trace!("URI: {}", e);
                    }
                }
            }

            if append_ext {
                match format!("{}{}", url, extension).parse::<hyper::Uri>() {
                    Ok(v) => {
                        urls.push(v);
                    }
                    Err(e) => {
                        trace!("URI: {}", e);
                    }
                }
            } else {
                match RE_EXT.replace_all(url.as_str(), extension.as_str()).parse::<hyper::Uri>() {
                    Ok(v) => {
                        urls.push(v);
                    }
                    Err(e) => {
                        trace!("URI: {}", e);
                    }
                }
            }
        }
    }

    urls
}

pub fn save_dir_results(path: &str, results: &Vec<SingleDirScanResult>) {
    let json_string = serde_json::to_string(&results).unwrap();

    let mut file = match File::create(Path::new(path)) {
        Ok(f) => f,
        Err(e) => {
            error!("Error while creating file: {}\n{}", path, e);
            return;
        }
    };

    match file.write_all(json_string.as_bytes()) {
        Ok(_) => debug!("Results saved to: {}", path),
        Err(e) => error!("Error while writing results to file: {}\n{}", path, e),
    };
}
