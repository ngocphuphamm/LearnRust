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

    // Check is empty 
    println!("Test is empty : {} ",hello.is_empty());

    //Contains 
    println!("Contains  : {}",hello.contains("world"));

    // Replace 
    println!("Replace : {}",hello.replace("Phu","giao"));

    println!("White space");
    // Loop through string by whitespace 
    for item in hello.split_whitespace(){
        
        println!("{}",item);
    }

    // Create string with capacity 
    let mut  s = String:: with_capacity(10);
    s.push('s');
    s.push('q');
    
    // Assertion testing 
    assert_eq!(s.len(), s.len());
    println!("{}",s);


}