// The name "canrun" is not available from within the crate for testing.
// I think this workaround should work ~95% of the time. I guess it
// will fall down if someone renames the crate or something.
// https://github.com/rust-lang/rust/issues/54363
use crate as canrun;
use canrun_codegen::domains;

domains! {
    pub domain Numbers {
        i32,
    }
}