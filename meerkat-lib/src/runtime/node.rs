use crate::runtime::Manager;
use crate::runtime::ast::Stmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub path: String,
    pub ast: Vec<Stmt>,
}

pub struct Node {
    pub programs: Vec<Program>,
    pub manager: Manager,
}

impl Node {
    pub fn new() -> Self {
        Self {
            programs: Vec::new(),
            manager: Manager::new(),
        }
    }

    pub fn load_program(&mut self, path: &str) -> Result<(), String> {
        if self.programs.iter().any(|p| p.path == path) {
            return Err(format!("Program at path '{}' is already loaded", path));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

        let ast = crate::runtime::parser::parser::parse_string(&content)?;

        self.programs.push(Program {
            path: path.to_string(),
            ast,
        });

        Ok(())
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}
