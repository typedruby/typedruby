use ast::*;
use ffi::Driver;
use std::rc::Rc;

pub fn parse(source_file: Rc<SourceFile>) -> Ast {
    parse_with_env(source_file, &[])
}

pub fn parse_with_env(source_file: Rc<SourceFile>, env: &[&str]) -> Ast {
    let mut driver = Driver::new(source_file.clone());
    for var in env.iter() {
        driver.declare(var);
    }

    let ast = driver.parse();
    let diagnostics = driver.diagnostics();

    Ast {
        node: ast.map(|node| *node),
        diagnostics: diagnostics.into_iter().map(|(level, message, begin, end)|
            Diagnostic {
                level: level,
                message: message,
                loc: Loc {
                    file: source_file.clone(),
                    begin_pos: begin,
                    end_pos: end,
                },
            }
        ).collect(),
    }
}
