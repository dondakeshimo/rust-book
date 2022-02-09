fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    another_function();
    println!("five: {}", five());

    if five() > 5 {
        println!("{} > 5", five());
    } else if five() < 5 {
        println!("{} < 5", five());
    } else {
        println!("{} = 5", five());
    }

    for num in (1..4).rev() {
        println!("{}!", num)
    }

    for num in 1..4 {
        println!("{}!", num)
    }
}

fn another_function() {
    println!("Another function.");
}

fn five () -> i32 {
    5
}
