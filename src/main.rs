use std::io;

#[derive(Debug)]
struct Input {
    a: u64,
    b: u64,
    c: u64
}

trait Congruent {

    fn is_congruent(&self) -> bool;
}

impl Congruent for Input  {

        fn is_congruent(&self) -> bool {

            if (self.a - self.b) % self.c == 0 {

                true
            }
            else {
                false
            }
            }
}


fn main() {

    let mut x = String::new();
    println!("Please enter the first number:");
    io::stdin().read_line(&mut x).expect("Number not entered");
    let x: u64 = x.trim().parse().expect("Please type a number");

    let mut y = String::new();
    println!("Please enter the second number:");
    io::stdin().read_line(&mut y).expect("Number not entered");
    let y: u64 = y.trim().parse().expect("Please type a number");

    let mut z = String::new();
    println!("Please enter the modulo number:");
    io::stdin().read_line(&mut z).expect("Number not entered");
    let z: u64 = z.trim().parse().expect("Please type a number");


   let input: Input = Input {
              a: x,
              b: y,
              c: z
   };

    println!("Is {:?} modulo congruent {:?}", input, input.is_congruent());
}
