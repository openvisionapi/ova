use std::path::PathBuf;

use anyhow::Result;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::Command;
use pretty_assertions::assert_eq;

#[test]
fn output_json() -> Result<()> {
    let mut server = mockito::Server::new();
    let url = server.url();

    dbg!(&url);

    let _ = server
        .mock("POST", "/api/v1/detection")
        .with_status(200)
        .with_body(
            r#"
{
  "description": "Detected objects",
  "predictions": [
    {
      "bbox": {
        "x1": 441,
        "x2": 983,
        "y1": 199,
        "y2": 1270
      },
      "label": "cat",
      "score": "0.92"
    }
  ]
}
"#,
        )
        .create();

    let cmd = Command::cargo_bin("ova")
        .unwrap()
        .env("OVA_DETECTION_URL", format!("{}/api/v1/detection", url))
        .arg("detection")
        .arg("-i")
        .arg("assets/cat.jpeg")
        .output()
        .unwrap();

    cmd.clone().assert().success();

    let output = String::from_utf8(cmd.stdout)?;
    let expected = r#"{
  "description": "Detected objects",
  "predictions": [
    {
      "bbox": {
        "x1": 441,
        "x2": 983,
        "y1": 199,
        "y2": 1270
      },
      "label": "cat",
      "score": "0.92"
    }
  ]
}
"#;
    assert_eq!(output, expected);

    Ok(())
}

#[test]
fn output_image() -> Result<()> {
    let mut server = mockito::Server::new();
    let url = server.url();

    let _ = server
        .mock("POST", "/api/v1/detection")
        .with_status(200)
        .with_body(
            r#"
{
  "description": "Detected objects",
  "predictions": [
    {
      "bbox": {
        "x1": 441,
        "x2": 983,
        "y1": 199,
        "y2": 1270
      },
      "label": "cat",
      "score": "0.92"
    }
  ]
}
"#,
        )
        .create();

    let cmd = Command::cargo_bin("ova")
        .unwrap()
        .env("OVA_DETECTION_URL", format!("{}/api/v1/detection", url))
        .arg("detection")
        .arg("-v")
        .arg("-i")
        .arg("assets/cat.jpeg")
        .output()
        .unwrap();

    cmd.clone().assert().success();

    assert_eq!(
        String::from_utf8(cmd.stdout)?,
        "Image saved in `output/cat.jpeg` directory\n"
    );
    assert!(PathBuf::from("output/cat.jpeg").exists());
    Ok(())
}
