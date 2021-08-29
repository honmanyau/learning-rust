// Write a function that returns the string `"Hello, World!"`.
pub fn hello_world() -> String {
  return String::from("Hello, World!");
}

// Write a function that concatenate two strings with the `+` operator.
pub fn string_concat_a(a: &str, b: &str) -> String {
  return String::from(a) + b;
}

// Write a function that concatenate two strings with the `format!` macro.
pub fn string_concat_b(a: &str, b: &str) -> String {
  return format!("{}{}", a, b);
}

// Write a function that concatenate two strings without using the `+` operator
// and the `!format` macro.
pub fn string_concat_c(a: &str, b: &str) -> String {
  let mut result = String::from(a);

  result.push_str(b);

  return result;
}

// Write a function that converts a string to lower cases.
pub fn lowercase(s: &str) -> String {
  return s.to_lowercase();
}

// Write a function that reverses the graphemes in a string.
pub fn reverse(s: &str) -> String {
  return s.chars().rev().collect();
}

// Write a function that reverses a vector of characters using a `for` loop
// **in-place**.
pub fn reverse_vec(mut v: Vec<char>) -> Vec<char> {
  let l = v.len();

  for start in 0..l {
      let end = l - 1 - start;

      if start == end { return v; }

      v.swap(start, end);
  }

  return v;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_hello_world() {
      let result = hello_world();

      assert_eq!(result, "Hello, World!");
  }

  #[test]
  fn test_string_concat_a() {
      let a = "Hello, ";
      let b = "World!";
      let result = string_concat_a(a, b);

      assert_eq!(result, "Hello, World!");
  }
  
  #[test]
  fn test_string_concat_a_first_empty() {
      let a = "";
      let b = "World!";
      let result = string_concat_a(a, b);

      assert_eq!(result, "World!");
  }

  #[test]
  fn test_string_concat_a_both_empty() {
      let a = "";
      let b = "";
      let result = string_concat_a(a, b);

      assert_eq!(result, "");
  }

  #[test]
  fn test_string_concat_b() {
      let a = "Hello, ";
      let b = "World!";
      let result = string_concat_b(a, b);

      assert_eq!(result, "Hello, World!");
  }

  #[test]
  fn test_string_concat_b_first_empty() {
      let a = "";
      let b = "World!";
      let result = string_concat_b(a, b);

      assert_eq!(result, "World!");
  }

  #[test]
  fn test_string_concat_b_both_empty() {
      let a = "";
      let b = "";
      let result = string_concat_b(a, b);

      assert_eq!(result, "");
  }

  #[test]
  fn test_string_concat_c() {
      let a = "Hello, ";
      let b = "World!";
      let result = string_concat_c(a, b);

      assert_eq!(result, "Hello, World!");
  }

  #[test]
  fn test_string_concat_c_first_empty() {
      let a = "";
      let b = "World!";
      let result = string_concat_c(a, b);

      assert_eq!(result, "World!");
  }

  #[test]
  fn test_string_concat_c_both_empty() {
      let a = "";
      let b = "";
      let result = string_concat_c(a, b);

      assert_eq!(result, "");
  }

  #[test]
  fn test_lowercase() {
      let result = lowercase("Hello, World!");

      assert_eq!(result, "hello, world!");
  }

  #[test]
  fn test_lowercase_empty() {
      let result = lowercase("");

      assert_eq!(result, "");
  }

  #[test]
  fn test_reverse() {
      let result = reverse("hello, world!");
      
      assert_eq!(result, "!dlrow ,olleh");
  }

  #[test]
  fn test_reverse_empty() {
      let result = reverse("");
      
      assert_eq!(result, "");
  }

  #[test]
  fn test_reverse_single_grapheme() {
      let result = reverse("");
      
      assert_eq!(result, "");
  }

  #[test]
  fn test_reverse_vec_hello() {
      let v = vec!['h', 'e', 'l', 'l', 'o'];
      let result = reverse_vec(v);

      assert_eq!(result[0], 'o');
      assert_eq!(result[1], 'l');
      assert_eq!(result[2], 'l');
      assert_eq!(result[3], 'e');
      assert_eq!(result[4], 'h');
  }

  #[test]
  fn test_reverse_vec_hannah() {
      let v = vec!['h', 'a', 'n', 'n', 'a', 'h'];
      let result = reverse_vec(v);

      assert_eq!(result[0], 'h');
      assert_eq!(result[1], 'a');
      assert_eq!(result[2], 'n');
      assert_eq!(result[3], 'n');
      assert_eq!(result[4], 'a');
      assert_eq!(result[5], 'h');
  }

  #[test]
  fn test_reverse_vec_empty() {
      let v = vec![];
      let result = reverse_vec(v);

      assert_eq!(result.len(), 0);
  }
}
