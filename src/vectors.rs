
use std :: mem ; 
pub fn run() { 
    let mut number  : Vec<i32>= vec![11,2,4,5,1];
    

    // Re - assign value 
    number[2] = 20 ; 
    println!("{:?}",number);


    // Add on to vector ; 
    number.push(5);
    number.push(6);

    // pop off last value
    number.pop();

    // get single val 
    println!("Single value : {}", number[0]);

    // Get array length 
    println!("Vector Length : {}", number.len());

    // Arrays are stack allocated  (know byte )
    println!("Vector occupies {} byte ", mem::size_of_val(&number));

    // Get slice 
    let slice  : &[i32] = &number[0..2]; 
    println!("Slice : {:?}",slice );


    // loop through vector values 
    for item in number.iter() {
        println!("Number : {}",item);
    }

    // loop & mutate values 
    for x in number.iter_mut(){
            *x *=2 ; 
    }
    println!("Number vec : {:?}",number);
}