use std::io;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
struct Task{
    msg: String,
    priority: u32,
}

fn main() {

    let mut file = match File::open("mylist.json") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("unable to open file, error = {}", err);
            return;
        }
    };

    let mut contents = String::new();
    let mut tasks : Vec<Task> = Vec::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            println!("File contents: \n{}", contents);
            tasks = serde_json::from_str(&contents).expect("Unable to parse JSON");
            for (index, task) in tasks.iter().enumerate() {
                println!("Index: {}, Task: {:?}", index, task);
            }
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            return;
        }
    }
    

    loop{
        let action = menu();

        if action == "edit" {
            edit(&mut tasks);
        } else if action == "delete" {
            delete(&mut tasks);
        } else if action == "add" {
            adding(&mut tasks);
        } else if action == "quit"{
            println!("You are exiting the program!");
            break;
        } else if action == "ls"{
            ls(&mut tasks);
        } else{
            println!("invalid entry: {} \n Try again!", action);
        }
    }
}

fn edit(vector: &mut Vec<Task>) {
    println!("Which task would you like to edit?");
    println!("Did you want to edit the name or priority of the task?:\n
                Enter name to change name\n
                Enter priority to change priority: ");
}

fn ls(vector: &mut Vec<Task>) {
    for (index, task) in vector.iter().enumerate() {
        println!("Index: {}, Task: {:?}", index, task);
    }
}

fn delete(vector: &mut Vec<Task>) {
    
    //IN FUTURE ADD AUTOFILL FUNCTIONALITY

    loop{
        let mut choice = String::new();

        println!("Which task would you like to delete?");
        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read line");
        choice = choice.trim().to_string();

        let mut indextodelete = 0;
        let mut flag: bool = false;
        for (index, task) in vector.iter().enumerate() {
            println!("Index: {}, {:?}", index, task);
            println!("{:?}",task.msg);
            println!("{}", choice);
            if task.msg == choice{
                indextodelete = index;
                println!("INDEX IS: {}", index);
                flag = true;
            }
        }

        if flag == true{
            vector.remove(indextodelete);
        } else {
            println!("Task not found!");
            continue;
        
        }

   
        ls(vector);

        for (index, task) in vector.iter_mut().enumerate() {
            if index >= indextodelete{
                println!("{}", task.priority);
                println!("{}", (task.priority - 1));
                task.priority -= 1;
            }
        }
        ls(vector);
        break;
    }
}

fn menu() -> String {
    
        let mut input = String::new();

        println!("Please select an option: 
        1) List Todo List (ENTER: ls)
        2) Edit Item in List (ENTER: edit)
        3) Add Item in List (ENTER: add)
        4) Delete Item in List (ENTER: delete)
        5) To Quit (ENTER: quit)");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("You entered: {}", input);
        input.trim().to_string()
}

fn adding(vector: &mut Vec<Task>) {
    println!("What task would you like to add? :");

    let mut task_name = String::new();

    io::stdin()
        .read_line(&mut task_name)
        .expect("failed to get name of added task");
    
    let trimmed_name = task_name.trim().to_string();

    let last_index = vector.len() - 1;
    let prinum: u32 = last_index as u32;

    if let Some(last_element) = vector.get(last_index) {
        println!("Last element: {:?}", last_element);
    } else {
        println!("Vector is empty or out of bounds");
    }

    let holder = Task {
        msg: trimmed_name,
        priority: (prinum + 2),
    };

    vector.push(holder);

    ls(vector);
}