use std::fs::read_to_string;


fn read(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        }else if reduced_line.starts_with("two"){
            '2'
        }else if reduced_line.starts_with("three"){
            '3'
        }else if reduced_line.starts_with("four"){
            '4'
        }else if reduced_line.starts_with("five"){
            '5'
        }else if reduced_line.starts_with("six"){
            '6'
        }else if reduced_line.starts_with("seven"){
            '7'
        }else if reduced_line.starts_with("eight"){
            '8'
        }else if reduced_line.starts_with("nine"){
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };
    
        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}


pub fn pt1() -> Option<i64> {
    let vec = read("./input");
    let mut sum: i64 = 0;

    for _item in vec{
        let mut first: char= 'a';
        let mut last: char= 'a';
        for _char in _item.chars() {
            if _char.is_numeric(){
                if first.is_alphabetic(){
                    first = _char;
                }
                else{
                    last = _char;
                }
            }
        }
        if first.is_numeric() && last.is_numeric() {
            let cal_str: String = format!("{}{}",first,last);
            let cal_num: i64 = cal_str.parse::<i64>().unwrap();
            sum += &cal_num;
        }
        else if first.is_numeric() {
            let cal_str: String = format!("{}{}",first,first);
            let cal_num: i64 = cal_str.parse::<i64>().unwrap();
            sum += &cal_num;
        }

    }

    Some(sum)
}


pub fn pt2() -> Option<i64> {
    let vec = read("./input");
    let mut sum: i64 = 0;


    // Make lookup table
    // fixup the vec to parse out the numbers


    for _item in vec{
        let mut first: char= 'a';
        let mut last: char= 'a';
        for _char in _item.chars() {
            if _char.is_numeric(){
                if first.is_alphabetic(){
                    first = _char;
                }
                else{
                    last = _char;
                }
            }
        }
        if first.is_numeric() && last.is_numeric() {
            let cal_str: String = format!("{}{}",first,last);
            let cal_num: i64 = cal_str.parse::<i64>().unwrap();
            sum += &cal_num;
        }
        else if first.is_numeric() {
            let cal_str: String = format!("{}{}",first,first);
            let cal_num: i64 = cal_str.parse::<i64>().unwrap();
            sum += &cal_num;
        }

    }

    Some(sum)
}

fn main() {
    print!("Part 1:\n");
    print!("{:?}\n",pt1().unwrap());
    print!("Part 2:\n");
    print!("{:?}\n",pt2().unwrap());

}
