#![cfg_attr(feature = "guest", no_std)]
#![no_main]

pub const N: usize = 19; // Size of the square matrices

use core::fmt;
use serde::{
    de::{SeqAccess, Visitor},
    ser::{SerializeSeq, Serializer},
    Deserialize, Deserializer, Serialize,
};

pub struct MyArray([i32; N * N]);

impl Default for MyArray {
    fn default() -> Self {
        MyArray([0i32; N * N])
    }
}

impl Serialize for MyArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(N * N))?;
        for element in self.0.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for MyArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ArrayVisitor;

        impl<'de> Visitor<'de> for ArrayVisitor {
            type Value = MyArray;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an array of 100 i32 elements")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<MyArray, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut arr = [0; N * N];
                for i in 0..N * N {
                    arr[i] = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                }
                Ok(MyArray(arr))
            }
        }

        deserializer.deserialize_seq(ArrayVisitor)
    }
}

pub fn matrix_multiply(a: &MyArray, b: &MyArray) -> MyArray {
    let mut result = [0; N * N];

    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                result[i * N + j] += a.0[i * N + k] * b.0[k * N + j];
            }
        }
    }

    MyArray(result)
}

pub fn check_matrix_equal(a: &MyArray, b: &MyArray) -> bool {
    for i in 0..N {
        for j in 0..N {
            if a.0[i * N + j] != b.0[i * N + j] {
                return false;
            }
        }
    }

    true
}

#[cfg(feature = "main_program")]
pub fn matmul(i: MyArray) -> bool {
    let t = matrix_multiply(&i, &i);
    check_matrix_equal(&t, &i)
}

#[cfg(not(feature = "main_program"))]
#[jolt::provable]
fn matmul(i: MyArray) -> bool {
    let t = matrix_multiply(&i, &i);
    check_matrix_equal(&t, &i)
}
