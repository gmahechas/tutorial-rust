fn main() {
    let grettings_first_part: String = get_gretings_first(String::from("Hello"));
    let grettings_second_part: String = get_gretings_second(String::from("World"));
    println!("{} {}", grettings_first_part, grettings_second_part);
}

fn get_gretings_first(grettings: String) -> String {
    format!("{}", grettings) // no semicolons because of the return
}

fn get_gretings_second(grettings: String) -> String {
    return format!("{}", grettings);
}
