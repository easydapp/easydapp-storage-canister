#![doc = include_str!("../README.md")]
// #![deny(unreachable_pub)] // ! lib needs to check this item
#![deny(unsafe_code)] // Reject the UNSAFE code
#![deny(missing_docs)] // ! Must write a document
#![warn(rustdoc::broken_intra_doc_links)] // Link validity in the document
#![warn(clippy::future_not_send)] // The object of asynchronous code association must be send
#![deny(clippy::unwrap_used)] // deny unwrap
#![deny(clippy::expect_used)] // deny expect
#![deny(clippy::panic)] // deny panic

mod types;

mod stable;

mod apis;

#[cfg(test)]
mod test;
