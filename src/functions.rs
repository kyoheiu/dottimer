use super::errors::MyError;

pub fn input_to_numvec(s: String) -> Result<Vec<u16>, MyError> {
    let mut result: Vec<u16> = vec![];
    for n in s.split_ascii_whitespace() {
        let x = n.parse()?;
        if x >= 1 && x <= 5 {
            result.push(x);
        }
    }
    Ok(result)
}
