mod dft;
mod irr_filter;
mod moving_average;
mod savitzky_golay;

use std::f64::consts::PI;

use moving_average::MovingAverage;
use savitzky_golay::SavitzkyGolay;

use crate::dft::DFT;
use crate::irr_filter::IIRFilter;

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");

    let mut sg: SavitzkyGolay<17> = SavitzkyGolay::new(960);
    // println!("{:?}", sg);

    let iir_num = [
        0.000024419565298361,
        0.000097678261193446,
        0.000146517391790169,
        0.000097678261193446,
        0.000024419565298361,
    ];
    let iir_den = [
        1.000000000000000,
        -3.615352702016196,
        4.918315719139102,
        -2.982871345869219,
        0.680299041791087,
    ];
    let mut iir: IIRFilter<5, 5> = IIRFilter::new(&iir_num, &iir_den);

    let mut dft1: DFT<16, { (16.0 * 10.25) as usize }> = DFT::new(960);
    let mut dft2: DFT<16, { (16.0 * 10.25) as usize }> = DFT::new(960);

    let mut mov_avg: MovingAverage<16> = MovingAverage::new();

    // let samples: [f64; 200] = [10.0; 200];

    println!("{:#?}", dft1);
    println!("{:#?}", dft2);
    for sample in 0..200 {
        // println!("{}, {}", sample, mov_avg.filter(sample));
        // println!("{:?}", iir.filter(sample));
        dft1.calculate(10000.0 * (2.0 * PI * 60.0 * sample as f64 / (960.0)).cos());
        print!("mag: {} | ", (dft1.im * dft1.im + dft1.re * dft1.re).sqrt());
        // println!("============================================ normal");
        dft2.calculate_symmetric(10000.0 * (2.0 * PI * 60.0 * sample as f64 / (960.0)).cos());
        println!("mag: {}", (dft2.im * dft2.im + dft2.re * dft2.re).sqrt());
        // println!("============================================ simetrico");
    }
}
