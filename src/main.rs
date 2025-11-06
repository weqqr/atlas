use std::error::Error;

use tracing::{debug, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

use crate::{
    raster::{Image, Rgba},
    render::render_node,
};

pub mod asset;
pub mod raster;
pub mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?
        .add_directive("atlas=debug".parse()?);

    tracing_subscriber::fmt().with_env_filter(filter).init();

    info!("rendering image");

    let mut image = render_node();

    let path = "test.png";

    image.save_png(path);

    debug!(message = "image saved!", ?path);

    Ok(())
}
