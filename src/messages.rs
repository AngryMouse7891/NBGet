use colored::Colorize;

pub fn info_msg(msg:&str){
    println!("{} {}","==>".green(),msg);
}

pub fn info_msg2(msg:&str){
    println!("{} {}","  ->".blue(),msg);
}

pub fn err_msg(msg:&str){
    println!("{} {} {}","==>".red(),"错误: ".red(),msg);
}

pub fn warn_msg(msg:&str){
    println!("{} {} {}","==>".red(),"警告: ".yellow(),msg);
}
