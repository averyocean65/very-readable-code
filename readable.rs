/* "Very Readable Code" - a repo by averyocean65
* This repository serves as a way to demonstrate how NOT to write code
* If the code doesn't execute correctly, then I don't wanna fix it cuz I just wanna write code that I think people would find funny
*/

use std::env;

const a: i32 = 3;

// so this is like the main function that gets called when the program starts and so we put the code we wanna execute in here and if we don't do that then our code won´t run yk?
fn main() {
    let my_variable: Vec<String> = env::args().collect();

    // so like if the length of my_variable is less than whatever the fuck i did there then we panic and give an error message telling the user the error that we decided to put in there, the argument of the panic macro is a format string and the relevant arguments, similar to the print! and println! macros in rust and the printf function in C/C++
    if my_variable.len() < a.try_into().unwrap() {
        // this is the panicing i described in my very short comment earlier
        panic!("yea so like you fucked up");
    }

    // so like now we do cool stuff
    let argument: &str = &*(&my_variable[0]);
    let argument_two: &str = &*(&my_variable[1]);

    let back = make_into_right_thing(argument);
    let yeaa = make_into_right_thing(argument_two);

    let mut very_unimportant = back & yeaa;
    let mut amen_break = back ^ yeaa;

    while very_unimportant != 0 {
        let b = very_unimportant << 1;
        very_unimportant = amen_break & b;
        amen_break ^= b;
    }

    // yay :)
    println!("we did a {} out of {} and {}", amen_break, back, yeaa);
}

// this prolly summons big, muscular, oiled up and really fuckin' smelly men
fn make_into_right_thing(abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890: &str) -> i32 {
    abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890.parse::<i32>().unwrap()
}