use itertools::Itertools;
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
fn test_positive_insert_root_relative() -> Result<(), String> {
    let path = PathBuf::from("relative");

    let mut trie = PathTrie::new();
    trie.insert(&path)?;

    let elements: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(vec![path], elements);

    Ok(())
}

#[test]
fn test_positive_insert_roots_2() -> Result<(), String> {
    let paths = vec![
        PathBuf::from("/"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("relative1"),
        PathBuf::from("relative2"),
    ];

    for permutation in Itertools::permutations(paths.iter(), 2) {
        let mut trie = PathTrie::new();
        trie.insert(permutation[0])?;
        trie.insert(permutation[1])?;

        let permutation: Vec<PathBuf> = permutation
            .into_iter()
            .cloned()
            .collect();
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(permutation, elements);
    }

    Ok(())
}

#[test]
fn test_positive_insert_roots_3() -> Result<(), String> {
    let paths = vec![
        PathBuf::from("/"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("relative1"),
        PathBuf::from("relative2"),
    ];

    for permutation in Itertools::permutations(paths.iter(), 3) {
        let mut trie = PathTrie::new();
        trie.insert(permutation[0])?;
        trie.insert(permutation[1])?;
        trie.insert(permutation[2])?;

        let permutation: Vec<PathBuf> = permutation
            .into_iter()
            .cloned()
            .collect();
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(permutation, elements);
    }

    Ok(())
}

#[test]
fn test_positive_insert_roots_4() -> Result<(), String> {
    let paths = vec![
        PathBuf::from("/"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("relative1"),
        PathBuf::from("relative2"),
    ];

    for permutation in Itertools::permutations(paths.iter(), 4) {
        let mut trie = PathTrie::new();
        trie.insert(permutation[0])?;
        trie.insert(permutation[1])?;
        trie.insert(permutation[2])?;
        trie.insert(permutation[3])?;

        let permutation: Vec<PathBuf> = permutation
            .into_iter()
            .cloned()
            .collect();
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(permutation, elements);
    }

    Ok(())
}

#[test]
fn test_positive_insert_roots_5() -> Result<(), String> {
    let paths = vec![
        PathBuf::from("/"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("relative1"),
        PathBuf::from("relative2"),
    ];

    for permutation in Itertools::permutations(paths.iter(), 5) {
        let mut trie = PathTrie::new();
        trie.insert(permutation[0])?;
        trie.insert(permutation[1])?;
        trie.insert(permutation[2])?;
        trie.insert(permutation[3])?;
        trie.insert(permutation[4])?;

        let permutation: Vec<PathBuf> = permutation
            .into_iter()
            .cloned()
            .collect();
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(permutation, elements);
    }

    Ok(())
}

#[test]
fn test_positive_insert_root_children() -> Result<(), String> {
    let paths = (
        PathBuf::from("/"),
        PathBuf::from("/child1"),
        PathBuf::from("/child1/child2"),
        PathBuf::from("/child1/child2/child3"),
        PathBuf::from("/child1/child4"),
        PathBuf::from("/child1/child4/child5"),
    );

    let mut trie = PathTrie::new();

    trie.insert(&paths.2)?;
    let expected = vec![
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.0)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.4)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.5)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.3)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.1)?;
    let expected = vec![
        paths.0.clone(),
        paths.1.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn test_positive_insert_here_children() -> Result<(), String> {
    let paths = (
        PathBuf::from("."),
        PathBuf::from("./child1"),
        PathBuf::from("./child1/child2"),
        PathBuf::from("./child1/child2/child3"),
        PathBuf::from("./child1/child4"),
        PathBuf::from("./child1/child4/child5"),
    );

    let mut trie = PathTrie::new();

    trie.insert(&paths.2)?;
    let expected = vec![
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.0)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.4)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.5)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.3)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.1)?;
    let expected = vec![
        paths.0.clone(),
        paths.1.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn test_positive_insert_parent_children() -> Result<(), String> {
    let paths = (
        PathBuf::from(".."),
        PathBuf::from("../child1"),
        PathBuf::from("../child1/child2"),
        PathBuf::from("../child1/child2/child3"),
        PathBuf::from("../child1/child4"),
        PathBuf::from("../child1/child4/child5"),
    );

    let mut trie = PathTrie::new();

    trie.insert(&paths.2)?;
    let expected = vec![
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.0)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.4)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.5)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.3)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.1)?;
    let expected = vec![
        paths.0.clone(),
        paths.1.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn test_positive_insert_relative_children() -> Result<(), String> {
    let paths = (
        PathBuf::from("relative"),
        PathBuf::from("relative/child1"),
        PathBuf::from("relative/child1/child2"),
        PathBuf::from("relative/child1/child2/child3"),
        PathBuf::from("relative/child1/child4"),
        PathBuf::from("relative/child1/child4/child5"),
    );

    let mut trie = PathTrie::new();

    trie.insert(&paths.2)?;
    let expected = vec![
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.0)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.4)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.5)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.3)?;
    let expected = vec![
        paths.0.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    trie.insert(&paths.1)?;
    let expected = vec![
        paths.0.clone(),
        paths.1.clone(),
        paths.2.clone(),
        paths.3.clone(),
        paths.4.clone(),
        paths.5.clone(),
    ];
    let actual: Vec<PathBuf> = trie.iter().collect();
    assert_eq!(expected, actual);

    Ok(())
}
