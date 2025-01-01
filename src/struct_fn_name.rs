// This currently export both the function and the struct
pub use aligned::Aligned;

mod aligned {
    // Renaming this struct will rename the `pub use` above, breaking the use in the test
    pub struct Aligned {
        size: usize,
    }

    #[allow(non_snake_case)]
    pub fn Aligned(size: usize) -> Aligned {
        Aligned {
            size
        }
    }
}

#[test]
fn aligned_rename_failure() {
    Aligned(5);
}

