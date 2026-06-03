use crate::runtime::ast::{ActionStmt, Decl, Expr, Stmt, Value, Field};

pub struct AstPrinter {
    spaces: usize,
}

impl Default for AstPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        Self { spaces: 2 }
    }

    pub fn with_spaces(spaces: usize) -> Self {
        Self { spaces }
    }

    fn print_indent(&self, indent: usize) {
        print!("{}", " ".repeat(indent * self.spaces));
    }

    pub fn print_program(&self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.print_stmt(stmt, 0);
        }
    }

    pub fn print_stmt(&self, stmt: &Stmt, indent: usize) {
        self.print_indent(indent);
        match stmt {
            Stmt::ActionStmt(action) => {
                println!("ActionStmt:");
                self.print_action_stmt(action, indent + 1);
            }
            Stmt::Update { service, service_id, service_binding_id, decls } => {
                println!("Update: {{ service: \"{}\", service_id: {}, service_binding_id: {} }}", service, service_id.0, service_binding_id.0);
                for decl in decls {
                    self.print_decl(decl, indent + 1);
                }
            }
            Stmt::Connect { path, addr } => {
                println!("Connect: {{ path: \"{}\", addr: \"{}\" }}", path, addr);
            }
            Stmt::Import { path, service, service_id, service_binding_id } => {
                println!("Import: {{ path: \"{}\", service: \"{}\", service_id: {}, service_binding_id: {} }}", path, service, service_id.0, service_binding_id.0);
            }
            Stmt::Service { name, name_id, binding_id, decls } => {
                println!("Service: {{ name: \"{}\", name_id: {}, binding_id: {} }}", name, name_id.0, binding_id.0);
                for decl in decls {
                    self.print_decl(decl, indent + 1);
                }
            }
            Stmt::Test { service, service_id, service_binding_id, stmts } => {
                println!("Test: {{ service: \"{}\", service_id: {}, service_binding_id: {} }}", service, service_id.0, service_binding_id.0);
                for s in stmts {
                    self.print_action_stmt(s, indent + 1);
                }
            }
            Stmt::Watch { expr } => {
                println!("Watch:");
                self.print_expr(expr, indent + 1);
            }
        }
    }

    pub fn print_decl(&self, decl: &Decl, indent: usize) {
        self.print_indent(indent);
        match decl {
            Decl::VarDecl { name, name_id, binding_id, val } => {
                println!("VarDecl: {{ name: \"{}\", name_id: {}, binding_id: {} }}", name, name_id.0, binding_id.0);
                self.print_expr(val, indent + 1);
            }
            Decl::DefDecl { name, name_id, binding_id, val, is_pub } => {
                println!("DefDecl: {{ name: \"{}\", name_id: {}, binding_id: {}, is_pub: {} }}", name, name_id.0, binding_id.0, is_pub);
                self.print_expr(val, indent + 1);
            }
            Decl::TableDecl { name, name_id, binding_id, fields } => {
                println!("TableDecl: {{ name: \"{}\", name_id: {}, binding_id: {} }}", name, name_id.0, binding_id.0);
                for field in fields {
                    self.print_field(field, indent + 1);
                }
            }
        }
    }

    fn print_field(&self, field: &Field, indent: usize) {
        self.print_indent(indent);
        println!("Field: {{ name: \"{}\", name_id: {}, binding_id: {}, type_: {:?} }}", field.name, field.name_id.0, field.binding_id.0, field.type_);
    }

    pub fn print_action_stmt(&self, stmt: &ActionStmt, indent: usize) {
        self.print_indent(indent);
        match stmt {
            ActionStmt::Let { name, expr, name_id, binding_id } => {
                println!("Let: {{ name: \"{}\", name_id: {}, binding_id: {} }}", name, name_id.0, binding_id.0);
                self.print_expr(expr, indent + 1);
            }
            ActionStmt::Expr(expr) => {
                println!("Expr:");
                self.print_expr(expr, indent + 1);
            }
            ActionStmt::Do(expr) => {
                println!("Do:");
                self.print_expr(expr, indent + 1);
            }
            ActionStmt::Assert(expr) => {
                println!("Assert:");
                self.print_expr(expr, indent + 1);
            }
            ActionStmt::Assign { var, expr, name_id, binding_id } => {
                println!("Assign: {{ var: \"{}\", name_id: {}, binding_id: {} }}", var, name_id.0, binding_id.0);
                self.print_expr(expr, indent + 1);
            }
            ActionStmt::Insert { row, table_name, table_name_id, table_binding_id } => {
                println!("Insert: {{ table_name: \"{}\", table_name_id: {}, table_binding_id: {} }}", table_name, table_name_id.0, table_binding_id.0);
                self.print_expr(row, indent + 1);
            }
        }
    }

    pub fn print_expr(&self, expr: &Expr, indent: usize) {
        self.print_indent(indent);
        match expr {
            Expr::Literal { val } => {
                println!("Literal:");
                self.print_value(val, indent + 1);
            }
            Expr::Variable { ident, name_id, binding_id } => {
                println!("Variable: {{ ident: \"{}\", name_id: {}, binding_id: {} }}", ident, name_id.0, binding_id.0);
            }
            Expr::Tuple { val } => {
                println!("Tuple:");
                for v in val {
                    self.print_expr(v, indent + 1);
                }
            }
            Expr::KeyVal { key, key_id, key_binding_id, value } => {
                println!("KeyVal: {{ key: \"{}\", key_id: {}, key_binding_id: {} }}", key, key_id.0, key_binding_id.0);
                self.print_expr(value, indent + 1);
            }
            Expr::Unop { op, expr } => {
                println!("Unop: {{ op: {:?} }}", op);
                self.print_expr(expr, indent + 1);
            }
            Expr::Binop { op, expr1, expr2 } => {
                println!("Binop: {{ op: {:?} }}", op);
                self.print_expr(expr1, indent + 1);
                self.print_expr(expr2, indent + 1);
            }
            Expr::If { cond, expr1, expr2 } => {
                println!("If:");
                self.print_expr(cond, indent + 1);
                self.print_expr(expr1, indent + 1);
                self.print_expr(expr2, indent + 1);
            }
            Expr::Func { params, param_ids, body } => {
                let mapped_param_ids: Vec<(u32, u64)> = param_ids.iter().map(|(s, b)| (s.0, b.0)).collect();
                println!("Func: {{ params: {:?}, param_ids: {:?} }}", params, mapped_param_ids);
                self.print_expr(body, indent + 1);
            }
            Expr::Call { func, args } => {
                println!("Call:");
                self.print_expr(func, indent + 1);
                for arg in args {
                    self.print_expr(arg, indent + 1);
                }
            }
            Expr::Action(stmts) => {
                println!("Action:");
                for stmt in stmts {
                    self.print_action_stmt(stmt, indent + 1);
                }
            }
            Expr::MemberAccess { service, service_id, service_binding_id, member, member_id, member_binding_id } => {
                println!("MemberAccess: {{ service: \"{}\", service_id: {}, service_binding_id: {}, member: \"{}\", member_id: {}, member_binding_id: {} }}", service, service_id.0, service_binding_id.0, member, member_id.0, member_binding_id.0);
            }
            Expr::Select { table_name, table_name_id, table_binding_id, column_names, column_ids, where_clause } => {
                let mapped_column_ids: Vec<(u32, u64)> = column_ids.iter().map(|(s, b)| (s.0, b.0)).collect();
                println!(
                    "Select: {{ table_name: \"{}\", table_name_id: {}, table_binding_id: {}, column_names: {:?}, column_ids: {:?} }}",
                    table_name, table_name_id.0, table_binding_id.0, column_names, mapped_column_ids
                );
                self.print_expr(where_clause, indent + 1);
            }
            Expr::Table { schema, records } => {
                println!("Table:");
                for field in schema {
                    self.print_field(field, indent + 1);
                }
                for record in records {
                    self.print_expr(record, indent + 1);
                }
            }
            Expr::Fold { table_name, table_name_id, table_binding_id, column_name, column_name_id, column_binding_id, operation, identity } => {
                println!(
                    "Fold: {{ table_name: \"{}\", table_name_id: {}, table_binding_id: {}, column_name: \"{}\", column_name_id: {}, column_binding_id: {} }}",
                    table_name, table_name_id.0, table_binding_id.0, column_name, column_name_id.0, column_binding_id.0
                );
                self.print_expr(operation, indent + 1);
                self.print_expr(identity, indent + 1);
            }
        }
    }

    pub fn print_value(&self, val: &Value, indent: usize) {
        self.print_indent(indent);
        match val {
            Value::Number { val } => {
                println!("Number: {}", val);
            }
            Value::Bool { val } => {
                println!("Bool: {}", val);
            }
            Value::String { val } => {
                println!("String: \"{}\"", val);
            }
            Value::Closure { params, param_ids, body, env: _, service_name, service_name_id, service_name_binding_id } => {
                let mapped_param_ids: Vec<(u32, u64)> = param_ids.iter().map(|(s, b)| (s.0, b.0)).collect();
                println!(
                    "Closure: {{ params: {:?}, param_ids: {:?}, service_name: \"{}\", service_name_id: {}, service_name_binding_id: {} }}",
                    params, mapped_param_ids, service_name, service_name_id.0, service_name_binding_id.0
                );
                self.print_expr(body, indent + 1);
            }
            Value::ActionClosure { stmts, env: _, service_name, service_name_id, service_name_binding_id } => {
                println!("ActionClosure: {{ service_name: \"{}\", service_name_id: {}, service_name_binding_id: {} }}", service_name, service_name_id.0, service_name_binding_id.0);
                for stmt in stmts {
                    self.print_action_stmt(stmt, indent + 1);
                }
            }
        }
    }
}
