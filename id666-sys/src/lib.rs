#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    use super::*;

    #[test]
    fn test_simple() {
        let mut id6: MaybeUninit<id666> = MaybeUninit::uninit();

        let mut spc = *include_bytes!("../test_fixtures/twinbeeeeeeeee.spc");

        unsafe {
            let parse_result = id666_parse(id6.as_mut_ptr(), spc.as_mut_ptr(), spc.len() as u64);
            assert_eq!(parse_result, 0);

            assert_eq!(id6.assume_init().total_len, 4992000);
        }
    }
}
