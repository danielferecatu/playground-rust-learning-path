fn main() {
    // Infinite loop
    // loop {
    //     println!("We loop forever!");
    // }


    loop {
        // Keep printing, printing, printing...
        println!("WE LOOP FOREVER!");

        // On the other hand, maybe we should stop!
        break;
    }


    let mut counter = 1;
    // stop_loop is set when loop stops
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            // Stop loop, return counter value
            break counter;
        }
    };

    // Loop should break when counter = 128
    println!("Break the loop at counter = {}.", stop_loop);

    
    counter = 0;
    while counter < 5 {
        println!("We loop a while...");
        counter += 1;
    }


    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
    }


    for number in 0..5 {
        println!("{}", number * 2);
    }
}
