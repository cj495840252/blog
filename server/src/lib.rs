//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;

/// Echo the user input on the server.
#[server(Echo)]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[server(Upload)]
pub async fn upload(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let a = "a";
        assert!(a == "a");
    }
}
