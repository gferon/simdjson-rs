use crate::charutils::*;
// begin copypasta
// These chars yield themselves: " \ /
// b -> backspace, f -> formfeed, n -> newline, r -> cr, t -> horizontal tab
// u not handled in this table as it's complex
pub const ESCAPE_MAP: [u8; 256] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x0.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0x2f, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, // 0x4.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x5c, 0, 0, 0, // 0x5.
    0, 0, 0x08, 0, 0, 0, 0x0c, 0, 0, 0, 0, 0, 0, 0, 0x0a, 0, // 0x6.
    0, 0, 0x0d, 0, 0x09, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0x7.
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// handle a unicode codepoint
// write appropriate values into dest
// src will advance 6 bytes or 12 bytes
// dest will advance a variable amount (return via pointer)
// return true if the unicode codepoint was valid
// We work in little-endian then swap at write time
#[inline(always)]
pub fn handle_unicode_codepoint(mut src_ptr: &[u8], dst_ptr: &mut [u8]) -> (usize, usize) {
    // hex_to_u32_nocheck fills high 16 bits of the return value with 1s if the
    // conversion isn't valid; we defer the check for this to inside the
    // multilingual plane check
    let mut code_point: u32 = hex_to_u32_nocheck(&src_ptr[2..]);
    src_ptr = &src_ptr[6..];
    let mut src_offset = 6;
    // check for low surrogate for characters outside the Basic
    // Multilingual Plane.
    if code_point >= 0xd800 && code_point < 0xdc00 {
        if (src_ptr[0] != b'\\') || src_ptr[1] != b'u' {
            return (0, src_offset);
        }

        let code_point_2: u32 = hex_to_u32_nocheck(&src_ptr[2..]);

        // if the first code point is invalid we will get here, as we will go past
        // the check for being outside the Basic Multilingual plane. If we don't
        // find a \u immediately afterwards we fail out anyhow, but if we do,
        // this check catches both the case of the first code point being invalid
        // or the second code point being invalid.
        if ((code_point | code_point_2) >> 16) != 0 {
            return (0, src_offset);
        }

        code_point = (((code_point - 0xd800) << 10) | (code_point_2 - 0xdc00)) + 0x10000;
        src_offset += 6;
    }
    let offset: usize = codepoint_to_utf8(code_point, dst_ptr);
    dbg!(offset);
    (offset, src_offset)
}
