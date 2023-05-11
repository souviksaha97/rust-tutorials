// #![no_std]
use plotters::prelude::*;
use microfft::real::rfft_128;

fn main() {
    let sine: [u16;128] = [
        499,  737,  917,  998,  958,  809,  586,  341,  135,   16,   13,
        127,  330,  573,  799,  953,  999,  924,  748,  512,  273,   89,
          3,   36,  180,  401,  646,  855,  979,  989,  880,  681,  438,
        210,   51,    0,   69,  241,  475,  715,  903,  995,  968,  828,
        610,  365,  153,   23,    8,  111,  307,  549,  779,  942, 1000,
        937,  769,  537,  295,  103,    6,   27,  162,  377,  622,  837,
        972,  993,  896,  704,  462,  230,   62,    0,   57,  220,  450,
        692,  888,  991,  976,  846,  634,  389,  171,   31,    4,   96,
        284,  524,  758,  930,  999,  948,  789,  561,  318,  119,   10,
         20,  144,  353,  598,  819,  963,  996,  910,  726,  487,  251,
         75,    0,   46,  200,  426,  669,  872,  986,  983,  864,  658,
        413,  190,   41,    1,   82,  262,  499
    ];

    let mut floater: [f32; 128] = [0.0; 128];

    for i in 0..sine.len() {
        floater[i] = sine[i] as f32;
    }

    let result = rfft_128(&mut floater);
    let mut magnitude: [i32; 64] = [0; 64];
    for i in 0..result.len() {
        // println!("{} - {}", i as f32/result.len() as f32 * 100.0, result[i].re);
        magnitude[i] = result[i].re as i32;
    }

    let root = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Line Chart", ("sans-serif", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..magnitude.len() as i32, 0..1000)
        .unwrap();

    // Draw the chart
    chart.configure_mesh().draw().unwrap();
    chart.draw_series(LineSeries::new(
        magnitude.iter().enumerate().map(|(i, &val)| (i as i32, val as i32)),
        &BLUE,
    )).unwrap();
}
