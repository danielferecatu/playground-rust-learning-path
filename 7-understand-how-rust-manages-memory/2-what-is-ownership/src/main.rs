fn main() {
    // `mascot` is not valid and cannot be used here, because it's not yet declared.
    {
        let mascot = String::from("ferris");   // `mascot` is valid from this point forward.
        // do stuff with `mascot`.
    }
    // this scope is now over, so `mascot` is no longer valid and cannot be used.


    {
        let mascot = String::from("ferris");
    }
    // err: not found in this scope
    // println!("{}", mascot);


    {
        let mascot = String::from("ferris");
        // mascot dropped here. The string data memory will be freed here.
    }


    {
        let mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris.
        let ferris = mascot;
        // ferris dropped here. The string data memory will be freed here.
    }


    {
        let mascot = String::from("ferris");
        let ferris = mascot;
        // err: value borrowed here after move
        // println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
    }


    // Ownership in functions

    {
        fn process(input: String) {}

        fn caller() {
            let s = String::from("Hello, world!");
            process(s); // Ownership of the string in `s` moved into `process`
            // err: value used here after move
            // process(s); // Error! ownership already moved.
        }
    }

    {
        fn process(input: u32) {}

        fn caller() {
            let n = 1u32;
            process(n); // Ownership of the number in `n` copied into `process`
            process(n); // `n` can be used again because it wasn't moved, it was copied.
        }
    }

    {
        fn process(s: String) {}

        let s = String::from("Hello, world!");
        process(s.clone()); // Passing another value, cloned from `s`.
        process(s); // s was never moved and so it can still be used.
    }
}
