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

    let mut print_string: String = String::new();
    let mut a_flag = false;

    println!("{}", tuple_return.0);
    println!("{}", tuple_return.1);
    println!("{}", tuple_return.2[0]);
    println!("{}", tuple_return.3);
    if tuple_return.3 != ""  {
        print_string = tuple_return.3;
    }  else  {
        for character in tuple_return.2  {
            if character == 'a'  {
                a_flag = true;
            }
        }

        if a_flag == true  {
            println!("avg");
            print_string = roll::avg_roller(tuple_return.0, tuple_return.1);
        }  else  {
          print_string = roll::print_all_rolls(tuple_return.0, tuple_return.1);
        }
        
    }

    println!("{}", print_string);


}
