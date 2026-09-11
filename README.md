# Banco de Dados em Memória com Extensões em Lua (`banco-memoria`)

**Integrantes do Grupo:**
- [SEU NOME COMPLETO AQUI]

---

## 🛠️ Como Compilar e Executar

### Pré-requisitos
Apenas o ecossistema padrão da linguagem **Rust** precisa estar instalado na máquina. Nenhuma biblioteca externa do sistema ou interpretador Lua precisa ser instalado separadamente, pois a crate `mlua` embuti o Lua 5.4 de forma estática via `vendored`.

### Compilação
Na raiz do repositório, execute:
```bash
cargo build --release