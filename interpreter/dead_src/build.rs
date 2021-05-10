// It's much easier to write and maintain actual code than this code gen mess

use std::env;
use std::fs;
use std::path::Path;

enum Import {
    Use {
        module: &'static str,
        type_name: &'static str,
    },
    Include {
        filename: &'static str,
    },
}

struct Type {
    name: &'static str,
    fields: Vec<Field>,
}

struct Field {
    name: &'static str,
    enum_type: &'static str,
    visitor_type: &'static str,
}

fn main() {
    let out_dir_env = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_env);

    define_ast(
        &out_dir.join("expression.rs"),
        "Expression",
        &[
            Import::Use {
                module: "token",
                type_name: "Token",
            },
            Import::Use {
                module: "object",
                type_name: "Object",
            },
        ],
        &[
            Type {
                name: "Assign",
                fields: vec![
                    Field {
                        name: "name",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                    Field {
                        name: "value",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                ],
            },
            Type {
                name: "Binary",
                fields: vec![
                    Field {
                        name: "left",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                    Field {
                        name: "operator",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                    Field {
                        name: "right",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                ],
            },
            Type {
                name: "Call",
                fields: vec![
                    Field {
                        name: "callee",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                    Field {
                        name: "paren",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                    Field {
                        name: "arguments",
                        enum_type: "Vec<Expr>",
                        visitor_type: "&[Expr]",
                    },
                ],
            },
            Type {
                name: "Get",
                fields: vec![
                    Field {
                        name: "object",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                    Field {
                        name: "name",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                ],
            },
            Type {
                name: "Grouping",
                fields: vec![
                    Field {
                        name: "expression",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                    Field {
                        name: "name",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                ],
            },
            Type {
                name: "Literal",
                fields: vec![Field {
                    name: "object",
                    enum_type: "Object",
                    visitor_type: "&Object",
                }],
            },
            Type {
                name: "Logical",
                fields: vec![
                    Field {
                        name: "left",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                    Field {
                        name: "operator",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                    Field {
                        name: "right",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                ],
            },
            Type {
                name: "Set",
                fields: vec![
                    Field {
                        name: "object",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                    Field {
                        name: "token",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                    Field {
                        name: "value",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                ],
            },
            Type {
                name: "Super",
                fields: vec![
                    Field {
                        name: "keyword",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                    Field {
                        name: "method",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                ],
            },
            Type {
                name: "This",
                fields: vec![Field {
                    name: "keyword",
                    enum_type: "Token",
                    visitor_type: "&Token",
                }],
            },
            Type {
                name: "Unary",
                fields: vec![
                    Field {
                        name: "token",
                        enum_type: "Token",
                        visitor_type: "&Token",
                    },
                    Field {
                        name: "value",
                        enum_type: "Box<Expr>",
                        visitor_type: "&Expr",
                    },
                ],
            },
            Type {
                name: "Variable",
                fields: vec![Field {
                    name: "name",
                    enum_type: "Token",
                    visitor_type: "&Token",
                }],
            },
        ],
    );

    define_ast(
        &out_dir.join("statement.rs"),
        "Smt",
        &[
            Import::Include {
                filename: "expression.rs",
            },
            Import::Use {
                module: "token",
                type_name: "Token",
            },
            Import::Use {
                module: "object",
                type_name: "Object",
            },
        ],
        &[
            Type {
                name: "Print",
                fields: vec![Field {
                    name: "value",
                    enum_type: "Expr",
                    visitor_type: "&Expr",
                }],
            },
            Type {
                name: "Expression",
                fields: vec![Field {
                    name: "value",
                    enum_type: "Expr",
                    visitor_type: "&Expr",
                }],
            },
            Type {
                name: "Var",
                fields: vec![Field {
                    name: "value",
                    enum_type: "Expr",
                    visitor_type: "&Expr",
                }],
            },
            Type {
                name: "Block",
                fields: vec![Field {
                    name: "statements",
                    enum_type: "Vec<Stmt>",
                    visitor_type: "&[Stmt]",
                }],
            },
        ],
    );

    println!("cargo:rerun-if-changed=build.rs");
}

fn define_ast(dest_path: &Path, base_name: &str, imports: &[Import], types: &[Type]) {
    fs::write(
        &dest_path,
        &format!(
            "
{imports}

pub enum {base_name} {{
    {enum_def}
}}

impl {base_name} {{
    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {{
        match self {{
            {accept_def}
        }}
    }}
}}

pub trait Visitor<T> {{
    {visitor_def}
}}
            ",
            imports = imports
                .iter()
                .map(|i| match i {
                    Import::Use { module, type_name } =>
                        format!("use crate::{}::{};", module, type_name),
                    Import::Include { filename } =>
                        format!("include!(concat!(env!(\"OUT_DIR\"), \"/{}\"));", filename),
                })
                .collect::<Vec<String>>()
                .join("\n"),
            base_name = base_name,
            enum_def = types
                .iter()
                .map(|t| format!(
                    "
{} {{
    {}
}},
                    ",
                    t.name,
                    t.fields
                        .iter()
                        .map(|f| format!("{}: {},", f.name, f.enum_type))
                        .collect::<Vec<String>>()
                        .join("\n")
                ))
                .collect::<Vec<String>>()
                .join("\n"),
            accept_def = types
                .iter()
                .map(|t| format!(
                    "{}::{} {{ {vars} }} => visitor.visit_{}({vars}),",
                    base_name,
                    t.name,
                    t.name.to_lowercase(),
                    vars = t
                        .fields
                        .iter()
                        .map(|f| f.name.to_owned())
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
                .collect::<Vec<String>>()
                .join("\n"),
            visitor_def = types
                .iter()
                .map(|t| format!(
                    "fn visit_{}(&mut self, {}) -> T;",
                    t.name.to_lowercase(),
                    t.fields
                        .iter()
                        .map(|f| format!("{}: {}", f.name, f.visitor_type))
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
                .collect::<Vec<String>>()
                .join("\n"),
        ),
    )
    .unwrap();
}
