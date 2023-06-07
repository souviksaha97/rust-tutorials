// #![no_std]

// External Includes
use dsp::window;
use microfft::{real::rfft_512, Complex32};
use micromath::F32Ext;

// Constants
pub const RADAR_SAMPLES: usize = 500;
pub const BINS: usize = 64;
pub const FLOAT_SAMPLES: usize = 512;
pub const FFT_SIZE: usize = FLOAT_SAMPLES / 2;
pub const DETECT_START_SAMPLE: usize = RADAR_SAMPLES / 8;
pub const DETECT_END_SAMPLE: usize = RADAR_SAMPLES / 2;
pub const THRESHOLD_PRESENCE: f32 = 0.0007;

fn main() {
    let mut alpha_slow: f32 = 0.001;
    let mut alpha_med: f32 = 0.05;
    let mut alpha_fast: f32 = 0.6;
    let mut first_run: bool = true;
    let mut slow_avg: [f32; 256] = [0.0; FFT_SIZE];
    let mut fast_avg: [f32; 256] = [0.0; FFT_SIZE];
    let mut presence_status: bool = false;
    // println!("Hello, world!");

    // Generate the dummy matrix
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
    let mut norm_fft = [0.0; FFT_SIZE];
    fft_spectrum(&mut radar_number, &mut norm_fft);

    if first_run {
        slow_avg = norm_fft;
        fast_avg = norm_fft;
        first_run = false;
    }

    let mut alpha_used: f32 = 0.0;

    if presence_status == false {
        alpha_used = alpha_med;
    } else {
        alpha_used = alpha_slow;
    }

    // let test = [3.0; FFT_SIZE];
    // println!("{:?}", k_multiply(10.0, test));
    slow_avg = add_array(
        k_multiply(1.0 - alpha_used, slow_avg),
        k_multiply(alpha_used, norm_fft),
    );

    fast_avg = add_array(
        k_multiply(1.0 - alpha_fast, fast_avg),
        k_multiply(alpha_fast, norm_fft),
    );

    let data = sub_array(fast_avg, slow_avg);

    let presence = max_array(slicer_array(data)) > THRESHOLD_PRESENCE;
    println!("{}", presence);
}

fn fft_spectrum(input_u16: &mut [[u16; RADAR_SAMPLES]; BINS], fft_norm: &mut [f32; FFT_SIZE]) {
    let win = window::blackman(FLOAT_SAMPLES);
    let mut mag_array = [[0.0; FFT_SIZE]; BINS];
    for i in 0..BINS {
        let mut f32_bin: [f32; FLOAT_SAMPLES] = [0.0; FLOAT_SAMPLES];

        for j in 0..RADAR_SAMPLES {
            f32_bin[j] = input_u16[i][j] as f32;
        }

        let mut windowed_bin: [f32; FLOAT_SAMPLES] = [0.0; FLOAT_SAMPLES];
        win.apply(&f32_bin, &mut windowed_bin);

        let fft = rfft_512(&mut windowed_bin);
        fft[0] = Complex32::new(0.0, 0.0);

        for j in 0..FFT_SIZE {
            mag_array[i][j] = ((fft[j].re as f32).powf(2.0) + (fft[j].im as f32).powf(2.0)).sqrt();
        }
    }

    for i in 0..FFT_SIZE {
        for j in 0..BINS {
            fft_norm[i] += mag_array[j][i];
        }
        fft_norm[i] /= RADAR_SAMPLES as f32;
    }
}

fn k_multiply(k: f32, array: [f32; FFT_SIZE]) -> [f32; FFT_SIZE] {
    let mut temp: [f32; FFT_SIZE] = [0.0; FFT_SIZE];

    for i in 0..FFT_SIZE {
        temp[i] = array[i] * k;
    }
    return temp;
}

fn add_array(arr1: [f32; FFT_SIZE], arr2: [f32; FFT_SIZE]) -> [f32; FFT_SIZE] {
    let mut temp: [f32; FFT_SIZE] = [0.0; FFT_SIZE];

    for i in 0..FFT_SIZE {
        temp[i] = arr1[i] + arr2[i];
    }
    return temp;
}

fn sub_array(arr1: [f32; FFT_SIZE], arr2: [f32; FFT_SIZE]) -> [f32; FFT_SIZE] {
    let mut temp: [f32; FFT_SIZE] = [0.0; FFT_SIZE];

    for i in 0..FFT_SIZE {
        temp[i] = arr1[i] - arr2[i];
    }
    return temp;
}

fn max_array(arr: [f32; DETECT_END_SAMPLE - DETECT_START_SAMPLE]) -> f32 {
    let mut max = arr[0];
    for i in 0..DETECT_END_SAMPLE - DETECT_START_SAMPLE {
        if arr[i] > max {
            max = arr[i];
        }
    }
    return max;
}

fn slicer_array(arr: [f32; FFT_SIZE]) -> [f32; DETECT_END_SAMPLE-DETECT_START_SAMPLE] {
    let mut slice = [0.0; DETECT_END_SAMPLE - DETECT_START_SAMPLE];
    for i in DETECT_START_SAMPLE..DETECT_END_SAMPLE {
        slice[i-DETECT_START_SAMPLE] = arr[i];
    }
    return slice;
}
