// primitive_types1.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)

enum TimeOfDay {
    Morning,
    Afternoon,
    Evening,
}

fn main() {
    let period_of_day = TimeOfDay::Morning;

    match period_of_day {
        TimeOfDay::Morning => println!("Good morning!"),
        TimeOfDay::Afternoon => println!("Good afternoon!"),
        TimeOfDay::Evening => println!("Good evening!"),
    }
}
