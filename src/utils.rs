pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
	let digits_end = s
		.char_indices()
		.find_map(|(idx, c)| if c.is_ascii_digit() { None } else { Some(idx) })
		.unwrap_or_else(|| s.len());

	let digits = &s[..digits_end];
	let remainder = &s[digits_end..];

	(remainder, digits)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
	match &s[0..1] {
		"+" | "-" | "*" | "/" => {}
		_ => panic!("[ Error ] no such operator: {}", &s[0..1]),
	}

	(&s[1..], &s[0..1])
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn extract_one_digit() {
		assert_eq!(extract_digits("1+2"), ("+2", "1"));
	}

	#[test]
	fn extract_multiple_digits() {
		assert_eq!(extract_digits("100+200"), ("+200", "100"));
	}

	#[test]
	fn do_not_extract_anything_from_empty_input() {
		assert_eq!(extract_digits(""), ("", ""));
	}

	#[test]
	fn extract_digits_with_no_remainder() {
		assert_eq!(extract_digits("100"), ("", "100"));
	}

	#[test]
	fn extract_plus() {
		assert_eq!(extract_op("+100"), ("100", "+"));
	}

	#[test]
	fn extract_sub() {
		assert_eq!(extract_op("-100"), ("100", "-"));
	}

	#[test]
	fn extract_mul() {
		assert_eq!(extract_op("*100"), ("100", "*"));
	}

	#[test]
	fn extract_div() {
		assert_eq!(extract_op("/100"), ("100", "/"));
	}
}
