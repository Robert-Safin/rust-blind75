#[allow(non_snake_case)]
#[allow(dead_code)]
mod problems;

fn main() {
    problems::array_string::String_Compression_443::compress(&mut vec![
        'a', 'a', 'b', 'b', 'c', 'c', 'c',
    ]);
}
