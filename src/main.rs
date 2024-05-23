use std::{borrow::Cow, fs, hash::BuildHasherDefault, num::ParseIntError, path::PathBuf, str::FromStr};

use anyhow::{bail, Result};
use clap::Parser;
use image::{io::Reader as ImageReader, RgbaImage};
use rustc_hash::FxHasher;
use thiserror::Error;
use wgpu::naga::FastHashMap;

mod shader;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(group = clap::ArgGroup::new("input_group").multiple(false).required(true))]
struct Cli {
    /// The wgsl or glsl shader to be used
    shader: PathBuf,

    /// The output location of the image
    #[clap(default_value = "output.png")]
    output: PathBuf,

    /// Image for the shader to be applied on
    #[arg(short, long, group = "input_group")]
    input: Option<PathBuf>,

    /// Accepts dimensions in the format: [width]x[height] (ex: 512x256)
    #[arg(long, value_name = "DIMENSIONS", group = "input_group")]
    blank: Option<Dimensions>,
}

#[derive(Debug, Clone)]
struct Dimensions {
    width: u32,
    height: u32,
}

#[derive(Debug, Error)]
enum DimensionsError {
    #[error("{0}")]
    IntError(ParseIntError),
    #[error("Dimensions not in the correct format. (did you miss the x?)")]
    FormatError,
}

impl FromStr for Dimensions {
    type Err = DimensionsError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if let Some((w, h)) = s.split_once('x') {
            Ok(Dimensions {
                width: u32::from_str(w).map_err(DimensionsError::IntError)?,
                height: u32::from_str(h).map_err(DimensionsError::IntError)?,
            })
        } else {
            Err(DimensionsError::FormatError)
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let shader = if let Some(ext) = args.shader.extension() {
        match ext.to_str() {
            Some("wgsl") => {
                let s = fs::read_to_string(args.shader)?;
                wgpu::ShaderSource::Wgsl(Cow::Owned(s))
            }
            Some("frag") => {
                let s = fs::read_to_string(args.shader)?;
                wgpu::ShaderSource::Glsl {
                    shader: Cow::Owned(s),
                    stage: wgpu::naga::ShaderStage::Fragment,
                    defines: FastHashMap::with_hasher(BuildHasherDefault::<FxHasher>::default()),
                }
            }
            _ => bail!("The provided file in not a .wgsl or .frag file"),
        }
    } else {
        bail!("Invalid file type specified");
    };

    let image: RgbaImage = if let Some(dim) = args.blank {
        RgbaImage::new(dim.width, dim.height)
    } else if let Some(image) = args.input {
        ImageReader::open(image)?
            .with_guessed_format()?
            .decode()?
            .into_rgba8()
    } else {
        unreachable!()
    };

    let new_image = pollster::block_on(shader::run(
        image.width(),
        image.height(),
        image.into_raw(),
        shader,
    ))?;

    new_image.save(args.output)?;

    Ok(())
}
