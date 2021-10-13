fn main() {
    let magazine = "two times three is not four".split_whitespace().collect::<Vec<&str>>();
    let note = "two times two is four".split_whitespace().collect::<Vec<&str>>();

    println!("magazine: {}", magazine);
    println!("note: {}", note)

}