use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use std::{thread, time};

mod string;
mod styles;

/// Rusty Clock
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The color to display in [red|green|etc.]
    #[arg(short, long, default_value_t = String::from("white"))]
    color: String,

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

    // manage colors
    let color: colored::Color = match args.color.parse() {
        Ok(c) => c,
        Err(_) => {
            return exit(&format!("{} is an invalid color", args.color), 1);
        }
    };

    let paths = styles::walk_style_dir();

    // Handle list flags
    if args.list_styles {
        if paths.is_empty() {
            return exit("No styles found!", 1);
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
            return exit(&format!("{} is not a valid style", args.style.clone()), 1);
        }
    }

    // Load assets
    let assets = styles::load_styles(&path_to_use);

    // Main event loop
    loop {
        // Save cursor position
        print!("\x1b7");

        let display_array = string::get_display_array();

        println!(
            "{}",
            string::build_string(display_array, &assets, 2, args.debug).color(color)
        );

        // Wait
        thread::sleep(time::Duration::from_millis(100));

        // Reset cursor
        print!("\x1b8");
    }
}

fn exit(msg: &str, code: i32) {
    println!("{}", msg);
    std::process::exit(code);
}
