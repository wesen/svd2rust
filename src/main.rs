extern crate clap;
extern crate svd2rust;
extern crate svd_parser as svd;
extern crate regex;
extern crate term;
extern crate term_painter;

use std::ascii::AsciiExt;
use std::fs::File;
use std::io::Read;

use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use clap::{App, Arg, SubCommand};

fn generate_peripherals(d: &svd::Device, pattern: Option<&str>) {
    match pattern {
        None => {
            for peripheral in &d.peripherals {
                println!("const {}: usize = 0x{:08x};",
                         peripheral.name,
                         peripheral.base_address);
            }
        }
        Some(pattern) => {
            if let Some(peripheral) = d.peripherals
                .iter()
                .find(|x| x.name.to_ascii_lowercase() == pattern)
                .or(d.peripherals.iter().find(|x| x.name.to_ascii_lowercase().contains(&pattern))) {
                println!("{}",
                         svd2rust::gen_peripheral(peripheral, &d.defaults)
                             .iter()
                             .map(|i| i.to_string())
                             .collect::<Vec<_>>()
                             .join("\n\n"));
            }
        }
    }
}

fn list_peripherals(d: &svd::Device, pattern: Option<&str>) {
    let t = term::stdout();
    let colorize = t.is_some() && t.unwrap().supports_color() && svd2rust::tty::stdout_isatty();

    let outputSettings = svd2rust::list::OutputSettings {
        verbosity: 2,
    };
    let searchSettings = svd2rust::list::SearchSettings {
        search_descriptions: true
    };

    let searchHighlightStyle = if colorize { BrightGreen.bold().to_style() } else { Plain.to_style() };
    let nameStyle = if colorize { White.bold().to_style() } else { Plain.to_style() };

    match pattern {
        None => {
            let max_field_len = d.peripherals.iter().map(|p| p.name.len()).max().unwrap();
            for peripheral in &d.peripherals {
                println!("{name:<0$} (0x{address:08x}) - {description}",
                         max_field_len,
                         name = nameStyle.paint(&peripheral.name),
                         address = peripheral.base_address,
                         description = peripheral.description.as_ref().unwrap_or(&"".to_owned()));
            }
        }
        Some(pattern) => {
            let regex = svd2rust::create_regex(pattern);
            for peripheral in &d.peripherals {
                if svd2rust::match_peripheral(&regex, peripheral, &searchSettings) {
                    let s = svd2rust::list_peripheral(peripheral, &outputSettings);

                    let mut prev = 0;
                    for pos in regex.find_iter(&s) {
                        print!("{}", &s[prev..pos.0]);
                        print!("{}", searchHighlightStyle.paint(&s[pos.0..pos.1]));
                        prev = pos.1;
                    }
                    print!("{}", &s[prev..]);
                }
            }
        }
    }
}

fn main() {
    let matches = App::new("svd2rust")
        .about("Generate Rust register maps (`struct`s) from SVD files")
        .arg(Arg::with_name("input")
            .help("Input SVD file")
            .required(true)
            .short("i")
            .takes_value(true)
            .value_name("FILE"))
        .subcommand(SubCommand::with_name("generate")
            .about("generate the code for a peripheral")
            .arg(Arg::with_name("peripheral")
                .help("Pattern used to select a single peripheral")
                .value_name("PATTERN")
                .index(1)))
        .subcommand(SubCommand::with_name("list")
            .about("list the available peripherals")
            .arg(Arg::with_name("extended")
                .short("x")
                .help("Extended search (searches descriptions too"))
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of output verbosity (1 = with registers, 2 = with fields"))
            .arg(Arg::with_name("peripheral")
                .help("Pattern used to select mtching peripherals")
                .value_name("PATTERN")
                .index(1)
                .multiple(true)))
        .version(concat!(env!("CARGO_PKG_VERSION"),
                         include_str!(concat!(env!("OUT_DIR"),
                                              "/commit-info.txt"))))
        .get_matches();

    let mut xml = &mut String::new();
    let input_file = matches.value_of("input").unwrap();
    match File::open(input_file) {
        Ok(mut f) => {
            f.read_to_string(&mut xml).unwrap();
        },
        Err(e) => {
            println!("Could not open {}: {}", input_file, e);
            std::process::exit(1);
        }
    }

    let d = svd::parse(xml);

    match matches.subcommand_name() {
        Some("generate") => {
            if let Some(sub_matches) = matches.subcommand_matches("generate") {
                generate_peripherals(&d, sub_matches.value_of("peripheral"))
            }
        },
        Some("list") => {
            if let Some(sub_matches) = matches.subcommand_matches("list") {
                list_peripherals(&d, sub_matches.value_of("peripheral"))
            }
        },
        _ => println!("{}", matches.usage()),
    }
}
