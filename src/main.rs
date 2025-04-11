use::std::io;
use::std::fs;
use::std::io::Write;
use::std::fs::OpenOptions;

fn main() {
    let mut input_process = String::new();

    println!("Insira o nome do processo:");

    io::stdin()
        .read_line(&mut input_process)
        .expect("Insira novamente!");

    if !input_process.trim().is_empty() {
        let mut loop_breaker = false;
        let mut message = String::new();
        
        let whoami = input_process;
        let message_file_path = "message.txt"; 
        let proc_file_path = "process-list.txt";
        
        let mut proc_list_file = OpenOptions::new()
            .append(true)          
            .create(true)  
            .open(proc_file_path)
            .expect("Erro ao abrir o arquivo");
    
        proc_list_file
            .write_all(whoami.as_bytes())
            .expect("Erro ao escrever no file!");

        while loop_breaker == false {

            let runtime_process_list = match fs::read_to_string(proc_file_path) {
                Ok(content) => content,
                Err(_) => String::new(),
            };

            if !runtime_process_list.trim().is_empty() {
                println!("Process:");

                for line in runtime_process_list.lines() {
                    let line = line;
                    println!("{}", line);
                }    
            }

            let retrieve_message = match fs::read_to_string(message_file_path) {
                Ok(content) => content,
                Err(_) => String::new(),
            };         

            if !retrieve_message.trim().is_empty() {
                for line in retrieve_message.lines() {
                    let line = line;
                    println!("{} -> {}", whoami, line);
                }

                match fs::remove_file(message_file_path) {
                    Ok(_) => continue,
                    Err(e) => eprintln!("Erro ao deletar arquivo: {}", e),
                }    
            }
            
            io::stdin()
                .read_line(&mut message)
                .expect("Insira novamente!");

            if !message.trim().is_empty(){
                let mut message_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(message_file_path)
                    .expect("Erro ao criar arquivo de menssagem");

                message_file.write_all(message.as_bytes()).expect("erro");

            }

            if message.trim() == "exit." {
                loop_breaker = true;
            }

        }   

        



    }
}
