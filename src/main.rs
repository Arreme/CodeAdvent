use std::str::FromStr;

fn main() {
    let _input = read_all::<u16>("src/input.txt");
    let _final_input = _input.unwrap();
    for x in _final_input {
        println!("{}",x);
    }
    //sort
    //add
}

fn read_all<T: FromStr>(file_name: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}







