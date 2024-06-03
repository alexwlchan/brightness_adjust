#![deny(warnings)]

use std::cmp::Ordering::Equal;
use std::str::FromStr;

use clap::Arg;
use palette::{FromColor, Hsv, Srgb};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct OutputValue {
    value: f32,
    rgb: Srgb<u8>,
    is_original: bool,
}

fn main() {
    let matches = clap::Command::new("brightness_adjust")
        .version(VERSION)
        .author("Alex Chan <alex@alexwlchan.net>")
        .about("Show some darker/lighter variants of a given colour")
        .arg(
            Arg::new("COLOUR")
                .help("colour to start with as a hex string, e.g. #d01c11")
                .required(true)
                .index(1),
        )
        .get_matches();

    // This .unwrap() is safe because "path" is a required param
    let base_colour = matches.get_one::<String>("COLOUR").expect("`COLOUR` is required");

    let rgb = match Srgb::from_str(base_colour) {
        Ok(c) => c.into_format::<u8>(),
        Err(e) => {
            eprintln!("Unable to parse COLOUR {:?} as hex: {}", base_colour, e);
            std::process::exit(1);
        }
    };

    let hsv: Hsv = Hsv::from_color(rgb.into_format::<f32>());

    let mut outputs = vec![OutputValue {
        value: hsv.value,
        rgb: rgb,
        is_original: true,
    }];

    for value in (0..=100).step_by(5) {
        let modified_hsv = Hsv::new(hsv.hue, hsv.saturation, value as f32 / 100.0);
        outputs.push(OutputValue {
            value: modified_hsv.value,
            rgb: Srgb::from_color(modified_hsv).into_format::<u8>(),
            is_original: false,
        });
    }

    // https://www.reddit.com/r/rust/comments/29kia3/comment/cilrzik/
    outputs.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(Equal));

    for op in outputs {
        let rgb = op.rgb;
        print!(
            "{:3}% = \x1B[38;2;{};{};{}m▇ #{:02x}{:02x}{:02x}\x1B[0m",
            (op.value * 100.0).round(),
            rgb.red,
            rgb.green,
            rgb.blue,
            rgb.red,
            rgb.green,
            rgb.blue
        );
        if op.is_original {
            println!(" (original colour)");
        } else {
            println!("");
        }
    }
}
