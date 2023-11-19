use crate::config::Detection as DetectionConfig;
use anyhow::Result;
use reqwest::blocking::multipart;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Detection {
    pub description: String,
    pub predictions: Vec<Predicition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Predicition {
    pub bbox: Bbox,
    pub label: String,
    pub score: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bbox {
    pub x1: u16,
    pub x2: u16,
    pub y1: u16,
    pub y2: u16,
}

pub fn detect(im: &PathBuf, config: &DetectionConfig) -> Result<Detection> {
    let client = reqwest::blocking::Client::new();

    let url = &config.url;

    let files = multipart::Form::new()
        .file("image", im)?
        .part("model", multipart::Part::text(config.model.to_string()));

    let response: Detection = client
        .post(url)
        .multipart(files)
        .send()?
        .json::<Detection>()?;

    Ok(response)
}
