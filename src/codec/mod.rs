// +--------------------------+
// |                          |
// |    Copyright (c) 2023    |
// |       Keith Cullen       |
// |                          |
// +--------------------------+

extern crate xfft;

use xfft::*;
use std::f64::consts::PI;

pub struct Ctx {
    num_new_samples: usize,
    win: Vec<f64>,
    rfft_ctx: rfft::Ctx,
}

impl Ctx {
    pub fn new(num_new_samples: usize) -> Ctx {
        let mut ctx = Ctx {
            num_new_samples: num_new_samples,
            win: Vec::new(),
            rfft_ctx: rfft::Ctx::new(2 * num_new_samples),
        };
        ctx.gen_han_win();
        ctx
    }

    fn gen_han_win(&mut self) {
        let size = self.num_new_samples * 2;
        if size == 0 || size & 0x1 != 0x0 {
            panic!("Hann window size must be an interger size of 2")
        }
        self.win.resize(self.num_new_samples, 0.0);
        let w = (2.0 * PI) / size as f64;
        for i in 0..self.num_new_samples {
            self.win[i] = 0.5 - 0.5 * (w * (i as f64 + 0.5)).cos();
        }
    }
}

pub struct Ch<'a, F> where F: Fn(&mut [f64]) {
    ctx: &'a Ctx,
    total_num_samples: usize,
    x_index: usize,
    y_index: usize,
    prev_in: Vec<f64>,
    fftx: Vec<f64>,
    prev_fftx: Vec<f64>,
    filter: F,
}

impl<F> Ch<'_, F> where F: Fn(&mut [f64]) {
    pub fn new(ctx: &Ctx, total_num_samples: usize, filter: F) -> Ch<'_, F>
        where F: Fn(&mut [f64]) {
        if total_num_samples == 0 {
            panic!("The total number of samples must be > 0");
        }
        let mut ch = Ch {
            ctx: ctx,
            total_num_samples: total_num_samples,
            x_index: 0,
            y_index: 0,
            prev_in: Vec::new(),
            fftx: Vec::new(),
            prev_fftx: Vec::new(),
            filter: filter,
        };
        ch.prev_in.resize(ctx.num_new_samples, 0.0);
        ch.fftx.resize(2 * ctx.num_new_samples, 0.0);
        ch.prev_fftx.resize(ctx.num_new_samples, 0.0);
        ch
    }

    // Carry out the stages of the filtering process for one
    // block with zero padding for the first and last blocks
    pub fn process(&mut self, y: &mut [f64], x: &[f64]) {
        if x.len() != self.total_num_samples || y.len() != self.total_num_samples {
            panic!("Input and output slices must have exact length");
        }
        let n = self.ctx.num_new_samples;
        let mut tempx = Vec::new();
        let mut tempy = Vec::new();

        tempx.resize(self.ctx.num_new_samples, 0.0);
        tempy.resize(self.ctx.num_new_samples, 0.0);

        while self.y_index < self.total_num_samples {
            if self.x_index == 0 {
                // Process first block
                // Read an input block but don't write an output block
                self.block(&mut tempy[..], &x[..n]);
                self.x_index += n;
            } else if self.x_index + n <= self.total_num_samples {
                // Process middle blocks
                // Read an input block and write an output block
                self.block(&mut y[self.y_index .. self.y_index + n],
                           &x[self.x_index .. self.x_index  + n]);
                self.x_index += n;
                self.y_index += n;
            } else if self.x_index < self.total_num_samples {
                // Process 2nd last block
                // Read a partial input block and write a full output block
                for i in 0 .. self.total_num_samples - self.x_index {
                    tempx[i] = x[i + self.x_index];
                }
                for i in self.total_num_samples - self.x_index .. self.ctx.num_new_samples {
                    tempx[i] = 0.0;
                }
                self.block(&mut y[self.y_index..], &tempx[..]);
                self.x_index += self.total_num_samples - self.x_index;
                self.y_index += n;
            } else if self.y_index < self.total_num_samples {
                // Process last block
                // Don't read an input block and write a partial output block
                for i in 0..n {
                    tempx[i] = 0.0;
                }
                self.block(&mut tempy[..], &tempx[..]);
                for i in self.y_index .. self.total_num_samples {
                    y[i] = tempy[i - self.y_index];
                }
                self.y_index = self.total_num_samples;
            }
        }
    }

    // Carry out the stages of the filtering process for one block
    fn block(&mut self, y: &mut [f64], x: &[f64]) {
        self.input(x);
        self.window();
        self.transform();
        self.output(y);
    }


    // Shift the input buffer left by num_new_samples
    // and read in the same number of new samples to
    // the right half of the input buffer
    fn input(&mut self, x: &[f64]) {
        let n = self.ctx.num_new_samples;
        for i in 0..n {
            self.fftx[i] = self.prev_in[i];
            self.fftx[i + n] = x[i];
            self.prev_in[i] = x[i];
        }
    }

    // Apply the window function
    fn window(&mut self) {
        let n = self.ctx.num_new_samples;
        for i in 0..n {
            self.fftx[i] *= self.ctx.win[i];
        }
        for i in n .. 2 * n {
            self.fftx[i] *= self.ctx.win[2 * n - 1 - i];
        }
    }

    // Compute FFT, scale FFT output,
    // Compute IFFT, scale IFFT output
    fn transform(&mut self) {
        let n = self.ctx.num_new_samples;
        let nn = 2 * n;
        let sf: f64 = 1.0 / (n as f64).sqrt();

        // Perform FFT
        self.ctx.rfft_ctx.fwd(&mut self.fftx[..]);
        for i in 0 .. nn {
            self.fftx[i] *= sf;
        }

        (self.filter)(&mut self.fftx[..]);

        // Perform inverse FFT
        self.ctx.rfft_ctx.bwd(&mut self.fftx[..]);
        for i in 0 .. nn {
            self.fftx[i] *= sf;
        }
    }

    // Overlap and add the right half of the previous time
    // window with the left half of the current time window
    // Both time windows are obtained from the output
    // of the IFFT
    fn output(&mut self, y: &mut [f64]) {
        let n = self.ctx.num_new_samples;
        for i in 0..n {
            y[i] = self.prev_fftx[i] + self.fftx[i];
            self.prev_fftx[i] = self.fftx[i + n];
        }
    }
}

#[cfg(test)]
mod tests;
