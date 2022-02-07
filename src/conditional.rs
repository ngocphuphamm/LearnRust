pub fn run(){
    let age  : u8 = 18  ; 

    let check_id : bool = true ;

    let know_person_of_age = true ; 
    // IF AND ELSE 
    if age >= 19 && check_id || know_person_of_age {
        println!("Ban co muon uong ruou ");
    }
    else if age < 21 && check_id || know_person_of_age  {
        println!("Ban ko du tuoi uong ruoui");
    }
    else {
        println!("Toi can check id cua ban ")
    }

}