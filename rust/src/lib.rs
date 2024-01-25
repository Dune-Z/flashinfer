#[cfg(test)]
mod tests {
    use super::*;

    extern "C" {
        fn vec_add(a: *const f32, b: *const f32, c: *mut f32, n: i32);
    }

    #[test]
    fn it_works() {
        let vec_a = vec![1.0, 2.0, 3.0];
        let vec_b = vec![4.0, 5.0, 6.0];
        let mut vec_c: Vec<f32> = vec![0.0, 0.0, 0.0];
        unsafe {
            vec_add(
                vec_a.as_ptr(),
                vec_b.as_ptr(),
                vec_c.as_mut_ptr(),
                vec_a.len() as i32,
            );
        }
        // print the result
        for &item in vec_c.iter() {
            println!("{}", item);
        }
    }
}
