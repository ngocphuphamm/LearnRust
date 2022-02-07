// struct  Color {
//     red : u8 , 
//      green : u8 ,
//       blue : u8 
// }

// Tuple struct
// struct Color(u8,u8,u8);


struct Person {
    first_name : String, 
    last_name : String,
}
 
impl Person {
    // Constructor person
    fn new (first : &str , last : &str) -> Person {
        Person{
            first_name : first.to_string(),
            last_name : last.to_string(),
        }
    }
    
    // method
    // // get full name 
    fn fullname (&self) -> String {
        // chuyen63 thanh string
       format!("{} {} ", self.first_name,self.last_name)
    }


    //set last name 
    fn set_last_name (&mut self,last :&str){
        self.last_name = last.to_string();
    }

    // name tuple
    fn to_tuple (self) -> (String, String) {
        (self.first_name, self.last_name)
    }
    
}


pub fn run(){
    // let mut c = Color{
    //     red : 255 , 
    //     green : 0 ,
    //     blue : 0 
    // };

    // c.red= 200; 

    // println!("{},{},{}",c.red,c.green,c.blue);


    // Tupple struct
    // let mut c =  Color(255,0,0);

    // println!("{},{},{}",c.0,c.1,c.2);


    let mut p = Person :: new("Ngoc","PhU");
    println!("{}",p.fullname());


    p.set_last_name("helloworld");
    println!("PERSON{}",p.fullname());
    
    println!("Person tuple {:?}",p.to_tuple());

    
}