extern crate clap;
use clap::{App, Arg, ArgGroup};
use std::process::Command;

fn main() {
    let args = App::new("cubic-backlight")
        .version("0.1.0")
        .author("Árni Dagur <arni@dagur.eu>")
        .arg(
            Arg::with_name("N")
                .short("N")
                .value_name("NUM")
                .help("Number of steps")
                .default_value("10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inc")
                .short("i")
                .long("inc")
                .help("Increase brightness"),
        )
        .arg(
            Arg::with_name("dec")
                .short("d")
                .long("dec")
                .help("Decrease brightness"),
        )
        .group(
            ArgGroup::with_name("operator")
                .arg("inc")
                .arg("dec")
                .required(true),
        )
        .arg(
            Arg::with_name("min")
                .short("m")
                .long("min")
                .default_value("0")
                .help("Minimum brightness percentage"),
        )
        .arg(
            Arg::with_name("max")
                .short("M")
                .long("max")
                .default_value("100")
                .help("Minimum brightness percentage"),
        )
        .get_matches();

    // Get values of arguments
    let n = args.value_of("N").unwrap().parse::<f32>().unwrap();
    let max = args.value_of("max").unwrap().parse::<f32>().unwrap();
    let min = args.value_of("min").unwrap().parse::<f32>().unwrap();

    // Get current brightness, assign it to b
    let output = Command::new("xbacklight")
        .output()
        .expect("failed to execute process")
        .stdout;
    let output = String::from_utf8(output).unwrap();
    let output = output.trim();
    let b = output.parse::<f32>().unwrap();

    // Calculate new brightness
    let cbrt_b = b.powf(1.0 / 3.0);
    let step = (max.powf(1.0 / 3.0) - min.powf(1.0 / 3.0)) / n;

    let mut nb;
    if args.is_present("dec") {
        nb = cbrt_b - step;
    } else {
        nb = cbrt_b + step;
    }
    nb = nb.powf(3.0);

    // Make sure min <= nb <= max
    if max < nb {
        nb = max;
    } else if nb < min {
        nb = min;
    }

    // Set new brightness
    Command::new("xbacklight")
        .arg("-set")
        .arg(&nb.to_string())
        .spawn()
        .expect("failed to execute process");
}
