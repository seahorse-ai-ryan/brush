#![recursion_limit = "256"]

pub mod ui;

use brush_process::{data_source::DataSource, process_loop::ProcessArgs};
use clap::{Error, Parser, builder::ArgPredicate, error::ErrorKind};

#[derive(Parser)]
#[command(
    author,
    version,
    arg_required_else_help = false,
    about = "Brush - universal splats"
)]
pub struct Cli {
    /// Source to load from (path or URL).
    #[arg(value_name = "PATH_OR_URL")]
    pub source: Option<DataSource>,

    #[arg(
        long,
        default_value = "true",
        default_value_if("source", ArgPredicate::IsPresent, "false"),
        help = "Spawn a viewer to visualize the training"
    )]
    pub with_viewer: bool,

    /// Reset all window sizes and positions to their default values
    #[arg(long, help = "Reset all window sizes and positions to their default values")]
    pub reset_windows: bool,

    #[clap(flatten)]
    pub process: ProcessArgs,
}

impl Cli {
    pub fn validate(self) -> Result<Self, Error> {
        if !self.with_viewer && self.source.is_none() {
            return Err(Error::raw(
                ErrorKind::MissingRequiredArgument,
                "When --with-viewer is false, --source must be provided",
            ));
        }
        Ok(self)
    }
}
