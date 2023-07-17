pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct ExternalResource;

impl ExternalResource{
    pub fn modify(&mut self) {
        unimplemented!();
    }

    pub fn check(&mut self) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("This will show to stdout if --show-output is enabled");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
