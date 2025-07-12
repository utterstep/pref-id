use pref_id::{Id, SEPARATOR_LENGTH, UUID_STRING_LENGTH, define_id};

define_id!(TestIdImplicitCrate, "test_id_implicit_crate");

#[test]
fn test_id_with_crate_unspecified() {
    use serde_test::{Token, assert_tokens};

    let id = uuid::Uuid::nil();
    let test_id = TestIdImplicitCrate::from(id);

    const EXPECTED_STRING: &str = "test_id_implicit_crate-00000000-0000-0000-0000-000000000000";
    assert_eq!(
        EXPECTED_STRING.len(),
        UUID_STRING_LENGTH + SEPARATOR_LENGTH + TestIdImplicitCrate::PREFIX.len()
    );

    assert_tokens(&test_id, &[Token::String(EXPECTED_STRING)]);
}
