use std::io;

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
        println!(" ");
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");
        println!(" ");
        let action: u32 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match action {
            1 => {
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

                    match input == "" {
                        true => {
                            println!("-------------------------------------------------");
                            println!(" ");
                            println!(">>TASKS<<");
                            println!(" ");
                            let mut i: u32 = 1;
                            for el in &tasks {
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
                continue;
            }

            2 => {
                println!("-------------------------------------------------");
                println!(" ");
                println!(">>Tasks TO DO<<");
                println!(" ");
                let mut i: u32 = 1;
                for el in &tasks {
                    println!("{i}. {}", el);
                    i += 1;
                }
                println!(" ");
                println!("Press any key to go back");

                let mut i = String::new();
                io::stdin().read_line(&mut i).expect("Failed to read line");

                println!(" ");
                println!("-------------------------------------------------");
                continue;
            }

            3 => loop {
                println!("-------------------------------------------------");
                println!(" ");
                println!(">>TASKS<<");
                println!(" ");
                let mut i: u32 = 1;
                for el in &tasks {
                    println!("{i}.{}", el);
                    i += 1;
                }

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
            },

            _ => {
                println!("-------------------------------------------------");
                println!("!!Choose an appropriate action!!");
                println!("-------------------------------------------------");
                continue;
            }
        }
    }
}
