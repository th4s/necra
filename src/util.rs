use std::array::TryFromSliceError;
use std::convert::TryFrom;
use thiserror::Error;

/// Converts the first n big endian bytes of a slice into usize
/// Uses padding if necessary
pub(crate) fn usize_from_bytes_be_padded(input: &[u8]) -> Result<usize, NumericError> {
    const SIZE_OF_USIZE: usize = std::mem::size_of::<usize>();
    let input_len = input.len();
    let input = match input_len {
        len @ 0..=SIZE_OF_USIZE => {
            let mut padded = [0_u8; SIZE_OF_USIZE];
            padded[SIZE_OF_USIZE - len..].copy_from_slice(input);
            padded
        }
        _ => <[u8; SIZE_OF_USIZE]>::try_from(&input[..SIZE_OF_USIZE])
            .map_err(NumericError::Conversion)?,
    };
    let out = usize::from_be_bytes(input);
    Ok(out)
}

/// Error for numeric conversions
#[derive(Debug, Error)]
pub enum NumericError {
    #[error("Error during numeric conversion: {0}")]
    Conversion(#[source] TryFromSliceError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_usize_from_bytes_be() {
        let first = vec![0_u8];
        let second = vec![0xff_u8];
        let third = vec![
            0x2a_u8, 0xac_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8, 0xff_u8,
        ];

        assert_eq!(usize_from_bytes_be_padded(&first[..]).unwrap(), 0_usize);
        assert_eq!(usize_from_bytes_be_padded(&second[..]).unwrap(), 255_usize);
        assert_eq!(
            usize_from_bytes_be_padded(&third[..]).unwrap(),
            3075114120563916799_usize
        );
    }
}
