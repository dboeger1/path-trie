use path_trie::PathTrie;
use std::path::PathBuf;


#[test]
fn test_positive_empty() {
    let trie = PathTrie::new();

    let elements: Vec<PathBuf> = trie.iter().collect();
    assert!(elements.is_empty());
}

#[test]
fn test_positive_insert_root_absolute() -> Result<(), String> {
    let path = PathBuf::from("/");

    let mut trie = PathTrie::new();
    trie.insert(&path)?;

    let elements: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(vec![path], elements);

    Ok(())
}

#[test]
fn test_positive_insert_root_relative() -> Result<(), String> {
    let path = PathBuf::from("relative");

    let mut trie = PathTrie::new();
    trie.insert(&path)?;

    let elements: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(vec![path], elements);

    Ok(())
}

#[test]
fn test_positive_insert_root_here() -> Result<(), String> {
    let path = PathBuf::from(".");

    let mut trie = PathTrie::new();
    trie.insert(&path)?;

    let elements: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(vec![path], elements);

    Ok(())
}

#[test]
fn test_positive_insert_root_parent() -> Result<(), String> {
    let path = PathBuf::from("..");

    let mut trie = PathTrie::new();
    trie.insert(&path)?;

    let elements: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(vec![path], elements);

    Ok(())
}

#[test]
fn test_positive_insert_roots_2() -> Result<(), String> {
    let paths = (
        PathBuf::from("/"),
        PathBuf::from("relative"),
        PathBuf::from("."),
        PathBuf::from(".."),
    );

    let mut trie = PathTrie::new();
    trie.insert(&paths.0)?;
    trie.insert(&paths.1)?;
    trie.insert(&paths.2)?;
    trie.insert(&paths.3)?;

    let elements: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(vec![paths.0, paths.1, paths.2, paths.3], elements);

    Ok(())
}
