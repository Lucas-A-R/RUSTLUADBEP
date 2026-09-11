mod cli;
mod command;
mod extensions;
mod storage;

use extensions::ExtensionRegistry;
use storage::Storage;

fn main() {
    let storage = Storage::new();
    let extensions = match ExtensionRegistry::new(storage.clone()) {
        Ok(reg) => reg,
        Err(err) => {
            eprintln!("ERRO ao inicializar sistema de extensoes: {}", err);
            std::process::exit(1);
        }
    };

    cli::run_repl(storage, extensions);
}
