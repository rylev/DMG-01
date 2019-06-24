#[inline(always)]
pub fn bit(condition: bool) -> u8 {
    if condition {
        1
    } else {
        0
    }
}