pub fn run(){

    let mut hello = String :: from ("Hello");

    // Get length 
    println!("Hello length : {}",hello.len());

    //Only 1  chararacter and only for string 
    hello.push('W');

    // Push string and only for string 
    hello.push_str(" Ngoc Phu");


    // Capacity in bytes 

    println!("Capacity {}",hello.capacity());
    println!("{}",hello);
}