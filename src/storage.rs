use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Estrutura de armazenamento em memória thread-safe.
#[derive(Clone, Debug, Default)]
pub struct Storage {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Insere ou atualiza um valor no banco
    pub fn set(&self, key: String, value: String) {
        let mut map = self.data.write().unwrap();
        map.insert(key, value);
    }

    /// Busca o valor associado a uma chave
    pub fn get(&self, key: &str) -> Option<String> {
        let map = self.data.read().unwrap();
        map.get(key).cloned()
    }

    /// Consulta genérica: busca se existe alguma chave contendo exatamente determinado valor.
    pub fn find_key_by_value(&self, target_value: &str) -> Option<String> {
        let map = self.data.read().unwrap();
        for (k, v) in map.iter() {
            if v == target_value {
                return Some(k.clone());
            }
        }
        None
    }
}