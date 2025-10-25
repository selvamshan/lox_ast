use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn gerenate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &["error", "token", "object"],
        &[
            "Assign      : name Token, value Box<Expr>",
            "Binary      : left Box<Expr>, operator Token, right Box<Expr>",
            "Grouping    : expression Box<Expr>",
            "Literal     : value Option<Object>",  
            "Logical     : left Box<Expr>, operator Token, right Box<Expr>",
            "Unary       : operator Token, right Box<Expr>",          
            "Variable    : name Token",
        ],
    )?;

    define_ast(
        output_dir,
        "Stmt",
        &["error", "token", "expr"],
        &[            
            "Block      : statements Vec<Stmt>",
            "Expression : expression Expr",
            "If         : condition Expr, then_branch Box<Stmt>, else_branch Option<Box<Stmt>>",
            "Print      : expression Expr",
            "Var        : name Token, initializer Option<Expr>",
            "While      : condition Expr, body Box<Stmt>"
        ],
    )?;

    Ok(())
}

fn define_ast(
    output_dir: &str,
    base_name: &str,
    imports: &[&str],
    types: &[&str],
) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_type = Vec::new();

    for i in imports {
        writeln!(file, "use crate::{}::*;", i)?;
    }
    writeln!(file)?;

    for ttype in types {
        let parts: Vec<&str> = ttype.split(":").collect();
        let base_class_name = parts[0].trim();
        let class_name = format!("{}{}", parts[0].trim(), base_name);
        let fields_split = parts[1].trim().split(",");
        let mut fields = Vec::new();
        for field in fields_split {
            let (field_name, field_type) = field.trim().split_once(" ").unwrap();
            fields.push(format!(
                "{} {}",
                field_name,
                field_type
            ));
        }
        tree_type.push(TreeType {
            base_class_name: base_class_name.to_string(),
            class_name,
            fields,
        });
    }
    //println!("tree_type: {:?}", tree_type);
    writeln!(file, "pub enum {} {{", base_name)?;
    for ttype in &tree_type {
        writeln!(file, "    {}({}),", ttype.base_class_name, ttype.class_name)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, visitor: &mut dyn {}Visitor<T>) -> Result<T, LoxError> {{",
        base_name
    )?;
    writeln!(file, "        match self {{")?;
    for ttype in &tree_type {
        writeln!(
            file,
            "            {}::{}(v) => v.accept(visitor),",
            base_name, ttype.base_class_name
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}\n")?;

    for ttype in &tree_type {
        writeln!(file, "pub struct {} {{", ttype.class_name)?;
        for field in &ttype.fields {
            let (field_name, field_type) = field.trim().split_once(" ").unwrap();
            writeln!(file, "    pub {}: {},", field_name, field_type)?;
        }
        writeln!(file, "}}\n")?;
    }

    writeln!(file, "pub trait {}Visitor<T> {{", base_name)?;
    for ttype in &tree_type {
        writeln!(
            file,
            "    fn visit_{}_{}(&mut self, {}: &{}) -> Result<T, LoxError>;",
            ttype.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            base_name.to_lowercase(),
            ttype.class_name
        )?;
    }
    write!(file, "}}\n\n")?;

    for ttype in &tree_type {
        writeln!(file, "impl {} {{", ttype.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &mut dyn {}Visitor<T>) -> Result<T, LoxError> {{",
            base_name
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            ttype.base_class_name.to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}\n")?;
    }
    Ok(())
}
