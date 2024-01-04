use std::io;


fn main() {

    // let new_task = get_user_input();
    // println!("{:?}", new_task);

}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn add_new_task(&self){
        println!("{:?}", self.tasks)
    }
}

fn get_user_input() -> Task {
    println!("Welcome to CLI Todo App");
    let mut task_title = String::new();
    println!("Enter your task title: ");
    io::stdin().read_line(&mut task_title).unwrap();
    task_title = task_title.trim().to_string();

    println!("Enter your task description: ");
    let mut task_desc = String::new();
    io::stdin().read_line(&mut task_desc).unwrap();
    task_desc = task_desc.trim().to_string();

    return Task {
        id: 1,
        title: task_title,
        description: task_desc,
        completed: false,
    };
}


