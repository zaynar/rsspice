//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const TKVSIZ: i32 = 8;
const LNSIZE: i32 = 80;

//$Procedure      DSPVRS ( Display Version )
pub fn DSPVRS(PNAME: &[u8], VRSN: &[u8], ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut LINE = [b' '; LNSIZE as usize];
    let mut TKV = [b' '; TKVSIZ as usize];

    spicelib::TKVRSN(b"toolkit", &mut TKV);

    fstr::assign(&mut LINE, PNAME);

    spicelib::SUFFIX(b"Version", 1, &mut LINE);
    spicelib::SUFFIX(VRSN, 1, &mut LINE);
    spicelib::SUFFIX(b", SPICE Toolkit", 0, &mut LINE);
    spicelib::SUFFIX(&TKV, 1, &mut LINE);

    spicelib::TOSTDO(&LINE, ctx)?;

    Ok(())
}
