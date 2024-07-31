mod file_handler;
use std::io;

fn main() {
    println!("Choose an option:");
    println!("1. Command-line interface");
    println!("2. GUI");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => run_cli(),
        _ => println!("Invalid choice"),
    }
}

fn run_cli() {
    let mut title = String::new();
    let mut description = String::new();
    let mut completed = String::new();
    let mut data = file_handler::load_todo_list().expect("Failed to load the todo list");

    println!("Enter the title: ");
    io::stdin().read_line(&mut title).expect("Something went wrong");
    println!("Enter the description: ");
    io::stdin().read_line(&mut description).expect("Something went wrong");
    println!("Enter the completed (Yes or No): ");
    io::stdin().read_line(&mut completed).expect("Something went wrong");

    let task_completed = completed.trim().to_uppercase() == "YES";
    match file_handler::Task::new(title.trim().to_string(), description.trim().to_string(), task_completed) {
        Ok(new_data) => {
            data.insert(new_data);
        }
        Err(_) => panic!("There is an error creating the task"),
    }

    file_handler::save_todo_list(&data).expect("Failed to save the todo list");
    data.print_data();
    title.clear();

    println!("Enter the title: ");
    io::stdin().read_line(&mut title).expect("Something went wrong");
    data.remove_task(&title);
    data.print_data();
    title.clear();

}
