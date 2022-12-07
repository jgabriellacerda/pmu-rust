pub struct IIRFilter<const N_NUM: usize, const N_DEN: usize> {
    numerator: [f64; N_NUM],
    denominator: [f64; N_DEN],
    size_num: usize,
    size_den: usize,
    num_buffer: [f64; N_NUM],
    den_buffer: [f64; N_DEN],
    k0_n: usize,
    k0_d: usize,
    previous_output: f64,
    out: f64,
}

impl<const N_NUM: usize, const N_DEN: usize> IIRFilter<N_NUM, N_DEN> {
    pub fn new(num: &[f64; N_NUM], den: &[f64; N_DEN]) -> IIRFilter<N_NUM, N_DEN> {
        let mut numerator = [0.0; N_NUM];
        let mut denominator = [0.0; N_DEN];
        for i in 0..N_NUM {
            numerator[i] = num[i];
        }
        for i in 0..N_DEN {
            denominator[i] = den[i];
        }

        IIRFilter {
            size_num: N_NUM,
            size_den: N_DEN - 1,
            numerator,
            denominator,
            num_buffer: [0.0; N_NUM],
            den_buffer: [0.0; N_DEN],
            k0_n: 0,
            k0_d: 0,
            previous_output: 0.0,
            out: 0.0,
        }
    }

    pub fn filter(&mut self, sample: f64) -> f64 {
        let mut out_num: f64;
        let mut out_den: f64;

        // Numerador
        self.num_buffer[self.k0_n] = sample;
        out_num = 0.0;
        for jj in 0..self.size_num {
            out_num += self.num_buffer[self.k0_n] * self.numerator[jj];
            self.k0_n = if self.k0_n == 0 {
                self.size_num - 1
            } else {
                self.k0_n - 1
            };
        }
        self.k0_n = if self.k0_n == self.size_num - 1 {
            0
        } else {
            self.k0_n + 1
        };

        // Denominador
        self.den_buffer[self.k0_d] = self.previous_output;
        out_den = 0.0;
        for jj in 0..self.size_den {
            out_den += self.den_buffer[self.k0_d] * self.denominator[jj + 1];
            self.k0_d = if self.k0_d == 0 {
                self.size_den - 1
            } else {
                self.k0_d - 1
            };
        }
        self.k0_d = if self.k0_d == self.size_den - 1 {
            0
        } else {
            self.k0_d + 1
        };

        self.out = (out_num - out_den) / self.denominator[0];
        self.previous_output = self.out;
        self.out
    }
}
