enum Person{
    Skinny,
    Fat,
    Height(i32),
    Weight(i32),
    Info{ name: String, height: i32},
}

fn inspect(p: Person) {
    match p {
        Person::Skinny    => println!("Is skinnny!"),
        Person::Fat       => println!("Is fat!"),
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        Person::Info{ name, height} => {
            println!("{} is {} tall!", name, height);
        }
    }
}

fn main(){
    let person = Person::Height(18);
    let danny = Person::Weight(10);
    let dave = Person::Info{ name:"Dave".to_owned(), height: 72};
    let john = Person::Fat;
    let larry = Person::Skinny;

    inspect(person);
    inspect(danny);
    inspect(dave);
    inspect(john);
    inspect(larry);
}
