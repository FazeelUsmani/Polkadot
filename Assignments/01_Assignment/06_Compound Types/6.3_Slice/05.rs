
fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    
    let slice = &s[0..3];
    println!("{}", "你".chars().count());
    println!("Length of slice is {}", slice.len());
    
    assert!(slice == "你");

    println!("Success!");
}
