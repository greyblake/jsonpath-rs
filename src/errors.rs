error_chain! {
    errors {
        Parse(msg: String) {
            description("parse error")
            display("{}", msg)
        }
    }

    foreign_links {
        ParseIntError(::std::num::ParseIntError);
    }
}
