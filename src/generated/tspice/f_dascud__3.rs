//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const NWI: i32 = 256;
const LNSIZE: i32 = 80;

//
// CHKCDS is a utility subroutine which compares a DAS cluster
// directory record to an expected cluster directory record.
//
pub fn CHKCDS(
    CDREC: &[i32],
    XCDREC: &[i32],
    OK: &mut bool,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let CDREC = DummyArray::new(CDREC, 1..);
    let XCDREC = DummyArray::new(XCDREC, 1..);
    let mut TITLE = [b' '; LNSIZE as usize];

    //
    // Local parameters
    //

    //
    // Local variables
    //

    testutil::CHCKSI(b"BWD ptr", CDREC[1], b"=", XCDREC[1], 0, OK, ctx)?;
    testutil::CHCKSI(b"FWD ptr", CDREC[2], b"=", XCDREC[2], 0, OK, ctx)?;
    testutil::CHCKSI(b"Min CHAR addr", CDREC[3], b"=", XCDREC[3], 0, OK, ctx)?;
    testutil::CHCKSI(b"Max CHAR addr", CDREC[4], b"=", XCDREC[4], 0, OK, ctx)?;
    testutil::CHCKSI(b"Min DP addr", CDREC[5], b"=", XCDREC[5], 0, OK, ctx)?;
    testutil::CHCKSI(b"Max DP addr", CDREC[6], b"=", XCDREC[6], 0, OK, ctx)?;
    testutil::CHCKSI(b"Min INT addr", CDREC[7], b"=", XCDREC[7], 0, OK, ctx)?;
    testutil::CHCKSI(b"Max INT addr", CDREC[8], b"=", XCDREC[8], 0, OK, ctx)?;
    testutil::CHCKSI(b"1st type", CDREC[9], b"=", XCDREC[9], 0, OK, ctx)?;
    testutil::CHCKSI(b"1st count", CDREC[10], b"=", XCDREC[10], 0, OK, ctx)?;

    for I in 11..=NWI {
        fstr::assign(&mut TITLE, b"the # cluster count");
        spicelib::REPMOT(&TITLE.clone(), b"#", (I - 9), b"L", &mut TITLE, ctx)?;

        testutil::CHCKSI(&TITLE, CDREC[I], b"=", XCDREC[I], 0, OK, ctx)?;
    }

    Ok(())
}
