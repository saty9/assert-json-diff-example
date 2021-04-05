#[cfg(test)]
mod tests {
    use assert_json_diff::assert_json_include;

    #[test]
    fn it_works() {
        let actual = r#"{"a":0,"b":1}"#;
        let expected = r#"{"b":1,"a":0}"#;
        assert_json_include!(actual: actual, expected: expected);
    }
}
