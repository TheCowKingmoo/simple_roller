use std::env;
use rand::Rng;

fn main() {
    // checks that a command was given
    if env::args().len() == 1  {
        println!("no input was given!");
        return;
    } 

    let mut rng = rand::thread_rng();
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
        roll_tuples.push(parse_input(input));
    }

    for tuple in roll_tuples  {
        println!("rolling for D{}", tuple.1);
        for i in 0..tuple.0  {
            println!("{}", rng.gen_range(1, tuple.1 + 1));
        }
    }

}


fn parse_input(input: String) -> (u32, u32)  {
    let mut number_of_dice = Vec::new();
    let mut dice_type = Vec::new();
    
    const ASCIIFIX: u32 = 48;

    let mut dice_num_flag = false;
    let mut d_char_flag = false;

    for character in input.chars()  {
      //convert char to u32
      let converted: u32 = u32::from(character) - ASCIIFIX;

      //check if char is an integer - no need to check lower as we are using u32
      if converted > 9 {
          //check if we got a number first
          if number_of_dice.len() < 1  {
              println!("error - no numbers before char input");
              return (0, 0);

         // triggers if we get non number input after we get dice type
          }  else if dice_type.len() > 1  {
            println!("error - too much input");
            return (0, 0);
          }

          //check if this is our single D character
          if d_char_flag == false && (character == 'd' || character == 'D') {
              d_char_flag = true;
              dice_num_flag = true;
          }  else  {
              println!("error - bad input");
          }

      //this is still the first set of numbers so we add to numDice
      }  else if dice_num_flag == false {
        number_of_dice.push(converted);
    //this is the second set of numbers so we add to dice_type
      }  else  {
          dice_type.push(converted);
      }

    }  //end for loop
    let return_values = (convert_vector_of_u32_to_single_u32(number_of_dice.as_mut_slice()), convert_vector_of_u32_to_single_u32(dice_type.as_mut_slice()));
    return return_values;

}  //end parse_input



fn convert_vector_of_u32_to_single_u32(input_vector: &mut [u32]) -> u32  {
  let mut return_value: u32 = 0;
  let mut i = 1;

  let l: u32 =  input_vector.len() as u32;

  for input in input_vector  {
    return_value += *input * 10u32.pow(l - i);
    i += 1;
  }
  return return_value;
}
