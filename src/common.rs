pub fn slice_to_usize(slice: &[u8]) -> usize {
    let str = unsafe { std::str::from_utf8_unchecked(slice) };
    str.parse::<usize>().unwrap()
}
