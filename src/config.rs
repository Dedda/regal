use std::path::Path;
lazy_static! {
    static ref CONFIG: Config = {
        let mut config_file = "/etc/regal/scanner.json".to_string();
        if let Some(c) = crate::ARGS.config.clone() {
            config_file = c;
        } else {
            if let Some(home) = dirs::home_dir() {
                let home_conf = home.join(".regal").join("scanner.json");
                if home_conf.is_file() {
                    config_file = home_conf.as_os_str().to_str().unwrap().to_string();
                }
            }
        }
        Config::from_file(config_file).unwrap()
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanDir {
    pub path: String,
    pub recursive: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub external_url: String,
    pub database_file: String,
    pub scan_dirs: Vec<ScanDir>,
}

impl Config {
    fn from_file(file: String) -> Option<Config> {
        if Path::new(&file).is_file() {
            if let Ok(data) = std::fs::read_to_string(&file) {
                return Some(serde_json::from_str::<Config>(&data).unwrap());
            }
            None
        } else {
            let external_url = std::env::var("EXTERNAL_URL").unwrap_or("localhost:8000".into());
            let database_file = "/var/regal/regal.sqlite3".to_string();
            let scan_dirs = vec![];
            Some(Config {
                external_url,
                database_file,
                scan_dirs,
            })
        }
    }
}

pub fn get() -> &'static Config {
    return &CONFIG;
}