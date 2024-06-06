use matmul_guest::{matmul, MyArray};

fn main() {
    let mat = MyArray::default();

    let b = matmul(mat);
    if !b {
        panic!("error matmul");
    }
}
