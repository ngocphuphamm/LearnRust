enum Movement { 
    //Variants 
    Up,
    Down,
    Left,
    Right
}

fn move_avartar(m : Movement) {
    //Perform action depending on info 
    match m {
        Movement ::Up => println!("AVARTAR MOVING UP "),
        Movement :: Down => println!("AVARTAR MOVING DOWN "),
        Movement :: Left => println!("AVARTAR MOVING LEFT "),
        Movement :: Right => println!("AVARTAR MOVING RIGHT "),

    }
}
pub fn run(){
    let avartar1 = Movement:: Up; 
    let avartar2 = Movement:: Down;
    let avartar3 = Movement:: Left; 
    let avartar4 = Movement:: Right;

    move_avartar(avartar1);
    move_avartar(avartar2);
    move_avartar(avartar3);
    
}