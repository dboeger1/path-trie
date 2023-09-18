use path_trie::PathTrie;
use std::path::PathBuf;

//use path_trie::*;


#[test]
fn it_works() {
    println!("\n========\n");

    let mut trie = PathTrie::new();

    let _ = trie.insert(&PathBuf::from("/a/b/c"));
    let _ = trie.insert(&PathBuf::from("/a/b"));
    let _ = trie.insert(&PathBuf::from("/a"));

    println!("{:#?}", trie);
    for path in trie.iter() {
        println!("{}", path.to_string_lossy());
    }

    println!("\n========");
}
