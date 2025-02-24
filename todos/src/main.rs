mod todo;

use std::io;
use todo::ToDoList;

fn main() {
    let mut todo_list = ToDoList::new();
    
    loop {
        println!("\n📌 To-Do List Menu:");
        println!("1️⃣ Add Task");
        println!("2️⃣ List Tasks");
        println!("3️⃣ Mark Task as Completed");
        println!("4️⃣ Delete Task");
        println!("5️⃣ Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("📝 Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                todo_list.add_task(description.trim().to_string());
            }
            "2" => {
                println!("\n📋 Your Tasks:");
                todo_list.list_tasks();
            }
            "3" => {
                println!("✅ Enter task ID to mark as completed:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Failed to read input");
                if let Ok(id) = task_id.trim().parse::<usize>() {
                    todo_list.mark_task_completed(id);
                } else {
                    println!("❌ Invalid ID!");
                }
            }
            "4" => {
                println!("🗑️ Enter task ID to delete:");
                let mut task_id = String::new();
                io::stdin().read_line(&mut task_id).expect("Failed to read input");
                if let Ok(id) = task_id.trim().parse::<usize>() {
                    todo_list.delete_task(id);
                } else {
                    println!("❌ Invalid ID!");
                }
            }
            "5" => {
                println!("👋 Exiting. Have a productive day!");
                break;
            }
            _ => println!("❌ Invalid choice! Please enter a number between 1-5."),
        }
    }
}
