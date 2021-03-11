# id666

Rust library that uses id666-sys

## usage

```rs
use id666::ID666;

fn main() {
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

```
