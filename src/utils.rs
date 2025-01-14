use std::io::Read;

use anyhow::Result;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(std::fs::File::open(input)?))
    }
}
