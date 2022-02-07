
use std :: mem ; 
pub fn run() { 
    let mut number : [i32;5] = [11,2,4,5,1];
    

    // Re - assign value 
    number[2] = 20 ; 
    println!("{:?}",number);

    // get single val 
    println!("Single value : {}", number[0]);

    // Get array length 
    println!("Array Length : {}", number.len());

    // Arrays are stack allocated  (know byte )
    println!("Vector occupies {} byte ", mem::size_of_val(&number));

    // Get slice 
    let slice  : &[i32] = &number[0..2]; 
    println!("Slice : {:?}",slice );
}