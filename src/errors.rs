use structs::Token;

error_chain! {
    errors {
        UnexpectedToken(token: Token) {
            description("unexpected token")
            display("unexpected token: '{}'", token)
        }
    }
}
