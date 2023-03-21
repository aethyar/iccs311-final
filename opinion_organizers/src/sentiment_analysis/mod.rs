
#[allow(dead_code)]
pub fn not_main() {
    println!("Bye, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        //
    }
}
