// https://docs.rs/utah/0.1.2/utah/
extern crate utah;

use utah::prelude::*;

fn main() {
    println!("Hello, world!");

    // test sample df
    let c = arr2(&[[2., 6.], [3., 4.], [2., 1.]]);
    let mut df: DataFrame<f64> = DataFrame::new(c)
                                            .columns(&["a", "b"]).unwrap()
                                            .index(&["1", "2", "3"]).unwrap();

    // this works
    println!("{:?}", df);


    // try to read in from file
    // let file_name = "ncbiRefSeq.txt";
    // let df: Result<DataFrame<f64>> = DataFrame::read_tsv(file_name);
    // let df: Result<DataFrame<f64>> = DataFrame::read_csv("../data/summary-combined.wes.csv");
    let df: Result<DataFrame<f64>> = DataFrame::read_csv("../data/uspop.csv");
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Decode("Failed converting \'Davidsons Landing\' from str.")', src/libcore/result.rs:906:4
    // doesnt work...


}
