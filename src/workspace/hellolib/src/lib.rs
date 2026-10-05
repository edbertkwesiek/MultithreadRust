// hellolib/src/lib.rs — already has add() from cargo new --lib
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add(a: u32, b: u32 )-> u32{
    left + right

}
#[cfg]
mod tests{
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

    }


}
