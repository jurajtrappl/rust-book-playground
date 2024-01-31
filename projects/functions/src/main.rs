fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 1;
        x + 1
    };

    println!("{y}");
}

fn another_function(x: i32) {
    println!("The value of x: {x}");
}