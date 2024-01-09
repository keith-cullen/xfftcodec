// +--------------------------+
// |                          |
// |    Copyright (c) 2023    |
// |       Keith Cullen       |
// |                          |
// +--------------------------+

use clap::Parser;
use nix::sched::{CpuSet, sched_getaffinity, sched_setaffinity};
use nix::unistd::Pid;
use std::fs::File;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

/// Accept an input file name, an output file name and a CPU
#[derive(Parser)]
struct Cli {
    /// The input file name
    #[arg(short = 'i', long = "in")]
    in_file_name: String,
    /// The output file name
    #[arg(short = 'o', long = "out")]
    out_file_name: String,
    /// The CPU
    #[arg(short = 'c', long = "cpu")]
    cpu: usize,
    /// The number of new samples per window
    #[arg(short = 'n', long = "num")]
    num_new_samples: usize,
}

fn show_cpus(msg: &str) {
    print!("{}", msg);
    let cpu_set = sched_getaffinity(Pid::from_raw(0)).unwrap();
    for i in 0..CpuSet::count() {
        if cpu_set.is_set(i).unwrap() {
            print!(" {}", i);
        }
    }
    println!("");
}

fn pin(cpu: usize) {
    if !cfg!(target_os = "linux") {
        eprintln!("Error: Unsupported platform");
        exit(1);
    }
    show_cpus("CPUs enabled before");
    let mut cpu_set = CpuSet::new();
    if let Err(str) = cpu_set.set(cpu) {
        eprintln!("Error: Failed to set CPU {}: {}", cpu, str);
        exit(1);
    }
    if let Err(str) = sched_setaffinity(Pid::from_raw(0), &cpu_set) {
        eprintln!("Error: Failed to set CPU affinity: {}", str);
        exit(1);
    }
    show_cpus("CPUs enabled after");
}

fn filter(x: &mut [f64]) {
    const SHIFT: usize = 4;
    for i in 0 .. x.len() - SHIFT {
        x[i] = x[i + SHIFT];
    }
    for i in x.len() - SHIFT .. x.len() {
        x[i] = 0.0;
    }
}

fn main() {
    let opts = Cli::parse();
    pin(opts.cpu);
    let mut infile = match File::open(Path::new(&opts.in_file_name[..])) {
        Ok(val) => val,
        Err(str) => {
            eprintln!("Error: Unable to open '{}': {}", opts.in_file_name, str);
            exit(1);
        },
    };
    let (header, mut data) = match wav::read(&mut infile) {
        Ok(val) => val,
        Err(str) => {
            eprintln!("Error: Unable to read wave file '{}': {}", opts.in_file_name, str);
            exit(1);
        },
    };
    let ref mut samples = match data {
        wav::BitDepth::Sixteen(ref mut val) => val,
        _ => {
            eprintln!("Error: Expected wave file '{}' to hav 16-bit sample format", opts.in_file_name);
            exit(1);
        },
    };
    let nn = samples.len();
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    x.resize(nn, 0.0);
    y.resize(nn, 0.0);
    for i in 0..nn {
        x[i] = samples[i] as f64;
    }
    let ctx = codec::Ctx::new(opts.num_new_samples);
    let mut ch = codec::Ch::new(&ctx, nn, filter);
    let start = Instant::now();
    ch.process(&mut y[..], &x[..]);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    for i in 0..nn {
        samples[i] = y[i] as i16;
    }
    let mut outfile = match File::create(Path::new(&opts.out_file_name[..])) {
        Ok(val) => val,
        Err(str) => {
            eprintln!("Error: Unable to create '{}': {}", opts.out_file_name, str);
            exit(1);
        },
    };
    if let Err(str) = wav::write(header, &data, &mut outfile) {
        eprintln!("Error: Unable to write wave file '{}': {}", opts.out_file_name, str);
        exit(1);
    };
}

pub mod codec;
