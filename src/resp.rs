use crate::resp_result::{RESPError, RESPResult};

fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<u8>> {
    // Vector to store the buffer
    let mut output = Vec::new();

    if *index >= buffer.len() {
        return Err(RESPError::OutOfBounds(*index));
    }

    // as we have 2 character \r\n terminator
    // we will keep track of the previous element
    // of the buffer
    let mut previous_elem: u8 = buffer[*index].clone();

    // flag to signal the character \n\r exits
    let mut seperator_found: bool = false;

    // temp track of what we are reading
    // in the buffer
    let mut final_index: usize = *index;

    // scan the buffer for \r\n
    // keep track of the previous element
    for &elem in buffer[*index..].iter() {
        final_index += 1;
        // Check if we just passed the terminator \r\n.
        if elem == b'\n' && previous_elem == b'\r' {
            //toggle the flag to set characters found
            seperator_found = true;
            break;
        }
        // Store the current element for
        previous_elem = elem.clone();
    }

    // check if the seperator_found flag
    // is true else return erro
    if !seperator_found {
        *index = final_index;
        return Err(RESPError::OutOfBounds(*index));
    }

    output.extend_from_slice(&buffer[*index..final_index - 2]);

    *index = final_index;

    Ok(output)
}

fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let output = binary_extract_line(buffer, index)?;

    Ok(String::from_utf8(output)?)
}

#[cfg(test)]
mod tests {

    use crate::resp_result::RESPError;

    use super::*;

    #[test]
    fn test_binary_extract_line() {
        let buffer = "OK\r\n".as_bytes();
        let mut index: usize = 0;

        let output = binary_extract_line(buffer, &mut index).unwrap();

        assert_eq!(output, "OK".as_bytes());
        assert_eq!(index, 4);
    }

    #[test]
    fn test_binary_extract_line_longer_string() {
        let buffer = "ECHO\r\n".as_bytes();
        let mut index: usize = 0;

        let output = binary_extract_line(buffer, &mut index).unwrap();

        assert_eq!(output, "ECHO".as_bytes());
        assert_eq!(index, 6);
    }

    #[test]
    fn test_binary_extract_line_empty_buffer() {
        let buffer = "".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 0);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_no_seperator() {
        // Test that the function binary_extract_line
        // returns the correct error when we try to
        // read a buffer that doesn't contain the
        // terminator \r\n.
        let buffer = "Ok".as_bytes();

        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_index_to_advance() {
        let buffer = "Ok".as_bytes();
        let mut index: usize = 1;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 2);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extracr_line_half_seperator() {
        let buffer = "Ok\r".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_incorrect_separator() {
        let buffer = "OK\n".as_bytes();
        let mut index: usize = 0;
        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => {
                assert_eq!(index, 3);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_as_string() {
        let buffer = "Ok\r\n".as_bytes();
        let mut index: usize = 0;

        let output = binary_extract_line_as_string(buffer, &mut index).unwrap();
        assert_eq!(output, String::from("Ok"));
        assert_eq!(index, 4);
    }
}
