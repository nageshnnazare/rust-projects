pub fn run() {
    let age = 18;
    let check_id: bool = false;
    let knows_person: bool = true;

    if age >= 21 && check_id || knows_person {
        println!("you are overage !");
    } else if age < 21 && check_id {
        println!("you are underage !");
    } else {
        println!("you are ok");
    }

    // shorthand if

    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age ? {}", is_of_age); 
}
