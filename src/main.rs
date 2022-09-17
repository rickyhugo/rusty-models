use csv::{Reader, ReaderBuilder};
use ndarray::{Array1, Array2};
use std::fs::File;
// use ndarray_csv::Array2Reader;

#[derive(Debug)]
struct DataFrame {
    n_cols: usize,
    n_rows: usize,
    user_id: Vec<String>,
    gender: Vec<String>,
    age: Vec<f32>,
    salary: Vec<f32>,
    purchased: Vec<i32>,
}

impl DataFrame {
    fn new() -> DataFrame {
        DataFrame {
            n_cols: 0usize,
            n_rows: 0usize,
            user_id: Vec::new(),
            gender: Vec::new(),
            age: Vec::new(),
            salary: Vec::new(),
            purchased: Vec::new(),
        }
    }

    fn read_csv(path: &str, has_headers: bool) -> DataFrame {
        let file: File = File::open(path).unwrap();
        let mut rdr: Reader<File> = ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);
        let mut data_frame = DataFrame::new();
        let mut n_rows: usize = 0;
        for result in rdr.records().into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
            n_rows += 1;
        }
        data_frame.n_rows = n_rows;
        data_frame.n_cols = rdr.headers().unwrap().len();
        return data_frame;
    }

    fn push(&mut self, row: &csv::StringRecord) {
        self.user_id.push(row[0].to_string());
        self.gender.push(row[1].to_string());
        self.age.push(row[2].parse().unwrap());
        self.salary.push(row[3].parse().unwrap());
        self.purchased.push(row[4].parse().unwrap());
    }
}

fn main() {
    // read data@@
    let path = "data/data.csv";
    let data = DataFrame::read_csv(path, true);
    println!("{:?}", data.n_cols);

    // least squares
    let vec1 = vec![1, 2, 3, 4];
    let vec2: Vec<i32> = vec![2, 2];
    let mat = Array2::from_shape_vec((2, 2), vec1).unwrap();
    let col_vec = Array1::from(vec2);
    let arr = mat.dot(&col_vec);
    println!("Array {:?}", arr);
}
