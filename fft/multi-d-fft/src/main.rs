// #![no_std]
use microfft::{real::rfft_512, Complex32};
use micromath::F32Ext;

// #[repr(C)]
// pub struct Presence {
//     is_present: bool,
//     bin_number: [u8; 3],
// }

const RADAR_SAMPLES: usize = 500;
const BINS: usize = 1;
const FLOAT_SAMPLES: usize = 512;
const FFT_SIZE: usize = FLOAT_SAMPLES / 2;

fn main() {
    let sample: [u16; RADAR_SAMPLES] = [
        2200, 3116, 3770, 3992, 3769, 3242, 2649, 2234, 2149, 2400, 2844, 3247, 3379, 3104, 2440,
        1555, 711, 165, 81, 461, 1157, 1918, 2492, 2714, 2571, 2200, 1828, 1685, 1907, 2481, 3242,
        3938, 4318, 4234, 3688, 2844, 1959, 1295, 1020, 1152, 1555, 1999, 2250, 2165, 1750, 1157,
        630, 407, 629, 1283, 2200, 3116, 3770, 3992, 3769, 3242, 2649, 2234, 2149, 2400, 2844,
        3247, 3379, 3104, 2440, 1555, 711, 165, 81, 461, 1157, 1918, 2492, 2714, 2571, 2200, 1828,
        1685, 1907, 2481, 3242, 3938, 4318, 4234, 3688, 2844, 1959, 1295, 1020, 1152, 1555, 1999,
        2250, 2165, 1750, 1157, 630, 407, 629, 1283, 2200, 3116, 3770, 3992, 3769, 3242, 2649,
        2234, 2149, 2400, 2844, 3247, 3379, 3104, 2440, 1555, 711, 165, 81, 461, 1157, 1918, 2492,
        2714, 2571, 2200, 1828, 1685, 1907, 2481, 3242, 3938, 4318, 4234, 3688, 2844, 1959, 1295,
        1020, 1152, 1555, 1999, 2250, 2165, 1750, 1157, 630, 407, 629, 1283, 2200, 3116, 3770,
        3992, 3769, 3242, 2649, 2234, 2149, 2400, 2844, 3247, 3379, 3104, 2440, 1555, 711, 165, 81,
        461, 1157, 1918, 2492, 2714, 2571, 2200, 1828, 1685, 1907, 2481, 3242, 3938, 4318, 4234,
        3688, 2844, 1959, 1295, 1020, 1152, 1555, 1999, 2250, 2165, 1750, 1157, 630, 407, 629,
        1283, 2200, 3116, 3770, 3992, 3769, 3242, 2649, 2234, 2149, 2400, 2844, 3247, 3379, 3104,
        2440, 1555, 711, 165, 81, 461, 1157, 1918, 2492, 2714, 2571, 2200, 1828, 1685, 1907, 2481,
        3242, 3938, 4318, 4234, 3688, 2844, 1959, 1295, 1020, 1152, 1555, 1999, 2250, 2165, 1750,
        1157, 630, 407, 629, 1283, 2200, 3116, 3770, 3992, 3769, 3242, 2649, 2234, 2149, 2400,
        2844, 3247, 3379, 3104, 2440, 1555, 711, 165, 81, 461, 1157, 1918, 2492, 2714, 2571, 2200,
        1828, 1685, 1907, 2481, 3242, 3938, 4318, 4234, 3688, 2844, 1959, 1295, 1020, 1152, 1555,
        1999, 2250, 2165, 1750, 1157, 630, 407, 629, 1283, 2200, 3116, 3770, 3992, 3769, 3242,
        2649, 2234, 2149, 2400, 2844, 3247, 3379, 3104, 2440, 1555, 711, 165, 81, 461, 1157, 1918,
        2492, 2714, 2571, 2200, 1828, 1685, 1907, 2481, 3242, 3938, 4318, 4234, 3688, 2844, 1959,
        1295, 1020, 1152, 1555, 1999, 2250, 2165, 1750, 1157, 630, 407, 629, 1283, 2200, 3116,
        3770, 3992, 3769, 3242, 2649, 2234, 2149, 2400, 2844, 3247, 3379, 3104, 2440, 1555, 711,
        165, 81, 461, 1157, 1918, 2492, 2714, 2571, 2200, 1828, 1685, 1907, 2481, 3242, 3938, 4318,
        4234, 3688, 2844, 1959, 1295, 1020, 1152, 1555, 1999, 2250, 2165, 1750, 1157, 630, 407,
        629, 1283, 2200, 3116, 3770, 3992, 3769, 3242, 2649, 2234, 2149, 2400, 2844, 3247, 3379,
        3104, 2440, 1555, 711, 165, 81, 461, 1157, 1918, 2492, 2714, 2571, 2200, 1828, 1685, 1907,
        2481, 3242, 3938, 4318, 4234, 3688, 2844, 1959, 1295, 1020, 1152, 1555, 1999, 2250, 2165,
        1750, 1157, 630, 407, 629, 1283, 2200, 3116, 3770, 3992, 3769, 3242, 2649, 2234, 2149,
        2400, 2844, 3247, 3379, 3104, 2440, 1555, 711, 165, 81, 461, 1157, 1918, 2492, 2714, 2571,
        2200, 1828, 1685, 1907, 2481, 3242, 3938, 4318, 4234, 3688, 2844, 1959, 1295, 1020, 1152,
        1555, 1999, 2250, 2165, 1750, 1157, 630, 407, 629, 1283
    ];

    let mut radar_number: [[u16; RADAR_SAMPLES]; BINS] = [[0; RADAR_SAMPLES]; BINS];

    for i in 0..BINS {
        // Generate a dummy array of 64x500
        radar_number[i] = sample;
    }

    let output_bins = compute_fft(&mut radar_number);
    println!("{}", output_bins);
}

fn compute_fft(input_u16: &mut [[u16; RADAR_SAMPLES]; BINS]) -> u8 {
    let mut max_idx_i: usize = 5;
    for i in 0..BINS {
        let mut max_magnitude_total: f32 = 0.0;
        

        let mut f32_bin: [f32; FLOAT_SAMPLES] = [0.0; FLOAT_SAMPLES];

        for j in 0..RADAR_SAMPLES {
            f32_bin[j] = input_u16[i][j] as f32;
        }
        // println!("{:?}", f32_bin);
        let fft = rfft_512(&mut f32_bin);
        // println!("{:?}", fft);
        fft[0] = Complex32::new(0.0, 0.0);
        // println!("{:?}", fft);
        let mut max_magnitude_bin: f32 = 0.0;
        let mut max_idx_bin: u16 = 0;
        for k in 0..FFT_SIZE {
            let temp_magnitude: f32 = ((fft[k].re as f32).powf(2.0) + (fft[k].im as f32).powf(2.0)).sqrt();
            // print!("{}, ", temp_magnitude);
            if temp_magnitude > max_magnitude_bin {
                max_magnitude_bin = temp_magnitude;
                max_idx_bin = k as u16;
            }
        }
        println!("{:?} {}", max_magnitude_bin, max_idx_bin);

        if max_magnitude_bin > max_magnitude_total {
            max_magnitude_total = max_magnitude_bin;
            max_idx_i = i;
        }
    }
    return max_idx_i as u8
}

