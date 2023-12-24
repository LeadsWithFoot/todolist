use std::io;

fn main() {

    //let mut mylist: Vec<String> = Vec::new();

    

    loop{

        
        //let mut action = String::new();
        let action = menu();

        if action == "edit" {
            println!("edit list")
        } else if action == "delete" {
            println!("delete item from list")
        } else if action == "add" {
            let new_task = adding();
            println!("Task Added: {}", new_task);
        } else if action == "quit"{
            println!("You are exiting the program!");
            break;
        } else if action == "ls"{
            println!("Listing todos");
        } else{
            println!("invalid entry: {}", action);

        }
    }

}

fn menu() -> String {
    
        let mut input = String::new();

        //testing, this is in menu_function, after i staged it

        println!("Please select an option: 
        1) List Todo List (ENTER: ls)
        2) Edit Item in List (ENTER: edit)
        3) Add Item in List (ENTER: add)
        4) Delete Item in List (ENTER: delete)
        5) To Quit (ENTER: quit)");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You inputed: {}", input);
        input.trim().to_string()
}

fn adding() -> String {
    println!("What item would you like to add? :");

    let mut task_name = String::new();

    io::stdin()
        .read_line(&mut task_name)
        .expect("failed to get name of added item");
    
    let trimmed_name = task_name.trim();
    return trimmed_name.to_string();

}