use matmul_guest::{matmul, MyArray};

fn main() {
    let m1 = MyArray::default();
    let m2 = MyArray::default();
    let m3 = MyArray::default();

    let b = matmul(m1, m2, m3);
    if !b {
        panic!("error matmul");
    }
}
