use std::io::stdin;
fn main(){
    let mut input = String::new();
    stdin().read_line(&mut input).expect("error");
    let mut txt = String::new();
    let mut oper = String::new();
    let mut parsed = vec![];
    for _char in input.chars(){
        for num in "1234567890".chars(){
            if _char == num{
                txt += &_char.to_string();
            }
        }
        for operator in "+-/*".chars(){
            if _char == operator{
                parsed.push(txt.trim().parse::<i32>().unwrap());
                txt = "".to_string();
                oper = _char.to_string();
            }
        }
    }
    parsed.push(txt.trim().parse::<i32>().unwrap());
    
    for i in parsed.iter(){
        print!("{},",i);
    }
    print!("\n");
    println!("{}",oper);
    match oper.as_str(){
        "+" => println!("{}",parsed[0] + parsed[1]),
        "-" => println!("{}",parsed[0] + parsed[1]),
        "*" => println!("{}",parsed[0] + parsed[1]),
        "/" => println!("{}",parsed[0] + parsed[1]),
        _ => println!("error")
    }
}