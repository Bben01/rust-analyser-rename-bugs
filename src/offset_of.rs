pub struct Offset {
    // renaming inner here won't rename inner in the offset_of! macro
    inner: Inner,
}

pub struct Inner {
    // renaming `a` here won't rename `a` in the offset_of! macro
    a: u8,
}

#[test]
fn rename_offset_of() {
    // Trying to rename inner or `a` from here results in a "No reference found at position"
    std::mem::offset_of!(Offset, inner.a);
}