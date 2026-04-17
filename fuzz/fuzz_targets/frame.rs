#![no_main]

use distributed_fuzzing::{encode_frame, parse_frame};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Some(frame) = parse_frame(data) else {
        return;
    };
    let Some(encoded) = encode_frame(&frame) else {
        return;
    };
    assert_eq!(parse_frame(&encoded), Some(frame));
});
