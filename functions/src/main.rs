// fn main() {
//     another_function(-5);
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// multiple

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// express n statements

fn main() {
    let y = {
        let x = 3;
        x + 1 //dont add semicolon
    };

    println!("The value of y is: {y}");
}
