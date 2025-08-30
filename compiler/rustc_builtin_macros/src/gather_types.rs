use rustc_ast as ast;
use rustc_expand::base::{
    Annotatable, ExtCtxt,
};
use rustc_span::Span;

pub(crate) fn expand(ecx: &mut ExtCtxt<'_>, _expand_span: Span, _meta_item: &ast::MetaItem, mut _item: Annotatable) -> Vec<Annotatable> {
    
    vec![]
}