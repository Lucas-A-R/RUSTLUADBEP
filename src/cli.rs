use crate::command::Command;
use crate::extensions::ExtensionRegistry;
use crate::storage::Storage;
use std::io::{self, BufRead, IsTerminal, Write};

pub fn run_repl(storage: Storage, extensions: ExtensionRegistry) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let is_terminal = io::stdout().is_terminal();

    loop {
        if is_terminal {
            print!("> ");
            let _ = io::stdout().flush();
        }

        let mut line = String::new();
        let bytes_read = match handle.read_line(&mut line) {
            Ok(n) => n,
            Err(_) => break,
        };

        if bytes_read == 0 {
            // Fim da entrada (EOF via pipe ou Ctrl+D)
            break;
        }

        let line_str = line.trim_end_matches(['\r', '\n']);

        match Command::parse(line_str) {
            Ok(Command::Exit) => break,
            Ok(Command::Add { key, value }) => {
                match extensions.process_add(&key, &value) {
                    Ok(val_to_store) => {
                        storage.set(key, val_to_store);
                        println!("OK");
                    }
                    Err(err_msg) => {
                        format_and_print_error(&err_msg);
                    }
                }
            }
            Ok(Command::Get { key }) => {
                match storage.get(&key) {
                    Some(raw_val) => {
                        match extensions.process_get(&key, &raw_val) {
                            Ok(formatted_val) => println!("{}", formatted_val),
                            Err(err_msg) => {
                                format_and_print_error(&err_msg);
                            }
                        }
                    }
                    None => println!("ERRO: chave inexistente"),
                }
            }
            Err(err_msg) => {
                format_and_print_error(&err_msg);
            }
        }
    }
}

fn format_and_print_error(err_msg: &str) {
    if err_msg.starts_with("ERRO:") {
        println!("{}", err_msg);
    } else {
        println!("ERRO: {}", err_msg);
    }
}