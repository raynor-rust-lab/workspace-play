fn main() {
    print();
}

fn print() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        // Capture the output of the print function
        let output = std::panic::catch_unwind(|| {
            print();
        });

        // Check if the function executed without panicking
        assert!(output.is_ok());
    }

}


