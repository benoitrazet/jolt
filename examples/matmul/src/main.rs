use guest::MyArray;

pub fn main() {
    let (prove_matmul, verify_matmul) = guest::build_matmul();

    let m1 = MyArray::default();
    let m2 = MyArray::default();
    let m3 = MyArray::default();

    let t = std::time::Instant::now();
    let native_output = guest::matmul(m1.clone(), m2.clone(), m3.clone());
    let (output, proof) = prove_matmul(m1, m2, m3);
    println!("Time receipt computation:{:?}", t.elapsed());

    let t = std::time::Instant::now();
    let is_valid = verify_matmul(proof);
    println!("Time receipt verify:{:?}", t.elapsed());

    assert_eq!(output, native_output, "output mismatch");
    //println!("output: {}", hex::encode(output));
    //println!("native_output: {}", hex::encode(native_output));
    println!("valid: {}", is_valid);
}
