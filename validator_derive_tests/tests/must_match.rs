use validator::Validate;

#[test]
fn can_validate_valid_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2"))]
        val: String,
        val2: String,
    }

    let s = TestStruct { val: "bob".to_string(), val2: "bob".to_string() };

    assert!(s.validate().is_ok());
}

#[test]
fn not_matching_fails_validation() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2"))]
        val: String,
        val2: String,
    }

    let s = TestStruct { val: "bob".to_string(), val2: "bobby".to_string() };

    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "must_match");
    assert_eq!(errs["val"][0].params["value"], "bob");
    assert_eq!(errs["val"][0].params["other"], "bobby");
}

#[test]
fn can_specify_code_for_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2", code = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct { val: "bob".to_string(), val2: "bobb".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].code, "oops");
}

#[test]
fn can_specify_message_for_must_match() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(must_match(other = "val2", message = "oops"))]
        val: String,
        val2: String,
    }
    let s = TestStruct { val: "bob".to_string(), val2: "bobb".to_string() };
    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"].len(), 1);
    assert_eq!(errs["val"][0].clone().message.unwrap(), "oops");
}

#[test]
fn can_validate_valid_must_match_option_some() {
    #[derive(Debug, Validate)]
    struct TestStructOpt {
        #[validate(must_match(other = "val2"))]
        val: Option<String>,
        val2: Option<String>,
    }

    let s = TestStructOpt { val: Some("bob".to_string()), val2: Some("bob".to_string()) };

    assert!(s.validate().is_ok());
}

#[test]
fn not_matching_fails_validation_option() {
    #[derive(Debug, Validate)]
    struct TestStructOpt {
        #[validate(must_match(other = "val2"))]
        val: Option<String>,
        val2: Option<String>,
    }

    let s = TestStructOpt { val: Some("bob".to_string()), val2: Some("bobby".to_string()) };

    let res = s.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val"));
    assert_eq!(errs["val"][0].code, "must_match");
}

#[test]
fn must_match_none_none_ok_option() {
    #[derive(Debug, Validate)]
    struct TestStructOpt {
        #[validate(must_match(other = "val2"))]
        val: Option<String>,
        val2: Option<String>,
    }

    let s = TestStructOpt { val: None, val2: None };
    assert!(s.validate().is_ok());
}

#[test]
fn required_and_must_match_with_option() {
    #[derive(Debug, Validate)]
    struct TestStructReqOpt {
        #[validate(required, must_match(other = "val2"))]
        val: Option<String>,
        #[validate(required)]
        val2: Option<String>,
    }

    // Both Some and equal -> OK
    let s_ok = TestStructReqOpt { val: Some("x".to_string()), val2: Some("x".to_string()) };
    assert!(s_ok.validate().is_ok());

    // One None -> errors on required (val2) and must_match (val)
    let s_err = TestStructReqOpt { val: Some("x".to_string()), val2: None };
    let res = s_err.validate();
    assert!(res.is_err());
    let err = res.unwrap_err();
    let errs = err.field_errors();
    assert!(errs.contains_key("val2"));
    assert!(errs.contains_key("val"));
}
