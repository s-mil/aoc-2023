use std::fs::read_to_string;



// fn read<R: Read>(io: R) -> Result<Vec<str>, Error> {
//     let br = BufReader::new(io);
//     br.lines()
//         .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
//         .collect()
// }

fn read(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
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
    let mut callibration: Vec<i64> = vec![];

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
            callibration.push(cal_num);
            sum += &cal_num;
        }
        else if first.is_numeric() {
            let cal_str: String = format!("{}{}",first,first);
            let cal_num: i64 = cal_str.parse::<i64>().unwrap();
            callibration.push(cal_num);
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
