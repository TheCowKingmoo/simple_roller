use std::env;


fn main() {

    println!("Hello, world!");

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


    for input in inputs  {
        println!("{}", input);
        let returnTuple = parseInput(input);
        println!("numdice = {}", returnTuple.0);
        println!("dicetype = {}", returnTuple.1);

    }

}


fn parseInput(input: String) -> (u32, u32)  {
    let mut numberOfDice = Vec::new();
    let mut diceType = Vec::new();
    
    const ASCIIFIX: u32 = 48;

    let mut diceNumFlag = false;
    let mut diceTypeFlag = false;
    let mut dCharFlag = false;

    for character in input.chars()  {
      //convert char to u32
      let converted: u32 = u32::from(character) - ASCIIFIX;

      //check if char is an integer
      if( converted > 9 || converted < 0)  {
          println!("finished with ints");

          //check if we got a number first
          if(numberOfDice.len() < 1)  {
              println!("no numbers before char input");
              return (0, 0);
          }

          //check if this is our single D character
          if( dCharFlag == false && (character == 'd' || character == 'D') )  {
              dCharFlag = true;
              diceNumFlag = true;
              println!("D is there");
          }  else  {
              println!("bad input");
          }

      //this is still the first set of numbers so we add to numDice
      }  else if(diceNumFlag == false) {
        numberOfDice.push(converted);
    //this is the second set of numbers so we add to diceType
      }  else  {
          diceType.push(converted);
      }

      println!("{}", converted);
    }  //end for loop
    let returnValues = (convertU32VecToU32(numberOfDice.as_mut_slice()), convertU32VecToU32(diceType.as_mut_slice()));
    return returnValues;

}  //end parseInput



fn convertU32VecToU32(inputVector: &mut [u32]) -> u32  {
  let mut returnValue: u32 = 0;
  let mut i = 1;

  let l: u32 =  inputVector.len() as u32;

  for input in inputVector  {
    returnValue += *input * 10u32.pow(l - i);
    i += 1;
  }
  return returnValue;
}
