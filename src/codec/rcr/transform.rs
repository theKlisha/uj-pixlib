lazy_static! {
    static ref DCT_COS_MUL: [f32; 64] = {
        let mut mul: [f32; 64] = [0_f32; 64];
        for k in 0..8 {
            for n in 0..8 {
                let kf = k as f32;
                let nf = n as f32;
                mul[k + 8*n] = (
                    (std::f32::consts::PI / 8.0)
                    * (0.0 + kf)
                    * (0.5 + nf)
                ).cos() * match k {
                    0 => 0.5 * (1.0 / 2_f32.sqrt()),
                    _ => 0.5,
                };
            }
        }

        mul
    };
}

pub fn dct(in_buf: &[f32; 64], out_buf: &mut [f32; 64]) {
    let mut mid_buf = [0.0; 64];

    for k in 0..8 {
        for y in 0..8 {
            mid_buf[y + 8*k] = (0..8)
                .map(|n| in_buf[y + 8*n] * DCT_COS_MUL[k + 8*n])
                .sum();
        }
    }

    for k in 0..8 {
        for x in 0..8 {
            out_buf[k + 8*x] = (0..8)
                .map(|n| mid_buf[n + 8*x] * DCT_COS_MUL[k + 8*n])
                .sum();
        }
    }
}

pub fn inv_dct(in_buf: &[f32; 64], out_buf: &mut [f32; 64]) {
    let mut mid_buf = [0.0; 64];

    for k in 0..8 {
        for y in 0..8 {
            mid_buf[y + 8*k] = (0..8)
                .map(|n| in_buf[y + 8*n] * DCT_COS_MUL[n + 8*k])
                .sum();
        }
    }

    for k in 0..8 {
        for x in 0..8 {
            out_buf[k + 8*x] = (0..8)
                .map(|n| mid_buf[n + 8*x] * DCT_COS_MUL[n + 8*k])
                .sum();
        }
    }
}

pub fn quant(in_buf: &[f32; 64], out_buf: &mut [u8; 64], table: &[i8; 64]) {
    for i in 0..64 {
        let x = (in_buf[i] / table[i] as f32) as i8;
        out_buf[i] = x.to_be_bytes()[0];
    }
}

pub fn inv_quant(in_buf: &[u8; 64], out_buf: &mut [f32; 64], table: &[i8; 64]) {
    for i in 0..64 {
        let x = i8::from_be_bytes([in_buf[i]]);
        out_buf[i] = x as f32 * table[i] as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverese_equality() {
        let spacial = [
            16.0, 11.0, 10.0, 16.0,  24.0,  40.0,  51.0,  61.0,
            12.0, 12.0, 14.0, 19.0,  26.0,  58.0,  60.0,  55.0,
            14.0, 13.0, 16.0, 24.0,  40.0,  57.0,  69.0,  56.0,
            14.0, 17.0, 22.0, 29.0,  51.0,  87.0,  80.0,  62.0,
            18.0, 22.0, 37.0, 56.0,  68.0, 109.0, 103.0,  77.0,
            24.0, 35.0, 55.0, 64.0,  81.0, 104.0, 113.0,  92.0,
            49.0, 64.0, 78.0, 87.0, 103.0, 121.0, 120.0, 101.0,
            72.0, 92.0, 95.0, 98.0, 112.0, 100.0, 103.0,  99.0,
        ];

        let mut frequency = [0.0; 64];
        dct(&spacial, &mut frequency);

        let mut new_spacial = [0.0; 64];
        inv_dct(&frequency, &mut new_spacial);

        for i in 0..64 {
            if i % 8 == 0 { println!(); }
            print!("\t{}", &frequency[i].round());
        }
        println!("\n");

        spacial
            .iter()
            .zip(new_spacial.iter())
            .for_each(|(a, b)| assert_eq!(a.round(), b.round()));
    }
}
