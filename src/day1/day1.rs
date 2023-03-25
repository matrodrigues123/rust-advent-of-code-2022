use std::path;

fn calory_count() {
    let filepath = path::Path::new("input.txt");
    let input = fs::read_to_string(filepath).unwrap();
    let (mut first, mut second, mut third) = (0, 0, 0);
    for elf in input.split("\n\n") {
        let mut curr_sum = 0;
        for calory in elf.split("\n") {
            curr_sum += calory.parse::<u32>().unwrap();
        }
        if curr_sum > first{
            (first, second, third) = (curr_sum, first, second);
        } else if curr_sum > second {
            (second, third) = (curr_sum, second);
        } else if curr_sum > third {
            third = curr_sum;
        }

    }
    println!("First: {first}\n");
    println!("Second: {second}\n");
    println!("Third: {third}\n");
    println!("Sum: {}", first + second + third);
}