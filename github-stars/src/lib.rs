fn sanitize_input(arg: String) -> Result<String, Box<dyn std::error::Error>> {
    let trimmed = arg.trim();
    if trimmed.is_empty() {
        Err("Github repo cannot be empty!".into())
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_fail() {
        let res_empty = sanitize_input(String::from(" "));
        assert!(res_empty.is_err());
        assert_eq!(
            res_empty.unwrap_err().to_string(),
            "Github repo cannot be empty!"
        );
    }

    #[test]
    fn test_sanitize_success() {
        let res_empty = sanitize_input(String::from("hello  "));
        assert_eq!(res_empty.unwrap(), "hello");
    }
}
