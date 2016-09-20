#![allow(dead_code)]
#![allow(unused_variables)]

struct Person {
    name: String,
    age: i32,
    friends: [String; 2],
}

impl Person {
    pub fn new() -> Person {
        Person {
            name: "Westin".to_string(),
            age: 32,
            friends: ["Test1".to_string(), "Test2".to_string()],
        }
    }
}

fn create_user() -> Person {
    // TODO: Update to take name, age, friends as params
    Person::new()
}

fn print_user(user: Person) {
    println!("{} is {} years old", user.name, user.age);
    // TODO: List Friends
}

fn print_friends() {
    // TODO: loop over friends
    // TODO: call in print_user
}
fn update_friends() {
    // TODO: mutate friends list
}


fn main() {
    let user = create_user();
    print_user(user);
}

// TODO: (*Bonus*) if you have extra time make dot chaining work
//  let user = Person::new()
//                .name(1.0)
//                .age(2.0)
//                .friends(['Westin'])
//                .finalize();
