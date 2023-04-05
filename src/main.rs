use ::std::fs::File;
use colored::*;
use std::io::{self, BufWriter, Write};

struct Task {
    name: String,
    status: bool,
}

fn main() {
    println!("{}", "TODO - list".cyan());
    println!("First Time? Use /help for commands");
    //Main Vector of all Tasks
    let mut alltasks: Vec<Task> = Vec::new();

    //Creates a file to save data at Save Command
    let file = File::create("data.txt").expect("Failed to create file");

    //Functioning

    loop {
        //Takes Input for Action
        let mut action = String::new();
        println!("");
        print!("{}", "/".green());
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut action)
            .expect("Error reading from STDIN");

        let action = action.trim();
        let action: &str = &action[..];

        match action {
            "add" => {
                println!("");
                println!("{}", "Add Tasks : ".green());
                loop {
                    let mut entry = String::new();
                    print!(">");
                    let _ = io::stdout().flush();
                    io::stdin()
                        .read_line(&mut entry)
                        .expect("Error reading from STDIN");
                    let entry = entry.trim();

                    match entry == "" {
                        true => {
                            break;
                        }
                        _ => {
                            let new: Task = Task {
                                name: String::from(entry),
                                status: false,
                            };
                            alltasks.push(new);
                        }
                    }
                }
            }

            "rem" => loop {
                println!("");
                println!("{}", "Tasks".yellow());
                let mut i: u32 = 1;
                for ele in &alltasks {
                    println!("{i}. {}", ele.name);
                    i += 1;
                }

                println!(
                    r"{}{}",
                    "Select the Task Number to remove".cyan(),
                    "( Type 0 to go back )"
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
                        break;
                    }
                    _ => {
                        if index > alltasks.len() {
                            println!("{}", "Enter an Appropriate Number".red());
                            continue;
                        } else {
                            alltasks.remove(index - 1);
                        }
                    }
                }
            },

            "done" => {
                loop {
                    //VIEW ALL
                    println!("");
                    println!("{}", "Tasks".yellow());
                    let mut i: u32 = 1;
                    for ele in &alltasks {
                        if ele.status == false {
                            println!("{i}. {}", ele.name);
                        } else {
                            println!("{i}. {}  {}", ele.name.green(), "✓".green());
                        }
                        i += 1;
                    }

                    println!(r"{}", "Select the Task Number to Mark as done".cyan(),);
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
                            break;
                        }
                        _ => {
                            if index > alltasks.len() {
                                println!("{}", "Enter an Appropriate Number".red());
                                continue;
                            } else {
                                alltasks[index - 1].status = true;
                                //Prints tasks
                            }
                        }
                    }
                }
            }

            "va" => {
                println!("");
                println!("{}", "Tasks".yellow());
                let mut i: u32 = 1;
                for ele in &alltasks {
                    if ele.status == false {
                        println!("{i}. {}", ele.name);
                    } else {
                        println!("{i}. {}  {}", ele.name.green(), "✓".green());
                    }
                    i += 1;
                }
            }
            "vi" => {
                println!("");
                println!("{}", "Incomplete Tasks".red());
                let mut i: u32 = 1;
                for ele in &alltasks {
                    if ele.status == false {
                        println!("{i}. {}", ele.name);
                    }
                    i += 1;
                }
            }

            "vc" => {
                println!("");
                println!("{}", "Complete Tasks".green());
                let mut i: u32 = 1;
                for ele in &alltasks {
                    if ele.status == true {
                        println!("{i}. {}", ele.name);
                    }
                    i += 1;
                }
            }
            "save" => {
                println!("");
                let mut writer = BufWriter::new(&file);
                writeln!(writer, "{}", "ALL Tasks").expect("Failed to write to file");
                let mut i = 0;
                for task in &alltasks {
                    writeln!(writer, "{i}.  {}  (done = {})", task.name, task.status)
                        .expect("Failed to write to file");
                    i += 1;
                }
                println!("{}", "Saved Sucessfully to data.txt".green());
            }

            "sum" => {
                println!("");
                println!("{}", "Summary".yellow());
                let mut comp: u32 = 0;
                for ele in &alltasks {
                    if ele.status == true {
                        comp += 1;
                    }
                }
                println!("Tasks Completed : {}/{}", comp, alltasks.len());
            }

            "help" => {
                println!("");
                println!("{}", "Commands".yellow());
                println!(
                    "add - Add Tasks
rem - Remove a Task
va - View All Tasks
vc - View Completed Tasks
vi - View Incomplete Tasks
done - Mark a Task as Done
sum - Summary
save - Save all the tasks to a Text File named (data.txt)
help - to open this;"
                );
            }
            _ => {
                println!("");
                println!("{}", "Enter Proper Command".red());
            }
        }
    }

}

