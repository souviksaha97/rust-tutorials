// #![no_std]

// External Includes
use dsp::window;
use microfft::{real::rfft_512, Complex32};
use micromath::F32Ext;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

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

    let file_path = "/home/souvik/GitLab/Embedded/rust-tutorials/json_pen/jumbo.json";

    // Read the JSON file
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Failed to read file");

    // Parse the JSON content
    let json_data: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");

    // Iterate over the keys in the JSON object
    if let Value::Object(obj) = json_data {
        for (key, value) in obj {
            // Parse the value as a 64x500 array
            if let Value::Array(array) = value {
                // Ensure the array has the correct dimensions
                if array.len() == BINS {
                    let mut content: [[f32; RADAR_SAMPLES]; BINS] = [[0.0; RADAR_SAMPLES]; BINS];

                    for (i, row) in array.iter().enumerate().take(BINS) {
                        if let Value::Array(row_array) = row {
                            // Ensure each row has the correct number of elements
                            if row_array.len() == RADAR_SAMPLES {
                                for (j, element) in row_array.iter().enumerate().take(RADAR_SAMPLES)
                                {
                                    if let Value::Number(num) = element {
                                        if let Some(num) = num.as_f64() {
                                            content[i][j] = num as f32;
                                        }
                                    }
                                }
                            } else {
                                panic!("Invalid number of elements in row");
                            }
                        } else {
                            panic!("Invalid row format");
                        }
                    }

                    // Print or use the content of the 64x500 array
                    // println!("Key: {}", key);
                    let mut norm_fft = [0.0; FFT_SIZE];
                    let max_bin = fft_spectrum(&mut content, &mut norm_fft);

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
                    println!("{}, {}", presence, max_bin);
                } else {
                    panic!("Invalid number of rows in array");
                }
            } else {
                panic!("Invalid array format");
            }
        }
    }
}

fn fft_spectrum(
    input_u16: &mut [[f32; RADAR_SAMPLES]; BINS],
    fft_norm: &mut [f32; FFT_SIZE],
) -> u8 {
    let mut max_idx_total: usize = 0;
    let mut max_magnitude_total: f32 = 0.0;

    let win = window::blackman(FLOAT_SAMPLES);
    let mut mag_array = [[0.0; FFT_SIZE]; BINS];
    for i in 0..BINS {
        let mut f32_bin: [f32; FLOAT_SAMPLES] = [0.0; FLOAT_SAMPLES];

        for j in 0..RADAR_SAMPLES {
            // Zero padding
            f32_bin[j + 6] = input_u16[i][j] as f32;
        }

        let mut windowed_bin: [f32; FLOAT_SAMPLES] = [0.0; FLOAT_SAMPLES];
        win.apply(&f32_bin, &mut windowed_bin);

        let fft = rfft_512(&mut windowed_bin);
        fft[0] = Complex32::new(0.0, 0.0);
        let mut max_magnitude_bin: f32 = 0.0;
        for j in 0..FFT_SIZE {
            mag_array[i][j] = ((fft[j].re as f32).powf(2.0) + (fft[j].im as f32).powf(2.0)).sqrt();
            if mag_array[i][j] > max_magnitude_bin {
                max_magnitude_bin = mag_array[i][j];
            }
        }

        if max_magnitude_bin > max_magnitude_total {
            max_magnitude_total = max_magnitude_bin;
            max_idx_total = i;
        }
    }

    for i in 0..FFT_SIZE {
        for j in 0..BINS {
            fft_norm[i] += mag_array[j][i];
        }
        fft_norm[i] /= RADAR_SAMPLES as f32;
    }

    return max_idx_total as u8;
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

fn slicer_array(arr: [f32; FFT_SIZE]) -> [f32; DETECT_END_SAMPLE - DETECT_START_SAMPLE] {
    let mut slice = [0.0; DETECT_END_SAMPLE - DETECT_START_SAMPLE];
    for i in DETECT_START_SAMPLE..DETECT_END_SAMPLE {
        slice[i - DETECT_START_SAMPLE] = arr[i];
    }
    return slice;
}
