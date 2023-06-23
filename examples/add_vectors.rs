use x_rust::x_vector::add_vectors;

fn main() {
    let vector1 = &[1, 2, 3, 4];
    let vector2 = &[5, 6, 7, 8];
    match add_vectors(vector1, vector2) {
        Ok(result) => println!("{:?}", result),
        Err(error) => println!("{}", error),
    }
}
