use std::{
    env::current_dir,
    fs::File,
    io::{BufRead, BufReader},
};

const PI: f64 = 3.14159265358979323846;

#[derive(Debug)]
pub struct SavitzkyGolay<const N_SG: usize> {
    numerator: [f64; N_SG],
    buffer: [f64; N_SG],
    gain: f64,
    out: f64,
    current_idx: usize,
    size: usize,
}

impl<const N_SG: usize> SavitzkyGolay<N_SG> {
    pub fn new(fs: usize) -> SavitzkyGolay<N_SG> {
        let mut sg_coef: [f64; N_SG] = [0.0; N_SG];
        SavitzkyGolay::load_coefficients(&mut sg_coef).expect("Error reading filter coeficcients.");

        let sg_gain: f64 = fs as f64 / (2.0 * PI);
        SavitzkyGolay {
            numerator: sg_coef,
            buffer: [0.0; N_SG],
            gain: sg_gain,
            out: 0.0,
            current_idx: 0,
            size: N_SG,
        }
    }

    fn load_coefficients(array: &mut [f64; N_SG]) -> std::io::Result<()> {
        let root = current_dir()?;
        let relative_path = format!("src/filters/savitzky_golay_{N_SG}.txt");
        let file_path = root.join(relative_path);
        let f = File::open(file_path)?;
        let mut reader = BufReader::new(f);

        for line in 0..N_SG {
            let mut coef = String::new();
            reader.read_line(&mut coef)?;
            let coef: f64 = coef.trim().parse::<f64>().unwrap();
            array[line] = coef;
        }
        Ok(())
    }

    pub fn filter(&mut self, sample: f64) -> f64 {
        self.buffer[self.current_idx] = sample;

        self.out = 0.0;

        let mut conv_idx: isize = self.current_idx as isize;
        for k in 0..self.size {
            self.out += self.buffer[conv_idx as usize] * self.numerator[k];

            conv_idx -= 1;
            if conv_idx < 0 {
                conv_idx += self.size as isize;
            }
        }

        self.out *= self.gain;

        self.current_idx += 1;
        if self.current_idx >= self.size {
            self.current_idx -= self.size;
        }
        self.out
    }

    pub fn update_bufer(&mut self, sample: f64) {
        self.buffer[self.current_idx] = sample;

        self.out = 0.0;

        self.current_idx += 1;
        if self.current_idx >= self.size {
            self.current_idx -= self.size;
        }
    }
}
