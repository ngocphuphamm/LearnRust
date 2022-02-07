use std::env;
pub fn run (){
    let args : Vec<String>  = env::args().collect();
    let command = args[1].clone();
    let name = "ngocphu";
    println!("Command {}",command);

    if command == "hello"
    {
        println!("Hello xin chao {}",name);
    }
    else
    {
        println!("cut");
    }
}