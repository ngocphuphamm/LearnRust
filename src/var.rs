pub fn run(){
    let name  = "Brad"; 
    //dung mutable  de thay doi 
    let mut age = 37; 
    println!("My name is {}  and {}",name,age);
    age = 38;
    println!("My name is {} and {}",name,age);

    // Define costant 
    const ID : i32 = 0001 ; 
    println!("{}",ID);

    // Assign multiple vars; 
    let (my_name,my_age) = ("brad",37);
    println!("{} is {}",my_name,my_age);
}