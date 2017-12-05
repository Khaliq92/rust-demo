extern crate utah;

use utah::prelude::*;

fn main() {
    println!("Hello, world!");
    
    // let file_name = "ncbiRefSeq.txt";
    // let df: Result<DataFrame<f64>> = DataFrame::read_tsv(file_name);
    let df: Result<DataFrame<f64>> = DataFrame::read_csv("summary-combined.wes.csv");
    
    
}
