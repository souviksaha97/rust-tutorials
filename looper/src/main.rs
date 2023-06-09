use std::fs::File;
use std::io::Read;
use serde_json::Value;

const ROWS: usize = 64;
const COLS: usize = 500;

fn main() {
    // Specify the path to the JSON file
    let file_path = "/home/souvik/GitLab/Embedded/rust-tutorials/json_pen/jumbo.json";

    // Read the JSON file
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Failed to read file");

    // Parse the JSON content
    let json_data: Value = serde_json::from_str(&file_content).expect("Failed to parse JSON");

    // Iterate over the keys in the JSON object
    if let Value::Object(obj) = json_data {
        for (key, value) in obj {
            // Parse the value as a 64x500 array
            if let Value::Array(array) = value {
                // Ensure the array has the correct dimensions
                if array.len() == ROWS {
                    let mut content: [[f32; COLS]; ROWS] = [[0.0; COLS]; ROWS];

                    for (i, row) in array.iter().enumerate().take(ROWS) {
                        if let Value::Array(row_array) = row {
                            // Ensure each row has the correct number of elements
                            if row_array.len() == COLS {
                                for (j, element) in row_array.iter().enumerate().take(COLS) {
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
                    println!("Key: {}", key);
                    // println!("Content: {:?}", content);
                } else {
                    panic!("Invalid number of rows in array");
                }
            } else {
                panic!("Invalid array format");
            }
        }
    }
}
