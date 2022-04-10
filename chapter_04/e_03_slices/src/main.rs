fn get_first_name(name: &str) -> &str {
    // let f_name = &name[0..=4];  

    let bytes = name.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &name[0..i];
        }
    }
    // [..5]   = [0, 1, 2, 3, 4]
    // [0..6]  = [0, 1, 2, 3, 4, 5]
    // [0..=5] = [0, 1, 2, 3, 4, 5]

   &name[..]

}

// fn get_second_word(name: &String)->&str {

//     let bytes = name.as_bytes();
//     let first_word = String::new();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' '{

//             continue;
//         }
//     }

// }
fn main() {

    // slices let you reference a contigious piece of a collection rather than the whole collection

    let full_name = String::from("Thisis a good book");
    println!("{}", get_first_name(&full_name));

    /*
     * 
     * &str has two fields [ptr and len]
     * String has three fields [ptr, len, capacity]
     */

     // string slice range must contain a valid UTF8 encoded string, if you slice in between it wil cause error

    // string literals are slices and they are immutable. Th






}

