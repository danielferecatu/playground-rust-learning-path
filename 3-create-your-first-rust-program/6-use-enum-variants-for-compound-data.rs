fn main() {
    #[derive(Debug)]
    enum WebEvent1 {
        // An enum variant can be like a unit struct without fields or data types
        WELoad,
        // An enum variant can be like a tuple struct with data types but no named fields
        WEKeys(String, char),
        // An enum variant can be like a classic struct with named fields and their data types
        WEClick { x: i64, y: i64 }
    }

    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress2(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick2 { x: i64, y: i64 }

    // Redefine the enum variants to use the data from the new structs
    // Update the page Load variant to have the boolean type
    #[derive(Debug)]
    enum WebEvent2 { WELoad(bool), WEClick(MouseClick2), WEKeys(KeyPress2) }


    // - Simple variant: WELoad(bool)
    let we_load = WebEvent2::WELoad(true);

    // - Struct variant: WEClick(MouseClick)
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick2 { x: 100, y: 250 };

    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent2::WEClick(click);

    // - Tuple variant: WEKeys(KeyPress)
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress2(String::from("Ctrl+"), 'N');
        
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent2::WEKeys(keys);


    // Define a tuple struct
    #[derive(Debug)]
    struct KeyPress(String, char);

    // Define a classic struct
    #[derive(Debug)]
    struct MouseClick { x: i64, y: i64 }

    // Define the WebEvent enum variants to use the data from the structs
    // and a boolean type for the page Load variant
    #[derive(Debug)]
    enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);
        
    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
        
    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
        
    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);
}
