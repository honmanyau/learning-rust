fn main() {
    let s1 = String::from("nyanpasu");
    let s2 = "nyanpasu".to_string();
    let mut s3 = String::from("nyan");
    let mut s4 = String::from("nyan");
    let s5 = format!("{}{}", "nyan", "pasu");

    s3.push_str("pasu"); // Note that "pasu" is of type str. This is coercion.
    s4 += "pasu"; // Note that "pasu" is of type str. This is coercion.

    println!("s1 is equal to s2: {}", s1 == s2);
    println!("s1 is equal to s3: {}", s1 == s3);
    println!("s1 is equal to s4: {}", s1 == s4);
    println!("s1 is equal to s4: {}", s1 == s5);
}
