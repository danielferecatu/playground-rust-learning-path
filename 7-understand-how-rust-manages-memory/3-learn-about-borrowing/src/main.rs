fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn change(message: &String) {
    // err: `message` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // message.push_str("!"); // We try to add a "!" to the end of our message
}

fn change_mut(text: &mut String) {
    text.push_str(", world");
}

fn main() {
    let greeting = String::from("hello");
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`

    
    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again

    
    change(&greeting);

    let mut greeting_mut = String::from("hello");
    change_mut(&mut greeting_mut);
    print_greeting(&greeting_mut);



    {
        let mut value = String::from("hello");

        let ref1 = &mut value;
        // error[E0499]: cannot borrow `value` as mutable more than once at a time
        // let ref2 = &mut value;
        // println!("{}, {}", ref1, ref2);
    }
    

    {
        let mut value = String::from("hello");

        let ref1 = &value;
        // error[E0502]: cannot borrow `value` as mutable because it is also borrowed as immutable
        // let ref2 = &mut value;

        // println!("{}, {}", ref1, ref2);
    }
}
