// My Original Solution

//fn to_hex(n: i32) -> String {
//    let uf_hex = format!("{:x}", n.clamp(0, 255));
//    if uf_hex.len() == 1 {
//        (String::from("0") + uf_hex.as_str()).to_uppercase()
//    } else {
//        uf_hex.to_uppercase()
//    }
//}
//
//fn rgb(r: i32, g: i32, b: i32) -> String {
//    format!("{}{}{}", to_hex(r), to_hex(g), to_hex(b))
//}

// Updated Solution
fn to_hex(n: i32) -> String {
    format!("{:02X}", n.clamp(0, 255))
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    format!("{}{}{}", to_hex(r), to_hex(g), to_hex(b))
}

// Can use uppercase X and get rid of .to_uppercase
//
// Can add 02 infront of X which will remove the need to concat a zero
