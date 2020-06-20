mod parse;
mod roll;



use std::env;

fn main() {
    // checks that a command was given
    if env::args().len() == 1  {
        println!("no input was given!");
        return;
    } 

    let mut _path = "";
    let mut inputs = Vec::new();
    let mut i = 0;
    for argument in env::args() {
        if i == 0  {
            _path = &argument;
        }  else  {
            inputs.push(argument);
        }
        i += 1;
    }

    let mut roll_tuples = Vec::new();

    // grab all inputs for errors before actually rolling
    for input in inputs  {
        roll_tuples.push(parse::parse_input(input));
    }

    roll::print_and_roll(roll_tuples.as_mut_slice());

}
