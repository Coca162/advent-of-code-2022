const INPUT: &str = include_str!("input.txt");

fn main() {
    // get raw file
    let clean = INPUT.replace("\r\n", "\n");

    //parse by getting elves first 
    let elf_calories = clean.split_terminator("\n\n").map(|elf| 
        elf.split_terminator('\n').map(|calorie| calorie.parse::<u32>().expect("Failed calorie conversion")).sum::<u32>()
    );

    let max = elf_calories.clone().max().expect("Failed at getting maximum!");

    println!("{max}");

    let mut top_3 = vec![0, 0, 0];

    for new in elf_calories {
        let (position, old) = top_3.iter().enumerate().min_by_key(|x| x.1).expect("Failed at getting max!");

        if &new > old {
            top_3[position] = new;
        }
    }

    let top_3_sum: u32 = top_3.iter().sum();

    println!("{top_3_sum}");
}