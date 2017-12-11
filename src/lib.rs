#[macro_use] extern crate error_chain;
extern crate failure;

use failure::Error;

mod a {
    mod error {
        error_chain! {
            errors {
                AnError {
                    description("an error")
                }
            }
        }
    }

    pub fn foo() -> Result<(), error::Error> {
        Err(error::ErrorKind::AnError.into())
    }
}

pub fn bar() -> Result<(), Error> {
    a::foo()?;
    Ok(())
}
