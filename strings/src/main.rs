fn main() {
    let data = "foo";

    let mut s1 = data.to_string();
    let s2  = "s";
    let s3 = "ss";
    let s5 = format!("{}{}{}", s1, s2, s3);
    println!("s1 is {}", s2);
}
