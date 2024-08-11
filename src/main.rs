use clap::Parser;
use std::path::PathBuf;
use std::{thread, time};
use std::iter::repeat;
use std::borrow::Cow;

mod string;
mod styles;

/// Rusty Clock
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The color to display in
    #[arg(short, long, default_value_t = 218)]
    color: u8,

    // The spacing between each digits
    #[arg(long, default_value_t = 2)]
    spacing: u8,

    /// The style of clock - Either a preset name or a path to a file
    #[arg(short, long, default_value_t = String::from("curvy"))]
    style: String,

    /// Whether to run it in debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// List styles
    #[arg(short, long, default_value_t = false)]
    list_styles: bool,
}

impl Args {
    fn get_ascii(&self) -> u8 {
        self.color
    }
}

/// Entrypoint
/// Parsing arguments
fn main() {
    let mut args = Args::parse();
    args.style = args.style.to_lowercase();

    // stdOut if debug
    if args.debug {
        println!("Parsed color: {}", args.color);
        println!("Parsed spacing: {}", args.spacing);
        println!("Parsed style: {}", args.style);
        println!("Parsed list_styles: {}", args.list_styles);
    }

    let paths = styles::walk_style_dir();

    // Handle list flags
    if args.list_styles {
        if paths.is_empty() {
            println!("No styles found!");
            return;
        }

        println!("Use a style with: rusty-clock -s <style>");
        println!("Available styles:");
        for style in paths.iter() {
            println!("  {:?}", style.file_stem().unwrap().to_str().unwrap());
        }
        return;
    }

    // Lookup styles
    let mut path_to_use = PathBuf::new();
    let mut default_style = false;

    for style in paths.iter() {
        if style.file_stem().unwrap().to_str().unwrap() == args.style {
            path_to_use = style.clone();
            default_style = true;
            break;
        }
    }

    if !default_style {
        path_to_use = PathBuf::from(&*args.style);

        if !path_to_use.is_file() {
            panic!("{}", format!("{} is not a valid style", args.style.clone()));
        }
    }

    // Load assets
    let assets = styles::load_styles(&path_to_use);

    // Main event loop
    loop {
        // Save cursor position
        print!("\x1b7");

        let display_array = string::get_display_array();

        // println!("{}", build_string(style_index, args.spacing));

        // Wait
        thread::sleep(time::Duration::from_millis(100));

        // Reset cursor
        print!("\x1b8");
    }
}

//TODO: Utility to add color
