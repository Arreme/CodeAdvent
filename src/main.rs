use std::str::FromStr;

fn main() {
    let _input = read_all::<u16>("src/input.txt");
    let mut _final_input = _input.unwrap();
    sort(&mut _final_input);
    for x in _final_input {
        println!("{}", x);
    }
    //add
}

fn sort(array: &mut Vec<u16>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
            }
        }
    }
}

fn read_all<T: FromStr>(file_name: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}
