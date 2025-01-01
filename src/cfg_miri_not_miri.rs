// here we have two items, one will be disabled and one enabled,
// if I rename the one that is enabled, the other one won't get renamed

#[cfg(miri)]
const CONST: usize = 6;

#[cfg(not(miri))]
const CONST: usize = 8;
