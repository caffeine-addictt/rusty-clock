use chrono::{prelude::Local, Timelike};
use std::iter::repeat;

// Convert XX:XX:XX to Vec<u8> (24h)
pub fn get_display_array() -> Vec<u8> {
    let now = Local::now();
    vec![
        (now.hour() / 10) as u8,
        (now.hour() % 10) as u8,
        10,
        (now.minute() / 10) as u8,
        (now.minute() % 10) as u8,
        10,
        (now.second() / 10) as u8,
        (now.second() % 10) as u8,
    ]
}

/// Utility to build the strings
///
/// Split them by their newlines into their own lines.
/// Take thoses lines of same index
pub fn build_string(
    display_array: Vec<u8>,
    assets: &[String],
    spacing_x: u8,
    debug: bool,
) -> String {
    let mut assets_to_use: Vec<Vec<String>> = display_array
        .iter()
        .map(|i| assets[*i as usize].lines().map(|s| s.to_string()).collect())
        .collect();

    // pad the x & y-axis
    let target_y_height = assets_to_use
        .iter()
        .map(|a| a.len())
        .max()
        .unwrap_or_default();

    let target_x_height = assets_to_use
        .iter()
        .map(|a| a.iter().map(|l| l.len()).max().unwrap_or_default())
        .max()
        .unwrap_or_default();

    if debug {
        println!("Debug: target_y_height: {}", target_y_height);
        println!("Debug: target_x_height: {}", target_x_height);
    }

    for asset in assets_to_use.iter_mut() {
        let diff_y = target_y_height - asset.len();
        let pad_top = diff_y / 2;
        let pad_bottom = diff_y - pad_top;

        let empty_line = " ".repeat(target_x_height);

        // handle y
        let mut new_asset: Vec<String> = repeat(empty_line.clone()).take(pad_top).collect();

        // handle x
        new_asset.extend(asset.iter().map(|line| {
            let diff_x = target_x_height - line.len();

            format!("{line}{}", " ".repeat(diff_x))
        }));

        new_asset.extend(repeat(empty_line).take(pad_bottom));

        *asset = new_asset;
    }

    // Join each i of every asset together
    let mut built_str = String::new();
    let spacing = " ".repeat(spacing_x as usize);

    for line_index in 0..target_y_height {
        let new_line = assets_to_use
            .iter()
            .map(|asset| asset[line_index].clone())
            .collect::<Vec<_>>();

        built_str.push_str(new_line.join(spacing.as_str()).as_str());
        built_str.push('\n');

        if debug {
            println!("Debug: {}", new_line.join(spacing.as_str()));
        }
    }

    built_str
}
