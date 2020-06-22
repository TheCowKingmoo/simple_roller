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
    let mut i = 0;
    let mut input_string = String::new();

    for argument in env::args() {
        if i == 0  {
            _path = &argument;
            input_string.push_str("~roll");
        }  else  {
            input_string.push_str(" ");
            input_string.push_str(&argument);
        }
        i += 1;
    }

    println!("{}", input_string);
    // grab all inputs for errors before actually rolling
    let tuple_return = parse::parse_roll_message(input_string);

    let num_roll = tuple_return.0;
    let dice_type = tuple_return.1;
    let add_on = tuple_return.2;
    let args = tuple_return.3;
    let err_string = tuple_return.4;

    let mut print_string: String = String::new();
    let mut a_flag = false;

    if err_string != ""  {
        print_string = err_string;
    }  else  {
        for character in args  {
            if character == 'a'  {
                a_flag = true;
            }
        }

        if a_flag == true  {
            println!("avg");
            print_string = roll::avg_roller(num_roll, dice_type, add_on);
        }  else  {
          print_string = roll::print_all_rolls(num_roll, dice_type, add_on);
        }
        
    }

    println!("{}", print_string);


}
