use umeboshi::UmeEnv;
use umeboshi::parser::eval;
use fnv::FnvHashMap;

#[test]
fn test_eval() {
    let mut ume_env: UmeEnv = FnvHashMap::default();
    let test_form = "(/ (+ 45 (- 7 2)) (* 2 5))".to_string();

    assert_eq!("5".to_string(), eval(&test_form, &mut ume_env));

    let test_form2 = "(if (eq 1 1) True False)".to_string();

    assert_eq!(
        "True".to_string(), 
        eval(&test_form2, &mut ume_env)
        );
}
