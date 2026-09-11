#[derive(Debug, PartialEq)]
pub enum Command {
    Add { key: String, value: String },
    Get { key: String },
    Exit,
}

impl Command {
    /// Faz o parse de uma linha bruta e converte no enum Command correspondente.
    pub fn parse(line: &str) -> Result<Command, String> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Err("linha em branco".to_string());
        }

        if trimmed == "EXIT" {
            return Ok(Command::Exit);
        }

        if let Some(rest) = trimmed.strip_prefix("ADD ") {
            let rest = rest.trim_start();
            if let Some((key, value)) = rest.split_once(' ') {
                let key = key.trim();
                let value = value.trim();
                if key.is_empty() || value.is_empty() {
                    return Err("comando ADD exige chave e valor nao vazios".to_string());
                }
                if key.contains(' ') {
                    return Err("a chave nao pode conter espacos".to_string());
                }
                return Ok(Command::Add {
                    key: key.to_string(),
                    value: value.to_string(),
                });
            } else {
                return Err("sintaxe invalida para ADD. Uso: ADD <chave> <valor>".to_string());
            }
        }

        if let Some(rest) = trimmed.strip_prefix("GET ") {
            let key = rest.trim();
            if key.is_empty() {
                return Err("comando GET exige uma chave".to_string());
            }
            if key.contains(' ') {
                return Err("a chave nao pode conter espacos".to_string());
            }
            return Ok(Command::Get {
                key: key.to_string(),
            });
        }

        Err(format!("comando desconhecido: '{}'", trimmed))
    }
}