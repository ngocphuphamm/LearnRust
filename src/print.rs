
pub fn run(){
    // Print to console
    println!("Hello my name phu ");

    // Basic Formatting; 
    println!("Number : {}",1);
    println!("{} is from {}","Brad","Mass");


    // Positional Agruments 
    println!("{1} is from {0} and like to {1}","Brad","Mass");

    //Named Agruments 
    println!("{name} is from {ac} and like to",
    name = "Ngoc phu ",
    ac = "Dang danh code"
    );

    //Place holder traits 
    println!("Binary : {:b} ,HEX : {:x} , Octal : {:o}",10,10,10);

    //Place holder for debu trait 
    println!("{:?}",(12,true,"hello"));
    

    // Basic Math 
    println!("{:?}",(12,true,"hello"));
}