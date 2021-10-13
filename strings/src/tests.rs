#[test]
fn unique_char_test() {
    assert_eq!(crate::has_unique_chars("abcdefghi"), true);
    assert_eq!(crate::has_unique_chars("asdfsdfa"), false);
}

#[test]
#[should_panic]
fn unique_char_panic_test() {
    crate::has_unique_chars("مرحبا بالعالم!");
}

#[test]
fn permutation_test() {
    assert_eq!(crate::is_permutation("abc", "cba"), true);
    assert_eq!(crate::is_permutation("pelicano", "canopeli"), true);
    assert_eq!(crate::is_permutation("monja", "jamon"), true);
    assert_eq!(crate::is_permutation("aaaaaa", "bb"), false);
    assert_eq!(crate::is_permutation("avion", "perro"), false);
}

#[test]
fn urlify_test() {
    assert_eq!(crate::urlify("hola mundo lindo!"), "hola%20mundo%20lindo!");
}

#[test]
fn base_64() {
    assert_eq!(crate::encode_base64("Hola Mundo!"), "SG9sYSBNdW5kbyE=");
}