mod moving_average;
mod savitzky_golay;

use std::env::current_dir;
// use std::{env, io};
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

use moving_average::MovingAverage;
use savitzky_golay::SavitzkyGolay;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");

    let mut sg: SavitzkyGolay<17> = SavitzkyGolay::new(960);
    println!("{:?}", sg);

    // let mut mov_avg: MovingAverage<16> = MovingAverage::new();

    let samples: [f64; 20] = [10.0; 20];

    for sample in samples {
        // println!("{}, {}", sample, mov_avg.filter(sample));
        println!("{:?}", sg.filter(sample));
    }
}
