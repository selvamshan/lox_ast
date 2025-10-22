
use std::fs::File;
use std::io::{self,Write};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn gerenate_ast(output_dir: &str) -> io::Result<()> {

    define_ast(output_dir, "Expr",
    &vec!["error", "token", "object"],
     &[
        "Literal     : value Option<Object>",
        "Grouping    : expression Box<Expr>",
        "Unary       : operator Token, right Box<Expr>",
        "Binary      : left Box<Expr>, operator Token, right Box<Expr>"
    ])?;

    define_ast(output_dir, "Stmt", 
    &vec!["error",  "expr"],
    &vec![
        "Expression: expression Expr",
        "Print     : expression Expr",        
    ])?;
    
    Ok(())
}



fn define_ast(output_dir:&str, base_name:&str, imports:&[&str], types:&[&str]) -> io::Result<()> {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_type = Vec::new();

    for i in imports {
        writeln!(file, "use crate::{}::*;", i)?;
    }
    write!(file, "\n")?;
   
    
    for ttype in types {
        let parts:Vec<&str> = ttype.split(":").collect();
        let base_class_name = parts[0].trim();
        let class_name = format!("{}{}", parts[0].trim(), base_name);
        let fields_split = parts[1].trim().split(",");  
        let mut fields = Vec::new();
        for field in fields_split {             
            let (field_name, field_type) = field.trim().split_once(" ").unwrap();
            fields.push(format!("{} {}",field_name.to_string(), field_type.to_string()));           
        }
        tree_type.push(TreeType { base_class_name: base_class_name.to_string(), class_name, fields });
    }
    //println!("tree_type: {:?}", tree_type);
    write!(file, "pub enum {} {{\n", base_name)?;
    for ttype in &tree_type {
        write!(file, "    {}({}),\n", ttype.base_class_name, ttype.class_name)?;
    }
    write!(file, "}}\n\n")?;

    write!(file, "impl {} {{\n", base_name)?;
    write!(file, "    pub fn accept<T>(&self, visitor: &mut dyn {}Visitor<T>) -> Result<T, LoxError> {{\n", base_name)?;
    write!(file, "        match self {{\n")?;
    for ttype in &tree_type {
        write!(file, "            {}::{}(v) => v.accept(visitor),\n", base_name, ttype.base_class_name)?;
    }
    write!(file, "        }}\n")?;
    write!(file, "    }}\n")?;
    write!(file, "}}\n\n")?;

    for ttype in &tree_type {
        write!(file, "pub struct {} {{\n", ttype.class_name)?;
        for field in &ttype.fields {             
            let (field_name, field_type) = field.trim().split_once(" ").unwrap();          
            write!(file, "    pub {}: {},\n", field_name, field_type)?;
        }
        write!(file, "}}\n\n")?;
    }


    write!(file, "pub trait {}Visitor<T> {{\n", base_name)?;
    for ttype in &tree_type {
        write!(file, "    fn visit_{}_{}(&mut self, {}: &{}) -> Result<T, LoxError>;\n", 
            ttype.base_class_name.to_lowercase(), base_name.to_lowercase(), base_name.to_lowercase(), ttype.class_name)?;
    }
    write!(file, "}}\n\n")?;

   
    for ttype in &tree_type {
        write!(file, "impl {} {{\n", ttype.class_name)?;
        write!(file, "    pub fn accept<T>(&self, visitor: &mut dyn {}Visitor<T>) -> Result<T, LoxError> {{\n", base_name)?;
        write!(file, "        visitor.visit_{}_{}(self)\n", ttype.base_class_name.to_lowercase(), base_name.to_lowercase())?;
        write!(file, "    }}\n")?;
        write!(file, "}}\n\n")?;
    }
    Ok(())
}