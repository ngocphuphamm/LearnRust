mod var ;
mod types;
mod string ; 
mod  tuple ; 
mod array ; 
fn main() {
      array :: run()
//  // CONST 
//     let mut x =  10 ; // mut = mutable có thể thay đỏi được 
//     println!("x = {}",x);
//     x=20; 
//     println!("x = {}",x);
// // KHAI BÁO BIẾN HẰNG SỐ
// // tên biến cách nhau dấu _ và ghi hoa 
// // U32 : INTERGER 32 BIT ; 

// // bất biến  : variable 
// //Mặc định  của biến trong rust là bất biến 
//     const HANG_SO: u32 = 1000000000000;
//     //
//     println!("HANG_SO = {}",HANG_SO);




//   // SHADOWING
//   // Giai thich khac kieu du lieu cung ten van ra value nhu thuong 
//   let x: i32 =10 ; 
//   println!("x= {}",x);
//   let  x : &str = "ten";
//   println!("x= {}",x);

//   // Khac global code van ra  khacd nhau  mac du cung ten 
//   let outer : i32 = 10 ;
//   {
//         let inner : i32 = 200; 
//         println!("inner= {}",inner);
//         let outer : i32 = 300; 
//         println!("outer= {}",outer);
//   }


//     // DATA TYPES 

//     // Integer ,String ,Boolean,Float
   
//     // phải gán biến cho nó 


//     // ScalarType : Biến đơn : khởi tạo 1 biến duy nhất 
//     // let x  :  isobit|| usobit = 100 

//     // Compoung Data :Tổng hợp  nhiều biến , nhiều kiểu dữ liệu trong 1 biến ,array 
//             //Tuple
//                 // Một dạng dữ liệu kết hợp với nhiều kiểu dữ liệu trong tup;e ( nhiều kiểu dữ liệu trong cái array ); 

//                 let tup : (&str,i32) = ("hello",100_000);
//                 // In ra tuple 
//                 println!("{:?}",tup);
//                 //In ra từng giá trị 
//                 // cách 1 
//                     let (_string : &str , _integer : i32) = tup;
//                     // truy cập tới vị trí của tuple 
//                     let _integer = i32:: &str = tup.1;
//                     println!("{}",_integer);
                                                                    
    
//     // ARRAY 
//   // LÀ MỘT DANH SÁCH KÍCH THƯỚC CỐ ĐỊNH VÀ CÁC KIỂU DỮ LIỆU TRONG ĐÓ ĐỒNG NHẤT 
//   let number : [i32;3] = [100,200,300];
//   let get_number  : i32 = number[1];
//   println!("{}",get_number);

//   // cách 2  
//   let _hashing:  [i32; 32] = [0; 32];
//   println!("hashing = {:?}",_hashing);
//   // dung vong lap duyet mang 
//   let integer : i32 = 0 ;
//   for integer in _hashing.iter() {
//       print!("{}",integer)
//   }
}
