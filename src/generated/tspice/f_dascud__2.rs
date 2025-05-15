//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//
// CHKFS is a utility subroutine which compares a DAS file summary
// to an expected summary.  Both summaries are packed into integer
// arrays.
//
pub fn CHKFS(
    FSUM: &[i32],
    XFSUM: &[i32],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let FSUM = DummyArray::new(FSUM, 1..);
    let XFSUM = DummyArray::new(XFSUM, 1..);

    testutil::CHCKSI(b"NRESVR", FSUM[1], b"=", XFSUM[1], 0, OK, ctx)?;
    testutil::CHCKSI(b"NRESVC", FSUM[2], b"=", XFSUM[2], 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMR", FSUM[3], b"=", XFSUM[3], 0, OK, ctx)?;
    testutil::CHCKSI(b"NCOMC", FSUM[4], b"=", XFSUM[4], 0, OK, ctx)?;
    testutil::CHCKSI(b"FREE", FSUM[5], b"=", XFSUM[5], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA(1) (CHAR)", FSUM[6], b"=", XFSUM[6], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA(2) (DP)", FSUM[7], b"=", XFSUM[7], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTLA(3) (INT)", FSUM[8], b"=", XFSUM[8], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTRC(1) (CHAR)", FSUM[9], b"=", XFSUM[9], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTRC(2) (DP)", FSUM[10], b"=", XFSUM[10], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTRC(3) (INT)", FSUM[11], b"=", XFSUM[11], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTWD(1) (CHAR)", FSUM[12], b"=", XFSUM[12], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTWD(2) (DP)", FSUM[13], b"=", XFSUM[13], 0, OK, ctx)?;
    testutil::CHCKSI(b"LASTWD(3) (INT)", FSUM[14], b"=", XFSUM[14], 0, OK, ctx)?;

    Ok(())
}
