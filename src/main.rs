fn main() {

 // CONST 
    let mut x =  10 ; // mut = mutable có thể thay đỏi được 
    println!("x = {}",x);
    x=20; 
    println!("x = {}",x);
// KHAI BÁO BIẾN HẰNG SỐ
// tên biến cách nhau dấu _ và ghi hoa 
// U32 : INTERGER 32 BIT ; 

// bất biến  : variable 
//Mặc định  của biến trong rust là bất biến 
    const HANG_SO: u32 = 1000000000000;
    //
    println!("HANG_SO = {}",HANG_SO);




  // SHADOWING
  // Giai thich khac kieu du lieu cung ten van ra value nhu thuong 
  let x: i32 =10 ; 
  println!("x= {}",x);
  let  x : &str = "ten";
  println!("x= {}",x);

  // Khac global code van ra  khacd nhau  mac du cung ten 
  let outer : i32 = 10 ;
  {
        let inner : i32 = 200; 
        println!("inner= {}",inner);
        let outer : i32 = 300; 
        println!("outer= {}",outer);
  }


    
    
}
