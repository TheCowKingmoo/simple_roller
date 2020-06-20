
pub fn parse_input(input: String) -> (u32, u32)  {
    let mut number_of_dice = Vec::new();
    let mut dice_type = Vec::new();
    
    const ASCIIFIX: u32 = 48;

    let mut dice_num_flag = false;
    let mut d_char_flag = false;


    if input.len() < 3  {
        println!("str not long enough to do anything with");
        return (0, 0);
    }

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
              return (0, 0);
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

#[test]
fn test_vector_of_u32_to_single_u32() {
    let mut input = Vec::new();
    input.push(1);
    input.push(0);
    input.push(0);
    input.push(1);
    let actual = 1001;
    let return_value = convert_vector_of_u32_to_single_u32(input.as_mut_slice());
    assert_eq!(actual, return_value);
}

#[test]
fn test_parse_input_no_int() {
    let input = "test".to_string();
    let expected = (0, 0);
    let return_value = parse_input(input);
    assert_eq!(expected, return_value);
}

#[test]
fn test_parse_input_one_int() {
    let input = "1D".to_string();
    let expected = (0, 0);
    let return_value = parse_input(input);
    assert_eq!(expected, return_value);
}

#[test]
fn test_parse_input_no_input() {
    let input = "".to_string();
    let expected = (0, 0);
    let return_value = parse_input(input);
    assert_eq!(expected, return_value);
}

#[test]
fn test_parse_input_one_int_two_d() {
    let input = "1DD".to_string();
    let expected = (0, 0);
    let return_value = parse_input(input);
    assert_eq!(expected, return_value);
}


#[test]
fn test_parse_input_no_d() {
    let input = "1R20".to_string();
    let expected = (0, 0);
    let return_value = parse_input(input);
    assert_eq!(expected, return_value);
}

#[test]
fn test_parse_input_regular_input() {
    let input = "1D20".to_string();
    let expected = (1, 20);
    let return_value = parse_input(input);
    assert_eq!(expected, return_value);
}