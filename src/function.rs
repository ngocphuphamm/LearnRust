pub fn run(){
    greeting("ngocphu","hello");

    //Blind function values to variables
    let get_sum = add(1,2);
    println!("{}",get_sum);

    // Closure 
    let n3 : i32  = 10 ; 
    let add_nums = |n1: i32 , n2 :i32| n1+n2+n3 ;
    println!("{}",add_nums(3,3));
}

fn greeting(greeting: &str, name : &str ){
    println!("{} {}",greeting,name);

}

fn add (n1: i32 , n2 : i32 )-> i32{
    n1 + n2 
}
