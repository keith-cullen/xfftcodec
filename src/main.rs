// +--------------------------+
// |                          |
// |    Copyright (c) 2023    |
// |       Keith Cullen       |
// |                          |
// +--------------------------+

use std::fs::File;
use std::path::Path;

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
    const NUM_NEW_SAMPLES: usize = 2048;
    let mut infile = File::open(Path::new("in.wav")).unwrap();
    let (header, mut data) = wav::read(&mut infile).unwrap();
    if let wav::BitDepth::Sixteen(ref mut samples) = data {
        let nn = samples.len();
        let mut x: Vec<f64> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        x.resize(nn, 0.0);
        y.resize(nn, 0.0);
        for i in 0..nn {
            x[i] = samples[i] as f64;
        }
        let ctx = codec::Ctx::new(NUM_NEW_SAMPLES);
        let mut ch = codec::Ch::new(&ctx, nn, filter);
        ch.process(&mut y[..], &x[..]);
        for i in 0..nn {
            samples[i] = y[i] as i16;
        }
    }
    let mut outfile = File::create(Path::new("out.wav")).unwrap();
    wav::write(header, &data, &mut outfile).unwrap();
}

pub mod codec;
