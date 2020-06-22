

const ARG_CHAR: char = '-';
const ARG_AVERAGE_CHAR: char = 'a';
const DICE_CHAR_LOWER: char = 'd';
const DICE_CHAR_UPPER: char = 'D';
const EMPTY_STRING: &str = "";
const E_TOO_SMALL_INPUT: &str = "input is too small";
const E_MORE_THAN_ONE_ARG_CHAR: &str = "more than one arg character was detected";
const E_NON_INT_BEFORE_D: &str = "got a non-integer before D";
const E_NON_INT_AFTER_D: &str = "got non-integer after D";
const E_DEFAULT: &str = "default error catcher";
const E_MULTIPLE_SAME_ARG: &str = "Error - more than one of the same arg detected";
const E_NO_MATCH_ARG: &str = "Error - no matching arguments";


pub fn parse_roll_message(message_string: String) -> (u32, u32, Vec<char>, String)  {

    const ASCIIFIX: u32 = 48;

    let mut arg_char_vector = Vec::new();
    let mut arg_str_return: String = String::new();

    if message_string.len() < 9  { //prefix + roll + whitespace + 1D1
      arg_str_return.push_str(E_TOO_SMALL_INPUT);
      return (0, 0, arg_char_vector, arg_str_return);
    }

    let mut number_of_dice = Vec::new();
    let mut dice_type = Vec::new();
    
    let mut dice_num_flag = false;
    let mut d_char_flag = false;
    let mut operator_flag = false;

    //println!("{}", message_string);

    let split = message_string.split_whitespace();

    let mut i = 0;
    for chunk_string in split  {

      println!("{}", chunk_string);

      // skip the first loop - always equal to ~roll
      if i == 0  {
        i += 1;
        continue;
      }

      // skip the first split string as we are looking at ~roll
      let mut j = 0;
      // now we break down the string into characters
      for character in chunk_string.chars()  {
        // check if we are looking at args
        if j == 0 && character == ARG_CHAR  {
          if operator_flag == true  {
            // we have already tried to pick up arguments
            arg_str_return.push_str(E_MORE_THAN_ONE_ARG_CHAR);
            return (0, 0, arg_char_vector, arg_str_return);

            }  else  {
              operator_flag = true;
              let arg_return_tuple = break_up_arg(chunk_string);
              arg_char_vector = arg_return_tuple.0;
              arg_str_return = arg_return_tuple.1;
              if arg_str_return != EMPTY_STRING  {
                return (0, 0, arg_char_vector, arg_str_return);
              }
              break;
            }
          }

          // this will move the char values down so that 1 will be equal to 1
          let converted: u32 = u32::from(character) - ASCIIFIX;

          //check if char is an integer - no need to check lower as we are using u32
          if converted > 9 {
              //check if we got a number first
              if number_of_dice.len() < 1  {
                arg_str_return.push_str(E_NON_INT_BEFORE_D);
                return (0, 0, arg_char_vector, arg_str_return);

    
             // triggers if we get non number input after we get dice type
              }  else if dice_type.len() > 1  {
                arg_str_return.push_str(E_NON_INT_AFTER_D);
                return (0, 0, arg_char_vector, arg_str_return);

              }
    
              //check if this is our single D character
              if d_char_flag == false && (character == DICE_CHAR_LOWER || character == DICE_CHAR_UPPER) {
                  d_char_flag = true;
                  dice_num_flag = true;
              }  else  {
                arg_str_return.push_str(E_DEFAULT);
                return (0, 0, arg_char_vector, arg_str_return);

              }
    
          //this is still the first set of numbers so we add to numDice
          }  else if dice_num_flag == false {
            number_of_dice.push(converted);
        //this is the second set of numbers so we add to dice_type
          }  else  {
              dice_type.push(converted);
          }  
          
          j += 1;

        } 

      i += 1;

    }
    return (convert_vector_of_u32_to_single_u32(number_of_dice.as_mut_slice()), convert_vector_of_u32_to_single_u32(dice_type.as_mut_slice()), arg_char_vector, arg_str_return);
}

fn break_up_arg(arg_string: &str) -> (Vec<char>, String)  {
  let mut i = 0;
  let mut return_vector = Vec::new();
  let mut a_flag = false;
  let mut return_str = EMPTY_STRING;
  let mut return_string = String::new();

  for character in arg_string.chars()  {
    if i == 0  {
      i += 1;
      continue;
    }

    let lower_character = character.to_ascii_lowercase();

    match lower_character  {
      // a = avg
      ARG_AVERAGE_CHAR => if a_flag == false  {
        a_flag = true;
        return_vector.push(character);
      }  else  {
        return_str = E_MULTIPLE_SAME_ARG;
        break;
      },
      // default catch all
      _ => return_str = E_NO_MATCH_ARG
    }
    
    i+=1;
  }
  return_string.push_str(return_str);
  return (return_vector, return_string);
}


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
  