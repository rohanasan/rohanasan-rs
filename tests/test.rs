use rohanasan;
#[cfg(test)]
mod url_test {
    use rohanasan::url_decode;

    #[test]
    fn it_works() {
        assert_eq!(String::from("hello world"), url_decode("hello%20world"));
    }
}
