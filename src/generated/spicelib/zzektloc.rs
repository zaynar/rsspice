//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TKKEY: i32 = 1;
const TKID: i32 = (TKKEY + 1);
const TKINT: i32 = (TKID + 1);
const TKDP: i32 = (TKINT + 1);
const TKQSTR: i32 = (TKDP + 1);
const TKLPAR: i32 = (TKQSTR + 1);
const TKRPAR: i32 = (TKLPAR + 1);
const TKCOMA: i32 = (TKRPAR + 1);
const TKDOT: i32 = (TKCOMA + 1);
const TKSTAR: i32 = (TKDOT + 1);
const TKEOQ: i32 = (TKSTAR + 1);
const KWLEN: i32 = 32;
const NKEYWD: i32 = 29;
const KWALL: i32 = 1;
const KWAND: i32 = (KWALL + 1);
const KWASND: i32 = (KWAND + 1);
const KWAVG: i32 = (KWASND + 1);
const KWBETW: i32 = (KWAVG + 1);
const KWBY: i32 = (KWBETW + 1);
const KWCNT: i32 = (KWBY + 1);
const KWDSND: i32 = (KWCNT + 1);
const KWDSTN: i32 = (KWDSND + 1);
const KWEQ: i32 = (KWDSTN + 1);
const KWFROM: i32 = (KWEQ + 1);
const KWGE: i32 = (KWFROM + 1);
const KWGRP: i32 = (KWGE + 1);
const KWGT: i32 = (KWGRP + 1);
const KWHAV: i32 = (KWGT + 1);
const KWIS: i32 = (KWHAV + 1);
const KWLE: i32 = (KWIS + 1);
const KWLIKE: i32 = (KWLE + 1);
const KWLT: i32 = (KWLIKE + 1);
const KWMAX: i32 = (KWLT + 1);
const KWMIN: i32 = (KWMAX + 1);
const KWNE: i32 = (KWMIN + 1);
const KWNOT: i32 = (KWNE + 1);
const KWNULL: i32 = (KWNOT + 1);
const KWOR: i32 = (KWNULL + 1);
const KWORDR: i32 = (KWOR + 1);
const KWSEL: i32 = (KWORDR + 1);
const KWSUM: i32 = (KWSEL + 1);
const KWWHER: i32 = (KWSUM + 1);

//$Procedure      ZZEKTLOC ( EK, locate token in tokenized EK query )
pub fn ZZEKTLOC(
    TOKID: i32,
    KWCODE: i32,
    NTOKEN: i32,
    TOKENS: &[i32],
    VALUES: &[i32],
    LOC: &mut i32,
    FOUND: &mut bool,
) {
    let TOKENS = DummyArray::new(TOKENS, 1..);
    let VALUES = DummyArray::new(VALUES, 1..);

    //
    // Error free.
    //
    *FOUND = false;
    *LOC = 1;

    while (*LOC <= NTOKEN) {
        if (TOKENS[*LOC] == TOKID) {
            if (TOKID == TKKEY) {
                //
                // To get a match, the keyword codes must match.
                //
                if (VALUES[*LOC] == KWCODE) {
                    *FOUND = true;
                    return;
                }
            } else {
                //
                // For non-keyword tokens, we're done at this point.
                //
                *FOUND = true;
                return;
            }
        }

        *LOC = (*LOC + 1);
    }
}
