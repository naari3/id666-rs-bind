use std::mem::MaybeUninit;

#[derive(Debug)]
pub enum ID666Error {
    InvalidData,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID666 {
    pub rip_year: Option<u32>,
    pub rip_month: Option<u32>,
    pub rip_day: Option<u32>,
    pub year: Option<u32>,

    pub play_len: Option<u32>,
    pub total_len: Option<u32>,
    pub len: Option<u32>,
    pub fade: Option<u32>,
    pub intro: Option<u32>,
    pub r#loop: Option<u32>,
    pub end: Option<u32>,
    pub mute: Option<u8>,
    pub loops: Option<u8>,
    pub ost_disc: Option<u8>,
    pub ost_track: Option<u8>,
    pub emulator: Option<u8>,
    pub amp: Option<u32>,
    pub binary: Option<u8>,
}

impl ID666 {
    pub fn from(data: &mut [u8]) -> Result<Self, ID666Error> {
        use id666_sys::*;
        let mut id6: MaybeUninit<id666> = MaybeUninit::uninit();
        unsafe {
            if id666_parse(id6.as_mut_ptr(), data.as_mut_ptr(), data.len() as u64) != 0 {
                return Err(ID666Error::InvalidData);
            }
            let id6 = id6.assume_init();

            Ok(Self {
                rip_year: minus_to_none_i32(id6.rip_year),
                rip_month: minus_to_none_i32(id6.rip_month),
                rip_day: minus_to_none_i32(id6.rip_day),
                year: minus_to_none_i32(id6.year),

                play_len: minus_to_none_i32(id6.play_len),
                total_len: minus_to_none_i32(id6.total_len),
                len: minus_to_none_i32(id6.len),
                fade: minus_to_none_i32(id6.fade),
                intro: minus_to_none_i32(id6.intro),
                r#loop: minus_to_none_i32(id6.loop_),
                end: minus_to_none_i32(id6.end),
                mute: Some(id6.mute),
                loops: Some(id6.loops),
                ost_disc: Some(id6.ost_disc),
                ost_track: Some(id6.ost_track),
                emulator: Some(id6.emulator),
                amp: Some(id6.amp),
                binary: Some(id6.binary),
            })
        }
    }
}

fn minus_to_none_i32(i: i32) -> Option<u32> {
    if i > 0 {
        Some(i as u32)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut data = include_bytes!("../test_fixtures/twinbeeeeeeeee.spc").to_vec();
        let id6 = ID666::from(&mut data).unwrap();

        assert_eq!(
            id6,
            ID666 {
                rip_year: Some(2021),
                rip_month: Some(3),
                rip_day: Some(6),
                year: None,

                play_len: Some(4352000),
                total_len: Some(4992000),
                len: Some(4352000),
                fade: Some(640000),
                intro: None,
                r#loop: None,
                end: None,
                mute: Some(0),
                loops: Some(0),
                ost_disc: Some(0),
                ost_track: Some(0),
                emulator: Some(48),
                amp: Some(65536),
                binary: Some(0),
            }
        );
    }
}
