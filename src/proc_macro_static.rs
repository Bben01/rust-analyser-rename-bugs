#[static_proc_macro::generate_static]
pub trait Test {}

#[static_proc_macro::use_static]
impl Test for () {
//   ^^^^
// Placing a caret on the Test above will make r-a panic
}
