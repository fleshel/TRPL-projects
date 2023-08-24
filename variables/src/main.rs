// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {

    let test: i32 = 789;

    println!("test = {test}");

    other_function(test);
}

fn other_function(x: i32) {

    println!("foo bar baz qux, double x={}", double_function(x));
}

fn double_function(x: i32) -> i32 {
    let double_x = {
        // expressions don't end in semicolons!
        x * 2 
    };

    // implicit return without semicolon
    double_x
}
