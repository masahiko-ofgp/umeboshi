use umeboshi::parser::eval;
use fnv::FnvHashMap;

#[test]
fn test_eval() {
    let mut global_env: FnvHashMap<String, String> = FnvHashMap::default();
    let test_form = "(/ (+ 45 (- 7 2)) (* 2 5))".to_string();

    assert_eq!("5".to_string(), eval(&test_form, &mut global_env));

    let test_form2 = "(print Hello, world!)".to_string();

    assert_eq!(
        "Hello, world!".to_string(), 
        eval(&test_form2, &mut global_env)
        );
}
