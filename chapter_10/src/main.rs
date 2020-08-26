use std::fmt::Display;

fn longest_with_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let char_list1 = "abcdg";
    let char_list2 = "qq";
    let ann = "ann";

    let result = longest_with_annoucement(&char_list1, &char_list2, ann);

    println!("The largest char is {}", result);
}
