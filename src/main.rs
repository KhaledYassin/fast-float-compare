use fast_float_compare::Float;

fn main() {
    // let values = [1.23, 4.56, -7.89, 0.12];
    // let raw_floats: Vec<Float> = values
    //     .iter()
    //     .map(|&n| Float::from_f64(n).unwrap())
    //     .collect();
    // println!("{:?}", raw_floats);

    let a = Float::from_f64(1.23).unwrap();
    println!("{:?}", a);
}

