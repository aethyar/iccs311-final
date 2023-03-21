
#[allow(dead_code)]
pub fn not_main1() {
    println!("Byee, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        //
    }
}
