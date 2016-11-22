extern crate clap;
extern crate svd2rust;
extern crate svd_parser as svd;

use std::ascii::AsciiExt;
use std::fs::File;
use std::io::Read;

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
    match pattern {
        None => {
            for peripheral in &d.peripherals {
                println!("{} at 0x{:08x}",
                         peripheral.name,
                         peripheral.base_address);
            }
        }
        Some(pattern) => {
            let pattern = pattern.to_ascii_lowercase();
            for peripheral in &d.peripherals {
                if peripheral.name.to_ascii_lowercase().contains(&pattern) {
                    println!("{}", peripheral.name);
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
            .arg(Arg::with_name("peripheral")
                .help("Pattern used to select mtching peripherals")
                .value_name("PATTERN")
            .index(1)))
        .version(concat!(env!("CARGO_PKG_VERSION"),
                         include_str!(concat!(env!("OUT_DIR"),
                                              "/commit-info.txt"))))
        .get_matches();

    let xml = &mut String::new();
    File::open(matches.value_of("input").unwrap())
        .unwrap()
        .read_to_string(xml)
        .unwrap();

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
