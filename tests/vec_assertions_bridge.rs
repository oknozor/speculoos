mod prelude_user {
    #![deny(deprecated)]

    use speculoos::prelude::*;

    #[test]
    fn prelude_users_keep_method_resolution_without_deprecation_warnings() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(3);
        assert_that(&&test_vec).has_length(3);
        assert_that(&test_vec).is_not_empty();

        let empty_vec: Vec<u8> = vec![];
        assert_that(&empty_vec).is_empty();
    }
}

mod direct_importer {
    use speculoos::assert_that;

    #[allow(deprecated)]
    use speculoos::vec::VecAssertions;

    #[allow(deprecated)]
    #[test]
    fn direct_imports_continue_to_resolve_methods() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(3);
        assert_that(&test_vec).is_not_empty();
    }
}

mod both_in_scope {
    use speculoos::prelude::*;
    use speculoos::Spec;

    #[allow(deprecated)]
    use speculoos::vec::VecAssertions;

    #[allow(deprecated)]
    #[test]
    fn qualified_syntax_disambiguates_when_both_traits_are_in_scope() {
        let test_vec = vec![1, 2, 3];
        let test_ref = &test_vec;
        let mut spec = assert_that(&test_ref);

        <Spec<&Vec<i32>> as VecAssertions>::has_length(&mut spec, 3);
        <Spec<&Vec<i32>> as SliceAssertions>::has_length(&mut spec, 3);
    }
}
