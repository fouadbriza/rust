fn main() {
    // let capacity = 2;
    // let result = check_capacity(capacity);
    // let decision = match result {
    //     Ok(result_message)=> result_message,
    //     Err(result_message)=> result_message
    // };
    // println!("for this capacity: {} the desicion is: {:?}",capacity,decision);
}



fn check_capacity(current_passangers: i32)-> Result<String,String> {
    if current_passangers<3 {
        Ok(String::from("Boarding allowed"))

    } else {
        Err(String::from("Flight is fully booked!"))
    }

} 

