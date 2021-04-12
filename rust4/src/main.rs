fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone();

    
    let l = calc_size_by_ref(&s1);
    println!("len1 {}", l);


    let (mut s1, l) = takes_ownership(s1);
    println!("len {}", l);
    {
        println!("s1 {}, s2 {}", s1, s2);
        s1.push_str(", world!"); // push_str() appends a literal to a String
    }
    let r1 = &mut s2;
    change(r1);

    change(&mut s2);
    println!("s1 {}, s2 {}", s1, s2);
}

fn takes_ownership(sz: String) -> (String, usize) {
    println!("inside {}", sz);
    let len = sz.len();
    (sz, len)
}
fn calc_size_by_ref(sz: &String) -> usize {
    println!("inside {}", sz);
    sz.len()
}

fn change(sz: &mut String) {
    sz.push_str(", somewhere");
}
fn new_string() -> String {
    let sz = String::from("over the rainbow");
    sz
}
fn find_between(sz: &str, start_tag: char, end_date: char) -> &str
{
    let chars = sz.chars();
    for (i, &item) in chars.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}