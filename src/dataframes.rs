use polars::prelude::*;
use std::fs::File;

pub fn write_csv(results: Vec<Vec<f32>>, fname: &str, size: usize) {
    let mut series_vec: Vec<Series> = Vec::new();
    for i in 0..size + 1 {
        let x: Vec<f32> = results.iter().map(|x| *x.get(i).unwrap()).collect();
        series_vec.push(Series::new(&i.to_string(), x))
    }
    let df = DataFrame::new(series_vec).unwrap();
    let mut file = File::create(fname).expect("could not create file");
    match CsvWriter::new(&mut file)
        .has_header(true)
        .with_delimiter(b',')
        .finish(&df)
    {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
