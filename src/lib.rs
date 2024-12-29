pub(crate) mod parser;

use crate::parser::python_core_parser::PythonCoreParser;

pub(crate) fn parse_stuff() -> Result<(), ()> {
    let parser = PythonCoreParser::new();
    Ok(())
}
