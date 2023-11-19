use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use strum::Display;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub detection: Detection,
    pub output_dir: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Detection {
    pub url: String,
    pub model: DetectionModel,
}

#[derive(Deserialize, Serialize, Debug, Display)]
pub enum DetectionModel {
    #[strum(serialize = "yolov4")]
    YOLOV4,
}

impl Config {
    pub fn load() -> Result<Self> {
        let detection = Detection {
            url: env::var("OVA_DETECTION_URL")
                .unwrap_or("https://api.openvisionapi.com/api/v1/detection".into()),
            model: DetectionModel::YOLOV4,
        };

        let output_dir = PathBuf::from(env::var("OVA_OUTPUT_DIR").unwrap_or("output".to_string()));

        fs::create_dir_all(&output_dir)?;

        Ok(Self {
            detection,
            output_dir,
        })
    }
}
