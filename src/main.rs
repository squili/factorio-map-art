mod blueprint;
mod blueprint_decode;
mod color;

use std::io::Read;

use blueprint::Container;
use clap::{
    Parser,
    Subcommand,
};
use flate2::read::ZlibDecoder;
use image::Rgba;
use pbr::ProgressBar;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Generate(GenerateArgs),
    Parse { source: String },
}

#[derive(Parser)]
struct GenerateArgs {
    input: String,
    output: String,
    #[clap(short = 'r')]
    resize: Option<String>,
    #[clap(short = 'd')]
    dither: bool,
    #[clap(short = 'b')]
    blur: Option<f32>,
    #[clap(short = 'u')]
    upscale: Option<f32>,
    #[clap(short = 'c')]
    contrast: Option<f32>,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Generate(args) => generate(args),
        Action::Parse { source } => parse(source),
    }
}

fn rgba_to_tuple(rgba: &Rgba<u8>) -> (u8, u8, u8, u8) {
    // SAFETY: i mean, it really should just be fine
    unsafe {
        (
            *rgba.0.get_unchecked(0),
            *rgba.0.get_unchecked(1),
            *rgba.0.get_unchecked(2),
            *rgba.0.get_unchecked(3),
        )
    }
}

// generate a blueprint from an image
fn generate(args: GenerateArgs) {
    println!("Loading image");
    let source = image::open(args.input).unwrap();
    println!("Converting image to RGBA8");
    let mut rgba_image = source.into_rgba8();

    let base_width = rgba_image.width() as f32;
    let base_height = rgba_image.height() as f32;

    if let Some(upscale) = args.upscale {
        println!("Upscaling");
        rgba_image = image::imageops::resize(
            &rgba_image,
            (base_width * upscale) as u32,
            (base_height * upscale) as u32,
            image::imageops::FilterType::CatmullRom,
        );
    }

    if let Some(contrast) = args.contrast {
        println!("Contrasting");
        image::imageops::colorops::contrast_in_place(&mut rgba_image, contrast)
    }

    if args.dither {
        println!("Dithering");
        image::imageops::dither(&mut rgba_image, &color::GlobalColorMap);
    }

    if let Some(blur) = args.blur {
        println!("Blurring");
        rgba_image = image::imageops::blur(&rgba_image, blur);
    }

    if let Some(resize) = args.resize {
        println!("Resizing");
        let (width, height) = match resize.split_once('x') {
            Some((width, height)) => (
                width.parse().expect("Invalid resize width"),
                height.parse().expect("Invalid resize height"),
            ),
            None => {
                let ratio = 1f32 / resize.parse::<f32>().expect("Invalid resize ratio");
                ((base_width * ratio) as u32, (base_height * ratio) as u32)
            },
        };
        rgba_image = image::imageops::resize(&rgba_image, width, height, image::imageops::FilterType::CatmullRom);
    }

    let width = rgba_image.width() as usize;
    let height = rgba_image.height() as usize;
    let mut entities = vec![vec![""; width]; height];
    let mut tiles = vec![vec![""; width]; height];

    let mut progress_bar = ProgressBar::new(rgba_image.width() as u64 * rgba_image.height() as u64);
    progress_bar.message("Processing image ");

    let mut x = 0;
    for (row, column, rgba) in rgba_image.enumerate_pixels() {
        x += 1;
        if x == 999 {
            progress_bar.add(999);
            x = 0;
        }
        let pixel = rgba_to_tuple(rgba);
        if pixel.3 == 0 {
            continue; // ignore transparent pixels
        }
        let (kind, name) = color::nearest_color((pixel.0, pixel.1, pixel.2));
        match kind {
            color::ColorKind::Entity => entities[column as usize][row as usize] = name,
            color::ColorKind::Tile => tiles[column as usize][row as usize] = name,
        }
    }
    progress_bar.finish();
    println!();

    let container = Container::build(entities, tiles);
    let encoded = container.encode();

    std::fs::write(args.output, encoded).unwrap();
}

// extract data from a blueprint
fn parse(source: String) {
    let b64_step = base64::decode(&source.as_bytes()[1..]).unwrap();
    let mut zlib_step = String::new();
    ZlibDecoder::new(&*b64_step).read_to_string(&mut zlib_step).unwrap();
    println!("Raw: {}", zlib_step);
    let container: blueprint_decode::Container = serde_json::from_str(&zlib_step).unwrap();
    println!("Blueprint version {}", container.blueprint.version);
    for icon in container.blueprint.icons {
        println!("Icon ({:?}) {}", icon.signal.kind, icon.signal.name);
    }
    for entity in container.blueprint.entities {
        println!("Entity {} at {} {}", entity.name, entity.position.x, entity.position.y);
    }
    for tile in container.blueprint.tiles {
        println!("Tile {} at {} {}", tile.name, tile.position.x, tile.position.y);
    }
}
