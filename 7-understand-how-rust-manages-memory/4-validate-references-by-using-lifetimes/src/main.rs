fn main() {
    let x = 0;
    {
        let y = 42;
        // error[E0597]: `y` does not live long enough
        // x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!

    
    {
        let magic1 = String::from("abracadabra!");
        let magic2 = String::from("shazam!");

        let result = longest_word(&magic1, &magic2);
        println!("The longest magic word is {}", result);
    }

    {
        let magic1 = String::from("abracadabra!");
        let result = String::from("");
        {
            let magic2 = String::from("shazam!");
            // error[E0597]: `magic2` does not live long enough
            // result = longest_word(&magic1, &magic2);
        }
        println!("The longest magic word is {}", result);
    }

    // Annotating lifetimes in types

    {
        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);
        println!("{:?}", fox);
        println!("{:?}", dog);
    }
}

fn longest_word<'lifetime>(x: &'lifetime String, y: &'lifetime String) -> &'lifetime String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Highlight<'document>(&'document str);
