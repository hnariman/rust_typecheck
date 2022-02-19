use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    return type_name::<T>();
}

fn main() {
    let x = 1;
    let y = 999.9;

    let message = "THIS is a message".to_string();
    println!("{}", type_of(x));
    println!("{}", type_of(y));
    println!("{}", type_of(message));
}
