use std::f64::consts::SQRT_2;

const PI: f64 = 3.14159265359;
const W0: f64 = 2.0 * PI * 60.0;

#[derive(Debug)]
pub struct DFT<const NPPC: usize, const N: usize> {
    pub re: f64,
    pub im: f64,
    sin_buffer: [f64; N],
    cos_buffer: [f64; N],
    sin_tab: [f64; NPPC],
    cos_tab: [f64; NPPC],
    w: [f64; N],
    ii: usize,
    k0: usize,
    size: usize,
    nppc: usize,
    odd: bool,
    gain: f64,
}

impl<const NPPC: usize, const N: usize> DFT<NPPC, N> {
    pub fn new(fs: usize) -> DFT<NPPC, N> {
        let odd = N % 2 == 1;

        let mut dft = DFT {
            re: 0.0,
            im: 0.0,
            sin_buffer: [0.0; N],
            cos_buffer: [0.0; N],
            sin_tab: [0.0; NPPC],
            cos_tab: [0.0; NPPC],
            w: [0.0; N],
            ii: 0,
            k0: 0,
            size: N,
            nppc: NPPC,
            odd,
            gain: 0.0,
        };
        DFT::init(fs, &mut dft);
        dft
    }

    fn init(fs: usize, dft: &mut DFT<NPPC, N>) {
        let T: f64 = 1.0 / fs as f64;
        let nppc: usize = (fs as f64 / 60.0) as usize;

        for i in 0..nppc {
            let rad = i as f64 * W0 * T;
            dft.cos_tab[i] = rad.cos();
            dft.sin_tab[i] = rad.sin();
        }

        let mut hamming: [f64; N] = [0.0; N];
        // report frequency
        let fr = 8.19;
        let mut sum_w = 0.0;

        for n in 0..N {
            let rad = (2.0 * PI / (N as f64)) * (n as f64);
            hamming[n] = 0.54 - 0.46 * rad.cos();
            if n == N / 2 {
                dft.w[n] = 1.0;
            } else {
                let rad =
                    2.0 * PI * (2.0 * fr / (fs as f64)) * ((n as isize - N as isize / 2) as f64);
                dft.w[n] = (rad.sin() / rad) * hamming[n];
            }
            sum_w += dft.w[n];
        }

        let gain = SQRT_2 / sum_w;
        dft.gain = gain;
    }

    pub fn calculate(&mut self, sample: f64) {
        let mut n: usize = self.k0;

        self.cos_buffer[n] = sample * self.cos_tab[self.ii];
        self.sin_buffer[n] = sample * self.sin_tab[self.ii];
        self.ii = (self.ii + 1) % self.nppc;
        self.re = 0.0;
        self.im = 0.0;

        for k in 0..self.size {
            println!("n: {} | k: {}", n, k);
            self.re += self.cos_buffer[n] * self.w[k];
            self.im += self.sin_buffer[n] * self.w[k];
            // println!("idx: {} | value: {}", n, self.cos_buffer[n] * self.w[k]);
            n = if n == 0 { self.size - 1 } else { n - 1 };
        }

        self.k0 = (self.k0 + 1) % self.size;

        self.re = self.re * self.gain;
        self.im = -self.im * self.gain;
    }

    pub fn calculate_symmetric(&mut self, sample: f64) {
        let n: usize = self.k0;
        let mut n1: usize = n;
        let mut n2: usize = (n + 1) % self.size;

        self.cos_buffer[n] = sample * self.cos_tab[self.ii];
        self.sin_buffer[n] = sample * self.sin_tab[self.ii];
        self.ii = (self.ii + 1) % self.nppc;
        self.re = 0.0;
        self.im = 0.0;

        for k in 0..(self.size / 2) {
            // println!("n1: {} | n2: {} | k: {}", n1, n2, k);
            self.re += (self.cos_buffer[n1] + self.cos_buffer[n2]) * self.w[k];
            self.im += (self.sin_buffer[n1] + self.sin_buffer[n2]) * self.w[k];
            n1 = if n1 == 0 { self.size - 1 } else { n1 - 1 };
            n2 = (n2 + 1) % self.size;
        }

        if self.odd {
            let k: usize = self.size / 2;
            self.re += (self.cos_buffer[n1]) * self.w[k];
            self.im += (self.sin_buffer[n1]) * self.w[k];

            println!("odd n1: {} | n2: {} | k: {}", n1, n2, k);
        }

        println!("{}", self.size / 2);

        self.k0 = (self.k0 + 1) % self.size;

        self.re = self.re * self.gain;
        self.im = -self.im * self.gain;
    }
}
