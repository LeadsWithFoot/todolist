use std::io;

fn main() {

    //let mut mylist: Vec<String> = Vec::new();

    

    loop{

        let mut choice = String::new();

        println!("Please select an option: 
        1) List Todo List (ENTER: ls)
        2) Edit Item in List (ENTER: edit)
        3) Add Item in List (ENTER: add)
        4) Delete Item in List (ENTER: delete)
        5) To Quit (ENTER: quit)");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        println!("You inputed: {}", choice);

        //tc == trimmed choice
        let tc = choice.trim();

        if tc == "edit" {
            println!("edit list")
        } else if tc == "delete" {
            println!("delete item from list")
        } else if tc == "add" {
            let new_task = adding();
            println!("Task Added: {}", new_task);
        } else if tc == "quit"{
            println!("You are exiting the program!");
            break;
        } else{
            println!("invalid entry: {}", choice);

        }
    }

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