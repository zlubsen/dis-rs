mod dis;

pub use dis::parse;
pub use dis::parse_as_version;
pub use dis::parse_v6 as parse_v6_pdus;
pub use dis::parse_v7 as parse_v7_pdus;

pub use dis::v6::parse_header as parse_v6_header;
pub use dis::v6::parse_multiple_header as parse_v6_headers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
