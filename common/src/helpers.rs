pub fn check_bounds<T>(src_len: usize, min_len: (u16, T), max_len: (u16, T)) -> Result<(), T> {
	if src_len < min_len.0 as usize {
		return Err(min_len.1)
	}
	if src_len > max_len.0 as usize {
		return Err(max_len.1)
	}

	Ok(())
}
