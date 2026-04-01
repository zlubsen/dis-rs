include!(concat!(
    env!("OUT_DIR"),
    "/",
    env!("TARGET_GENERATED_SISO_REF_010_FILENAME")
));

include!(concat!(
    env!("OUT_DIR"),
    "/",
    env!("TARGET_GENERATED_SISO_1278_GEN3_FILENAME")
));

pub mod other {
    pub mod parser {
        use crate::PduBody;
        use nom::IResult;

        pub fn other_body(input: &[u8]) -> IResult<&[u8], PduBody> {
            todo!()
        }
    }
}

pub(crate) mod utils {
    const NO_REMAINDER: usize = 0;

    /// Struct to hold the length (in bits or bytes) of parts of a padded record.
    /// Such that `data_length` + `padding_length` = `record_length`.
    #[derive(Debug)]
    pub struct PaddedRecordLengths {
        pub data_length: usize,
        pub padding_length: usize,
        pub record_length: usize,
    }

    impl PaddedRecordLengths {
        #[must_use]
        pub fn new(
            data_length_bytes: usize,
            padding_length_bytes: usize,
            record_length_bytes: usize,
        ) -> Self {
            Self {
                data_length: data_length_bytes,
                padding_length: padding_length_bytes,
                record_length: record_length_bytes,
            }
        }
    }

    /// Calculates the length of a data record when padded to `pad_to_num` octets or bits,
    /// given that the length of the data in the record is `data_length`.
    /// The function returns a tuple consisting of the length of the data, the length of the padding, and the total (padded) length of the record.
    ///
    /// For example, a piece of data of 12 bytes that needs to be aligned to 16 bytes will have a
    /// data length of 12 bytes, a padding of 4 bytes and a final length of 12 + 4 bytes. The function will return 16 in this case.
    pub(crate) fn length_padded_to_num(
        data_length: usize,
        pad_to_num: usize,
    ) -> PaddedRecordLengths {
        let data_remaining = data_length % pad_to_num;
        let padding_num = if data_remaining == 0 {
            0usize
        } else {
            pad_to_num - data_remaining
        };
        let record_length = data_length + padding_num;
        assert_eq!(
            record_length % pad_to_num,
            NO_REMAINDER,
            "The length for the data record is not aligned to {pad_to_num} octets. Data length is {data_length} octets."
        );

        PaddedRecordLengths::new(data_length, padding_num, record_length)
    }
}
