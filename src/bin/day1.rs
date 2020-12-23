use std::fs;

fn main() {
//    let (x, y) = find_summing_to_2020(vec![1010, 232, 54, 1010]);
    let (x, y) = find_two_summing_to_2020(read_list("input/day1.txt"));
    println!("Two numbers are {} and {}, sum is {}, product is {}",
             x, y, x + y, x * y);
    let (x, y, z) = find_three_summing_to_2020(read_list("input/day1.txt"));
    println!("Three numbers are {} and {} and {}, sum is {}, product is {}",
             x, y, z, x + y + z, x * y * z);
}

fn find_three_summing_to_2020(numbers: Vec<u32>) -> (u32, u32, u32) {
    for x in numbers.iter() {
        for y in numbers.iter() {
            if *x + *y < 2020 {
                for z in numbers.iter() {
                    if *x + *y + *z == 2020 {
                        return (*x, *y, *z)
                    }
                }
            }
        }
    }

    panic!("Couldn't find any summing to 2020!");
}

fn find_two_summing_to_2020(numbers: Vec<u32>) -> (u32, u32) {
    for x in numbers.iter() {
        for y in numbers.iter() {
            if *x + *y == 2020 {
                return (*x, *y)
            }
        }
    }

    panic!("Couldn't find any summing to 2020!");
}

fn read_list(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename).expect("Error reading file!");
    let mut list: Vec<u32> = Vec::new();
    for line in contents.lines() {
        list.push(line.parse::<u32>().unwrap());
    }
    list
}
