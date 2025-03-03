//! IMAP parser implementation

mod grammar;
mod response;
#[cfg(test)]
mod tests;
mod types;

use grammar::greeting;
use nom::combinator::map;
use nom::IResult;
use response::ImapResponse;

pub(crate) fn parse(i: &[u8]) -> IResult<&[u8], ImapResponse<'_>> {
    map(greeting, |v| ImapResponse::Greeting(v))(i)
}
