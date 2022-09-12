use std::sync::Arc;

use db_utils::Upcast;
use filesystem::db::FilesGroup;
use filesystem::ids::{FileLongId, VirtualFile};
use pretty_assertions::assert_eq;
use smol_str::SmolStr;
use syntax::node::ast::{ItemList, SyntaxFile, TerminalEndOfFile, TokenEndOfFile, Trivia};
use syntax::node::db::SyntaxGroup;
use syntax::node::{SyntaxNode, Terminal, Token, TypedSyntaxNode};

use crate::db::ParserGroup;
use crate::test_utils::ParserDatabaseForTesting;

fn build_empty_file_green_tree(db: &dyn SyntaxGroup) -> SyntaxFile {
    let eof_token = TokenEndOfFile::new_green(db, SmolStr::from(""));
    let eof_terminal = TerminalEndOfFile::new_green(
        db,
        Trivia::new_green(db, vec![]),
        eof_token,
        Trivia::new_green(db, vec![]),
    );
    SyntaxFile::from_syntax_node(
        db,
        SyntaxNode::new_root(
            db,
            SyntaxFile::new_green(db, ItemList::new_green(db, vec![]), eof_terminal),
        ),
    )
}

#[test]
fn test_parser() {
    let db = ParserDatabaseForTesting::default();

    // Parse empty cairo file.
    let file_id = db.intern_file(FileLongId::Virtual(VirtualFile {
        parent: None,
        name: "file.cairo".into(),
        content: Arc::new("".into()),
    }));
    let syntax_file = db.file_syntax(file_id).unwrap();
    let diagnostics = db.file_syntax_diagnostics(file_id);
    assert_eq!(diagnostics.format(&db), "");

    let expected_syntax_file = build_empty_file_green_tree(db.upcast());

    assert_eq!(*syntax_file, expected_syntax_file);
}
