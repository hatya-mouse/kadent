use syntect::parsing::SyntaxSet;

pub(crate) fn kasl_syntax_set() -> SyntaxSet {
    let binary_data: &[u8] = include_bytes!("../../../kasl_syntax/kasl_syntax.pack");
    syntect::dumps::from_binary(binary_data)
}
