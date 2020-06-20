
use rand::Rng;

pub fn print_and_roll(roll_tuples: &mut [(u32, u32)])  {
    let mut rng = rand::thread_rng();
    for tuple in roll_tuples  {
        println!("rolling for D{}", tuple.1);
        for _i in 0..tuple.0  {
            println!("{}", rng.gen_range(1, tuple.1 + 1));
        }
    }
}