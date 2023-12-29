// +--------------------------+
// |                          |
// |    Copyright (c) 2023    |
// |       Keith Cullen       |
// |                          |
// +--------------------------+

use super::*;
use std::f64::consts::PI;

fn compute_snr(s1: &[f64], s2: &[f64]) -> f64
{
    let mut sss: f64 = 0.0;
    let mut ses: f64 = 0.0;
    let mut err: f64;

    for i in 0..s1.len() {
        println!("s1: {}. s2: {}", s1[i], s2[i]);
        sss += s1[i] * s1[i];
        err  = s1[i] - s2[i];
        ses += err * err;
    }
    10.0 * (sss / ses).log10()
}

fn filter(_x: &mut [f64]) {}

#[test]
fn fist_last_block() {
    const SNR_THRESH: f64 = 300.0;
    const TOTAL_NUM_SAMPLES: usize = 128;
    const NUM_NEW_SAMPLES: usize = 128;
    const W_LEFT: f64 = 8.0 * PI / TOTAL_NUM_SAMPLES as f64;
    const W_RIGHT: f64 = 12.0 * PI / TOTAL_NUM_SAMPLES as f64;
    let mut x_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut x_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];

    for i in 0..TOTAL_NUM_SAMPLES {
        x_left[i]  = (W_LEFT * i as f64).cos();
        x_right[i] = (W_RIGHT * i as f64).cos();
    }
    let ctx = Ctx::new(NUM_NEW_SAMPLES);
    let mut left_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    let mut right_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    left_ch.process(&mut y_left[..], &x_left[..]);
    right_ch.process(&mut y_right[..], &x_right[..]);
    let snr = compute_snr(&x_left[..], &y_left[..]);
    if snr < SNR_THRESH {
        panic!("Left channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
    let snr = compute_snr(&x_right[..], &y_right[..]);
    if snr < SNR_THRESH {
        panic!("Right channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
}

#[test]
fn fist_mid_last_block() {
    const SNR_THRESH: f64 = 300.0;
    const TOTAL_NUM_SAMPLES: usize = 256;
    const NUM_NEW_SAMPLES: usize = 128;
    const W_LEFT: f64 = 8.0 * PI / TOTAL_NUM_SAMPLES as f64;
    const W_RIGHT: f64 = 12.0 * PI / TOTAL_NUM_SAMPLES as f64;
    let mut x_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut x_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];

    for i in 0..TOTAL_NUM_SAMPLES {
        x_left[i]  = (W_LEFT * i as f64).cos();
        x_right[i] = (W_RIGHT * i as f64).cos();
    }
    let ctx = Ctx::new(NUM_NEW_SAMPLES);
    let mut left_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    let mut right_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    left_ch.process(&mut y_left[..], &x_left[..]);
    right_ch.process(&mut y_right[..], &x_right[..]);
    let snr = compute_snr(&x_left[..], &y_left[..]);
    if snr < SNR_THRESH {
        panic!("Left channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
    let snr = compute_snr(&x_right[..], &y_right[..]);
    if snr < SNR_THRESH {
        panic!("Right channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
}

#[test]
fn first_second_last_last_block() {
    const SNR_THRESH: f64 = 300.0;
    const TOTAL_NUM_SAMPLES: usize = 128 + 32;
    const NUM_NEW_SAMPLES: usize = 128;
    const W_LEFT: f64 = 8.0 * PI / TOTAL_NUM_SAMPLES as f64;
    const W_RIGHT: f64 = 12.0 * PI / TOTAL_NUM_SAMPLES as f64;
    let mut x_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut x_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];

    for i in 0..TOTAL_NUM_SAMPLES {
        x_left[i]  = (W_LEFT * i as f64).cos();
        x_right[i] = (W_RIGHT * i as f64).cos();
    }
    let ctx = Ctx::new(NUM_NEW_SAMPLES);
    let mut left_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    let mut right_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    left_ch.process(&mut y_left[..], &x_left[..]);
    right_ch.process(&mut y_right[..], &x_right[..]);
    let snr = compute_snr(&x_left[..], &y_left[..]);
    if snr < SNR_THRESH {
        panic!("Left channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
    let snr = compute_snr(&x_right[..], &y_right[..]);
    if snr < SNR_THRESH {
        panic!("Right channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
}

#[test]
fn all_blocks() {
    const SNR_THRESH: f64 = 300.0;
    const TOTAL_NUM_SAMPLES: usize = 1024 + 32;
    const NUM_NEW_SAMPLES: usize = 128;
    const W_LEFT: f64 = 8.0 * PI / TOTAL_NUM_SAMPLES as f64;
    const W_RIGHT: f64 = 12.0 * PI / TOTAL_NUM_SAMPLES as f64;
    let mut x_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut x_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];

    for i in 0..TOTAL_NUM_SAMPLES {
        x_left[i]  = (W_LEFT * i as f64).cos();
        x_right[i] = (W_RIGHT * i as f64).cos();
    }
    let ctx = Ctx::new(NUM_NEW_SAMPLES);
    let mut left_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    let mut right_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    left_ch.process(&mut y_left[..], &x_left[..]);
    right_ch.process(&mut y_right[..], &x_right[..]);
    let snr = compute_snr(&x_left[..], &y_left[..]);
    if snr < SNR_THRESH {
        panic!("Left channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
    let snr = compute_snr(&x_right[..], &y_right[..]);
    if snr < SNR_THRESH {
        panic!("Right channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
}

#[test]
#[should_panic]
fn invalid() {
    const SNR_THRESH: f64 = 300.0;
    const TOTAL_NUM_SAMPLES: usize = 128;
    const NUM_NEW_SAMPLES: usize = 256;
    const W_LEFT: f64 = 8.0 * PI / TOTAL_NUM_SAMPLES as f64;
    const W_RIGHT: f64 = 12.0 * PI / TOTAL_NUM_SAMPLES as f64;
    let mut x_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_left: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut x_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];
    let mut y_right: [f64; TOTAL_NUM_SAMPLES] = [0.0; TOTAL_NUM_SAMPLES];

    for i in 0..TOTAL_NUM_SAMPLES {
        x_left[i]  = (W_LEFT * i as f64).cos();
        x_right[i] = (W_RIGHT * i as f64).cos();
    }
    let ctx = Ctx::new(NUM_NEW_SAMPLES);
    let mut left_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    let mut right_ch = Ch::new(&ctx, TOTAL_NUM_SAMPLES, filter);
    left_ch.process(&mut y_left[..], &x_left[..]);
    right_ch.process(&mut y_right[..], &x_right[..]);
    let snr = compute_snr(&x_left[..], &y_left[..]);
    if snr < SNR_THRESH {
        panic!("Left channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
    let snr = compute_snr(&x_right[..], &y_right[..]);
    if snr < SNR_THRESH {
        panic!("Right channel SNR: {} is below the threshold: {}", snr, SNR_THRESH);
    }
}
