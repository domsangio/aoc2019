mod d1;
mod d2;

fn main() {
    let d1_contents = std::fs::read_to_string("input/1.txt").expect("Unable to read file");
    d1::runner(&d1_contents);

    let d2_contents = std::fs::read_to_string("input/2.txt").expect("Unable to read file");
    d2::runner(&d2_contents);
}
