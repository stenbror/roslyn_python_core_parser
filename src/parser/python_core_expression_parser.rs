
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use super::python_core_parser::PythonCoreParser;

// Trait for expression grammar rule ///////////////////////////////////////////////////////////////
trait ExpressionRules {
    fn parse_named_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_no_cond_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_lambda_def_expr(&mut self, is_conditional: bool) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_or_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_and_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_not_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comparison_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_star_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_xor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_and_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_shift_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_arith_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_term_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_factor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_power_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_await_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_list_comp_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_trailer_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_subscript_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_subscript_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_expr_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_dictionary_set_maker_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_arg_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_argument_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comp_iter_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_sync_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comp_if_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_yield_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

// Implementing all expression grammar rules ///////////////////////////////////////////////////////
impl ExpressionRules for PythonCoreParser {
    fn parse_named_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_test_no_cond_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_lambda_def_expr(&mut self, is_conditional: bool) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_or_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_and_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_not_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_comparison_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_star_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_xor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_and_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_shift_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_arith_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_term_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_factor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_power_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_await_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_test_list_comp_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_trailer_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_subscript_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_subscript_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_expr_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_test_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_dictionary_set_maker_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_arg_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_argument_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_comp_iter_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_sync_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_comp_if_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_yield_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(1 == 1, true);
    }
}