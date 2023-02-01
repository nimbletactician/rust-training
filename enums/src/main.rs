#[derive(Debug)]
// Overlapping memory
enum Stuff {
    A(i32),
    B(String),
    C(bool),
    D(S),
}
#[derive(Debug)]
struct S {
    a: i32,
    b: String,
    c: bool,
}

// impl block associates function to type
impl S {
    fn new() -> Self {
        Self {
            a: 0,
            b: "".to_owned(),
            c: false,
        }
    }
}

fn main() {
    print!("s = {:?}", S::new());
    /*
    // tuple just like struct has non overlapping memory
    let tuple = (1, 2, "hi");
    print!("tuple @0={}", tuple.0);
    print!("tuple @1={}", tuple.1);
    print!("tuple @2={}", tuple.2);
    let stuff = Stuff::A(23);
    let stuff = Stuff::C(false);

    // Literal/Direct contruction of struct
    let stuff = Stuff::D(S {
         a: 0,
         b: "Bye".to_owned(),
         c: true,
     });
    println!("stuff = {stuff:?}");
    */

    let stuff = Stuff::C(true);
    print!("stuff = {stuff:?}");
    // switch statement
    match stuff {
        Stuff::A(a) => {
            print!("A = {a}");
        }
        Stuff::B(b) => {
            print!("B = {b}");
        }
        Stuff::C(c) => {
            print!("C = {c}");
        }
        Stuff::D(_) => todo!(),
    }
}
