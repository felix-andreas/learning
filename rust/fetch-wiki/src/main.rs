fn main() {
    let url = "https://en.wikipedia.org/wiki/Albert_Einstein";
    let response = reqwest::blocking::get(url).unwrap().text().unwrap();
    let search = "Einstein";
    let occurrences = response.matches("Albert Einstein").count();
    println!("occurrences of {} = {}", search, occurrences);
}
