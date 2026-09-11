use crate::storage::Storage;
use mlua::{Function, Lua, Table};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct ExtensionRegistry {
    lua: Lua,
    // Mapeia o prefixo da extensão (ex: "cpf_") para seu identificador interno
    prefixes: HashMap<String, String>,
}

impl ExtensionRegistry {
    pub fn new(storage: Storage) -> Result<Self, String> {
        let lua = Lua::new();

        // 1. Registra as funções genéricas de consulta ao banco no ambiente global do Lua (`db`)
        let storage_get = storage.clone();
        let db_get_fn = lua
            .create_function(move |_, key: String| Ok(storage_get.get(&key)))
            .map_err(|e| format!("Erro ao criar callback db.get: {}", e))?;

        let storage_find = storage.clone();
        let db_find_fn = lua
            .create_function(move |_, val: String| Ok(storage_find.find_key_by_value(&val)))
            .map_err(|e| format!("Erro ao criar callback db.find_key_by_value: {}", e))?;

        let db_table = lua
            .create_table()
            .map_err(|e| format!("Erro ao criar tabela db: {}", e))?;
        db_table
            .set("get", db_get_fn)
            .map_err(|e| format!("Erro ao registrar db.get: {}", e))?;
        db_table
            .set("find_key_by_value", db_find_fn)
            .map_err(|e| format!("Erro ao registrar db.find_key_by_value: {}", e))?;

        lua.globals()
            .set("db", db_table)
            .map_err(|e| format!("Erro ao definir global db: {}", e))?;

        // 2. Inicializa a tabela interna de extensoes
        let extensions_table = lua
            .create_table()
            .map_err(|e| format!("Erro ao criar tabela de extensoes: {}", e))?;
        lua.globals()
            .set("_EXTENSIONS", extensions_table)
            .map_err(|e| format!("Erro ao registrar global _EXTENSIONS: {}", e))?;

        let mut registry = Self {
            lua,
            prefixes: HashMap::new(),
        };

        registry.load_extensions_from_dir("extensions")?;

        Ok(registry)
    }

    /// Varre o diretório 'extensions/' e carrega todos os arquivos .lua encontrados
    fn load_extensions_from_dir<P: AsRef<Path>>(&mut self, dir_path: P) -> Result<(), String> {
        let path = dir_path.as_ref();
        if !path.exists() || !path.is_dir() {
            return Ok(());
        }

        let entries = fs::read_dir(path)
            .map_err(|e| format!("Erro ao ler diretorio 'extensions': {}", e))?;

        let mut ext_counter = 0;

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("lua") {
                let code = fs::read_to_string(&path)
                    .map_err(|e| format!("Erro ao ler arquivo {:?}: {}", path, e))?;

                ext_counter += 1;
                let ext_id = format!("ext_{}", ext_counter);

                // Executa o script Lua e espera o retorno da tabela da extensão
                let ext_table: Table = self
                    .lua
                    .load(&code)
                    .eval()
                    .map_err(|e| format!("Erro ao carregar extensao {:?}: {}", path, e))?;

                let prefix: String = ext_table
                    .get("prefix")
                    .map_err(|_| format!("Extensao {:?} nao declarou o campo 'prefix'", path))?;

                let extensions_glob: Table = self
                    .lua
                    .globals()
                    .get("_EXTENSIONS")
                    .map_err(|e| format!("Erro ao obter _EXTENSIONS: {}", e))?;

                extensions_glob
                    .set(ext_id.clone(), ext_table)
                    .map_err(|e| format!("Erro ao guardar extensao no _EXTENSIONS: {}", e))?;

                self.prefixes.insert(prefix, ext_id);
            }
        }

        Ok(())
    }

    fn find_matching_extension(&self, key: &str) -> Option<&String> {
        for (prefix, ext_id) in &self.prefixes {
            if key.starts_with(prefix) {
                return Some(ext_id);
            }
        }
        None
    }

    /// Executa a validação/transformação da extensão no comando ADD
    pub fn process_add(&self, key: &str, value: &str) -> Result<String, String> {
        if let Some(ext_id) = self.find_matching_extension(key) {
            let extensions_glob: Table = self
                .lua
                .globals()
                .get("_EXTENSIONS")
                .map_err(|e| e.to_string())?;

            let ext_table: Table = extensions_glob.get(ext_id.as_str()).map_err(|e| e.to_string())?;

            if let Ok(add_fn) = ext_table.get::<_, Function>("add") {
                let result: (bool, String) = add_fn
                    .call((key, value))
                    .map_err(|e| format!("Erro na execucao Lua: {}", e))?;

                if result.0 {
                    Ok(result.1) // Sucesso (retorna valor transformado/validado)
                } else {
                    Err(result.1) // Erro levantado pela extensão
                }
            } else {
                Ok(value.to_string())
            }
        } else {
            Ok(value.to_string())
        }
    }

    /// Executa a formatação/transformação da extensão no comando GET
    pub fn process_get(&self, key: &str, value: &str) -> Result<String, String> {
        if let Some(ext_id) = self.find_matching_extension(key) {
            let extensions_glob: Table = self
                .lua
                .globals()
                .get("_EXTENSIONS")
                .map_err(|e| e.to_string())?;

            let ext_table: Table = extensions_glob.get(ext_id.as_str()).map_err(|e| e.to_string())?;

            if let Ok(get_fn) = ext_table.get::<_, Function>("get") {
                let result: (bool, String) = get_fn
                    .call((key, value))
                    .map_err(|e| format!("Erro na execucao Lua: {}", e))?;

                if result.0 {
                    Ok(result.1)
                } else {
                    Err(result.1)
                }
            } else {
                Ok(value.to_string())
            }
        } else {
            Ok(value.to_string())
        }
    }
}