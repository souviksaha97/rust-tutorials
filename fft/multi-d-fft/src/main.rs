// #![no_std]
use microfft::{real::rfft_512, Complex32};
use micromath::F32Ext;

// #[repr(C)]
pub struct Presence {
    is_present: bool,
    bin_number: [u8; 3],
}

const RADAR_SAMPLES: usize = 500;
const BINS: usize = 64;
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
        1555, 1999, 2250, 2165, 1750, 1157, 630, 407, 629, 1283,
    ];

    let mut radar_number: [[u16; RADAR_SAMPLES]; BINS] = [[0; RADAR_SAMPLES]; BINS];

    for i in 0..BINS {
        // Generate a dummy array of 64x500
        radar_number[i] = sample;
    }

    let mut float_adc_data1: [[f32; FLOAT_SAMPLES]; BINS] = [[0.0; FLOAT_SAMPLES]; BINS];
    convert_u16_array_to_f32(&radar_number, &mut float_adc_data1);

    let mut fft_1: [[Complex32; FFT_SIZE]; BINS] = [[Complex32::new(0.0, 0.0); FFT_SIZE]; BINS];
    compute_fft(&mut float_adc_data1, &mut fft_1);

    let mut output_magnitude: [[f32; FFT_SIZE]; BINS] = [[0.0; FFT_SIZE]; BINS];
    calculate_magnitudes(&mut fft_1, &mut output_magnitude);

    let mut output_bins = Presence {
        is_present: false,
        bin_number: [0, 0, 0],
    };

    compute_max_index(&mut output_magnitude, &mut output_bins.bin_number[0]);
    println!("{}", output_bins.bin_number[0]);
}

fn convert_u16_array_to_f32(
    input_u16: &[[u16; RADAR_SAMPLES]; BINS],
    output_f32: &mut [[f32; FLOAT_SAMPLES]; BINS],
) {
    for i in 0..BINS {
        for j in 0..RADAR_SAMPLES {
            output_f32[i][j] = input_u16[i][j] as f32; // typecast to f32
        }
    }
    // println!("{:?}", output_f32);
}

fn compute_fft(
    input_f32: &mut [[f32; FLOAT_SAMPLES]; BINS],
    output_fft: &mut [[Complex32; FFT_SIZE]; BINS],
) {
    for i in 0..output_fft.len() {
        let result = rfft_512(&mut input_f32[i]);
        output_fft[i] = *result;
    }
    // println!("{:?}", output_fft);
}

fn calculate_magnitudes(
    input_fft: &mut [[Complex32; FFT_SIZE]; BINS],
    output_magnitude: &mut [[f32; FFT_SIZE]; BINS],
) {
    for i in 0..BINS {
        for j in 1..output_magnitude[0].len() {
            output_magnitude[i][j] = ((input_fft[i][j].re as f32).powf(2.0)
                + (input_fft[i][j].im as f32).powf(2.0))
            .sqrt();
        }
    }
    println!("{:?}", output_magnitude);
}

fn compute_max_index(output_magnitude: &mut [[f32; FFT_SIZE]; BINS], max_index: &mut u8) {
    let mut max_index_list: [u32; BINS] = [0; BINS];
    for i in 0..BINS {
        let mut idx = 0;
        for j in 0..FFT_SIZE {
            if output_magnitude[i][j] > output_magnitude[i][idx] {
                idx = j;
            }
        }
        max_index_list[i] = output_magnitude[i][idx] as u32; // calculate maximum magnitude in each bin
        // println!("{}", idx);
    }

    let mut idx = 0;
    for i in 0..BINS {
        if max_index_list[i] > max_index_list[idx] {
            // find bin with the highest magnitude
            idx = i;
        }
    }
    *max_index = idx as u8;
    // println!("{:?}", max_index_list);
}
