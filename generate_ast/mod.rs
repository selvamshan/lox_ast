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
        &["error", "token", "object", "rc"],
        &[
            "Assign      : name Token, value Rc<Expr>",
            "Binary      : left Rc<Expr>, operator Token, right Rc<Expr>",
            "Call        : callee Rc<Expr>, paren Token, arguments Vec<Rc<Expr>>",
            "Grouping    : expression Rc<Expr>",
            "Literal     : value Option<Object>",  
            "Logical     : left Rc<Expr>, operator Token, right Rc<Expr>",
            "Unary       : operator Token, right Rc<Expr>",          
            "Variable    : name Token",
        ],
    )?;

    define_ast(
        output_dir,
        "Stmt",
        &["error", "token", "expr", "rc"],
        &[            
            "Block      : statements Rc<Vec<Rc<Stmt>>>",
            "Break      : token Token",
            "Expression : expression Rc<Expr>",
            "Function   : name Token, params Rc<Vec<Token>>, body Rc<Vec<Rc<Stmt>>>",
            "If         : condition Rc<Expr>, then_branch Rc<Stmt>, else_branch Option<Rc<Stmt>>",
            "Print      : expression Rc<Expr>",
            "Return     : keyword Token, value Option<Rc<Expr>>",
            "Var        : name Token, initializer Option<Rc<Expr>>",
            "While      : condition Rc<Expr>, body Rc<Stmt>"
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
        if i == &"rc" {
            writeln!(file, "use std::rc::Rc;")?;
        } else {
            writeln!(file, "use crate::{}::*;", i)?;
        }        
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
        writeln!(file, "    {}(Rc<{}>),", ttype.base_class_name, ttype.class_name)?;
    }
    writeln!(file, "}}\n")?;

    writeln!(file, "impl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, wrapper:Rc<{}>, {}_visitor:  &dyn {}Visitor<T>) -> Result<T, LoxResult> {{",
        base_name,
        base_name.to_lowercase(),
        base_name
    )?;
    writeln!(file, "        match self {{")?;
    for ttype in &tree_type {
        writeln!(
            file,  
            "            {}::{}(v) => {}_visitor.visit_{}_{}(wrapper, &v),",
            base_name, 
            ttype.base_class_name,
            base_name.to_lowercase(),
            ttype.base_class_name.to_lowercase(),
            base_name.to_lowercase()
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
            "    fn visit_{0}_{1}(&self, wrapper: Rc<{2}>, {3}: &{4}) -> Result<T, LoxResult>;",
            ttype.base_class_name.to_lowercase(),
            base_name.to_lowercase(),
            base_name,
            base_name.to_lowercase(),
            ttype.class_name
        )?;
    }
    write!(file, "}}\n\n")?;
    /*
    for ttype in &tree_type {
        writeln!(file, "impl {} {{", ttype.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&mut self, visitor: &dyn {}Visitor<T>) -> Result<T, LoxResult> {{",
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
     */
    Ok(())
}
