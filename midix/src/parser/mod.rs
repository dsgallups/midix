#![doc = r#"
# Reader for parsing midi

Inspired by <https://docs.rs/quick-xml/latest/quick_xml/>


## TODO
- [ ] Config


Parser should have read_event() which will YIELD a type from it.
Our types should be refactored such that constructors are crate visible only
and then we can have owned types accordingly. So really reader should have our types

We should probably have a parser that can yield an enum
"#]

pub mod old_reader;
pub mod reader;
pub mod types;
