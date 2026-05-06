pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
	let s = "hello";
	let s1 = "camelCase".to_string();
	let s2 = "olá!";

	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
	println!("\tstr_len(\"{}\") = {}", s2, str_len(s2));
}
}