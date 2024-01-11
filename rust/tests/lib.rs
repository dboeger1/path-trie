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
fn test_positive_insert_roots() -> Result<(), String> {
    let roots = vec![
        PathBuf::from("/"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("relative1"),
        PathBuf::from("relative2"),
    ];

    for root_count in 2..=5 {
        for permutation in Itertools::permutations(roots.iter(), root_count) {
            let mut trie = PathTrie::new();
            for root_index in 0..=(root_count - 1) {
                trie.insert(permutation[root_index])?;
            }

            let permutation: Vec<PathBuf> = permutation
                .into_iter()
                .cloned()
                .collect();
            let elements: Vec<PathBuf> = trie.iter().collect();
            assert_eq!(permutation, elements);
        }
    }

    Ok(())
}

#[test]
fn test_positive_insert_root_children() -> Result<(), String> {
    let roots = vec![
        PathBuf::from("/"),
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("relative"),
    ];

    for root in roots {
        let paths = vec![
            root.clone(),
            root.join("child1"),
            root.join("child1/child2"),
            root.join("child1/child2/child3"),
            root.join("child1/child4"),
            root.join("child1/child4/child5"),
        ];

        let mut trie = PathTrie::new();

        trie.insert(&paths[2])?;
        let expected = vec![
            paths[2].clone(),
        ];
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(expected, elements);

        trie.insert(&paths[0])?;
        let expected = vec![
            paths[0].clone(),
            paths[2].clone(),
        ];
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(expected, elements);

        trie.insert(&paths[4])?;
        let expected = vec![
            paths[0].clone(),
            paths[2].clone(),
            paths[4].clone(),
        ];
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(expected, elements);

        trie.insert(&paths[5])?;
        let expected = vec![
            paths[0].clone(),
            paths[2].clone(),
            paths[4].clone(),
            paths[5].clone(),
        ];
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(expected, elements);

        trie.insert(&paths[3])?;
        let expected = vec![
            paths[0].clone(),
            paths[2].clone(),
            paths[3].clone(),
            paths[4].clone(),
            paths[5].clone(),
        ];
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(expected, elements);

        trie.insert(&paths[1])?;
        let expected = vec![
            paths[0].clone(),
            paths[1].clone(),
            paths[2].clone(),
            paths[3].clone(),
            paths[4].clone(),
            paths[5].clone(),
        ];
        let elements: Vec<PathBuf> = trie.iter().collect();
        assert_eq!(expected, elements);
    }

    Ok(())
}

#[test]
fn test_negative_insert_empty() {
    let mut trie = PathTrie::new();
    assert!(trie.insert(&PathBuf::from("")).is_err());
}

#[test]
fn test_negative_insert_duplicate() {
    let paths = vec![
        PathBuf::from("/a/b/c/d"),
        PathBuf::from("./a/b/c/d"),
        PathBuf::from("../a/b/c/d"),
        PathBuf::from("a/b/c/d"),
    ];

    for path in paths {
        let mut trie = PathTrie::new();
        let _ = trie.insert(&path);
        assert!(trie.insert(&path).is_err());
    }
}
