use clap::{arg, ArgAction, Command};
use std::path::PathBuf;

pub fn cli() -> Command {
    Command::new("ova")
        .about("OVA Client CLI")
        .subcommand_required(true)
        .subcommand(
            Command::new("detection")
                .about("Object Detection")
                .arg(
                    arg!(--image <image>)
                        .short('i')
                        .required(true)
                        .help("path to image file")
                        .value_parser(clap::value_parser!(PathBuf)),
                )
                .arg(
                    arg!(--visualize)
                        .short('v')
                        .help("Draw boxes on the detected objects")
                        .action(ArgAction::SetTrue)
                        .required(false),
                ),
        )
}
