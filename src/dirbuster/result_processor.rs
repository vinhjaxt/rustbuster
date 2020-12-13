use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleDirScanResult {
    pub url: String,
    pub method: String,
    pub status: u16,
    pub size: Option<String>,
    pub error: Option<String>,
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultProcessorConfig {
    pub include: Vec<u16>,
    pub ignore: Vec<u16>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanResult {
    pub results: Vec<SingleDirScanResult>,
    config: ResultProcessorConfig,
}

impl ScanResult {
    pub fn new(config: ResultProcessorConfig) -> Self {
        ScanResult {
            results: Vec::<SingleDirScanResult>::new(),
            config,
        }
    }

    pub fn maybe_add_result(&mut self, res: SingleDirScanResult) -> bool {
        trace!("{:?}", res);
        let mut ignore = false;
        let mut include = false;
        for code in self.config.ignore.iter() {
            if res.status == *code {
                ignore = true;
                break;
            }
        }

        for code in self.config.include.iter() {
            if res.status == *code {
                include = true;
                break;
            }
        }

        if !ignore && (self.config.include.is_empty() || include) {
            self.results.push(res);
            return true;
        }

        false
    }
}
