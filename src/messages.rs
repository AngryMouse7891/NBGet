use colored::Colorize;

pub fn info_msg(msg:&str){
    println!("{} {}","==>".green(),msg);
}

pub fn err_msg(msg:&str){
    println!("{} {} {}","==>".red(),"错误: ".red(),msg);
}
