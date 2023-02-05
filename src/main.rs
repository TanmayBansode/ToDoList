use std::io;

fn add_new_task(tasks: &mut Vec<String>) {
    println!("-------------------------------------------------");
    println!(" ");
    println!(">>Enter Tasks<<");
    println!(" ");
    println!("Type and Enter to add a Task.");
    println!("(Type done when finished)");
    println!(" ");
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Error");
        input.pop();

        match input == "done" {
            true => {
                println!("-------------------------------------------------");
                println!(" ");
                println!(">>TASKS<<");
                println!(" ");
                let mut i: u32 = 1;
                for el in tasks {
                    println!("{i}.{}", el);
                    i += 1;
                }

                break;
            }

            _ => {
                tasks.push(input);
            }
        }
    }
    println!(" ");
    println!("-------------------------------------------------");
}

fn print_tasks(tasks: &Vec<String>) {
    for (i, el) in tasks.iter().enumerate() {
        println!("{}. {}", i + 1, el);
    }
}

fn show_tasks(tasks: &Vec<String>) {
    println!("-------------------------------------------------");
    println!(" ");
    println!(">>TASKS<<");
    println!(" ");
    print_tasks(&tasks);
    println!(" ");
    println!("-------------------------------------------------");
}

fn mark_as_done(tasks: &mut Vec<String>) {
    loop {
        println!("-------------------------------------------------");
        println!(" ");
        println!(">>TASKS<<");
        println!(" ");
        print_tasks(&tasks);
        println!(" ");
        println!(
            " (Press 0 to go back to menu) \n\
    Serial Number of task that is done :"
        );
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match index == 0 {
            true => {
                println!(" ");
                println!("-------------------------------------------------");
                break;
            }
            _ => {
                tasks.remove(index - 1);
            }
        }
    }
}

fn main() {
    let mut tasks = Vec::new();

    loop {
        println!(" ");
        println!("TO DO LIST");
        println!(" ");
        println!(
            "1 > Add tasks\n\
             2 > View List\n\
             3 > Mark Tasks as DONE
            "
        );
        println!("(Choose Action)");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        let action: u32 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match action {
            1 => {
                add_new_task(&mut tasks);
                // only use `continue` if there is a chance that below code also has possibility to run. If user enters 1, then there is no point in running below code.
                continue;
            }

            2 => {
                show_tasks(&tasks);
                continue;
            }

            3 => {
                mark_as_done(&mut tasks);
                continue;
            }

            _ => {
                println!("-------------------------------------------------");
                println!("!!Choose an appropriate action!!");
                println!("-------------------------------------------------");
                continue;
            }
        }
    }
}
