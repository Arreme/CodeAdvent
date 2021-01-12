use std::str::FromStr;

fn main() {
    let _input = read_all::<u32>("src/input.txt");
    let mut _final_input = _input.unwrap();
    sort(&mut _final_input);
    for x in &_final_input {
        for y in &_final_input {
            for z in &_final_input {
                    match x+y+z {
                    2020 => println!("Found one! {}",x*y*z),
                    x if x > 2020 => {
                        break;
                    }
                    _ => {},
                }
            }
        }
    }
}

fn sort(array: &mut Vec<u32>) {
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
