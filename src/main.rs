use anyhow::Result;
use clap::crate_version;
use ova::cli;
use ova::config;
use ova::detection;
use ova::drawing;
use std::path::PathBuf;

fn main() -> Result<()> {
    let matches = cli::cli().version(crate_version!()).get_matches();
    let config = config::Config::load()?;
    if let Some(("detection", args)) = matches.subcommand() {
        if let Some(image_path) = args.get_one::<PathBuf>("image") {
            let mut im = image::open(image_path)?.into_rgba8();
            let detection: detection::Detection = detection::detect(image_path, &config.detection)?;

            if args.get_flag("visualize") {
                drawing::draw(&mut im, detection);
                let dest_path = config.output_dir.join(image_path.file_name().unwrap());
                im.save(&dest_path)?;
                println!("Image saved in `{}` directory", dest_path.to_string_lossy());
            } else {
                println!("{}", serde_json::to_string_pretty(&detection)?);
            }
        }
    }
    Ok(())
}
