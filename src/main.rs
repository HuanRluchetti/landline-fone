use::std::io;
use::std::fs;
use::std::io::Write;
use::std::fs::OpenOptions;
use std::thread;

fn main() {

    let mut input_process = String::new();

    println!("-------------------------------------------");
    println!("Insira o nome do processo:");

    io::stdin()
        .read_line(&mut input_process)
        .expect("Insira novamente!");
    println!("-------------------------------------------");

    if !input_process.trim().is_empty() {
        
        let whoami = input_process;
        let whoami_message = whoami.clone();
        let message_file_path = "message.txt"; 
        let proc_file_path = "process-list.txt";

        let mut message = String::new();
        let mut allocated_process = Vec::<String>::new();
        let mut proc_list_file = OpenOptions::new()
            .append(true)          
            .create(true)  
            .open(proc_file_path)
            .expect("Erro ao abrir o arquivo");
    
        proc_list_file
            .write_all(&whoami.as_bytes())
            .expect("Erro ao escrever no file!");


        let sender = thread::spawn(move || {
            loop {
                
                io::stdin()
                .read_line(&mut message)
                .expect("Insira novamente!");

                if !message.trim().is_empty(){
                    let formarted_message = format!("{} -> {}", whoami.trim(), message.trim()); 

                    let mut message_file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(message_file_path)
                        .expect("Erro ao criar arquivo de menssagem");

                    message_file.write_all(formarted_message.as_bytes()).expect("erro");
                }
            }

            });


            let receiver = thread::spawn(move || {

                loop {
                    let runtime_process_list = match fs::read_to_string(proc_file_path) {
                        Ok(content) => content,
                        Err(_) => String::new(),
                    };
        
                    if !runtime_process_list.trim().is_empty() {
                        
                        let mut i: u8 = 0;
                        
                        for line in runtime_process_list.lines() {
                            
                            let line = line;
        
                            if !allocated_process.contains(&line.to_string()) {
                                if i == 0 {
                                    println!("-------------------------------------------");
                                    println!("Process List:");
                                }
        
                                allocated_process.push(line.to_string());
                                println!("{}", line);    
                            }
        
                            i += 1;
                        }    
                    };
        
                    let retrieve_message = match fs::read_to_string(message_file_path) {
                        Ok(content) => content,
                        Err(_) => String::new(),
                    };         
        
                    if !retrieve_message.trim().is_empty() {
                        let mut is_self_message = false;

                        for line in retrieve_message.lines() {
                            let line = line;

                            if !line.contains(whoami_message.trim()) {
                                println!("{}",line);
                                is_self_message = true;
                            }
                        };    
        
                        if is_self_message {
                            fs::remove_file(message_file_path).expect("Erro ao deletar arquivo de menssagem");
                        }
                    }
                }
            });

            receiver.join().expect("Erro na thread do receiver!");
            
            sender.join().expect("Erro na thread do sender!");

    }
}
