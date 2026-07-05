use std::process::Output;

fn binary_extract_line(buffer: &[u8], index: &mut usize) -> Result<Vec<u8>, ()> {
    // Vector to store the buffer
    let mut output = Vec::new();

    // as we have 2 charact \r\n terminator
    // we will keep track of the previous element
    // of the buffer
    let mut previous_elem: u8 = buffer[*index].clone();

    // temp track of what we are reading
    // in the buffer
    let mut final_index: usize = *index;

    // scan the buffer for \r\n
    // keep track of the previous element
    for &elem in buffer[*index..].iter() {
        final_index += 1;
        // Check if we just passed the terminator \r\n.
        if elem == b'\n' && previous_elem == b'\r' {
            break;
        }
        // Store the current element for
        previous_elem = elem.clone();
    }

    output.extend_from_slice(&buffer[*index..final_index - 2]);

    *index = final_index;

    Ok(output)
}

#[cfg(test)]
mod tests {
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
}
