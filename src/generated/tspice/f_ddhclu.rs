//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const FTSIZE: i32 = 5000;
const RSVUNT: i32 = 2;
const SCRUNT: i32 = 1;
const UTSIZE: i32 = ((20 + SCRUNT) + RSVUNT);
const READ: i32 = 1;
const WRITE: i32 = 2;
const SCRTCH: i32 = 3;
const NEW: i32 = 4;
const NUMAMH: i32 = 4;
const BIGI3E: i32 = 1;
const LTLI3E: i32 = 2;
const VAXGFL: i32 = 3;
const VAXDFL: i32 = 4;
const NUMBFF: i32 = 4;
const STRSIZ: i32 = 8;
const STRLEN: i32 = ((STRSIZ + 1) * NUMBFF);
const DAF: i32 = 1;
const DAS: i32 = 2;
const NUMARC: i32 = 2;
const RECL: i32 = 1024;
const FILEN: i32 = 255;
const CBFSIZ: i32 = 1024;

//$Procedure F_DDHCLU ( ZZDDHCLU Test Family )
pub fn F_DDHCLU(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NUT: i32 = 0;
    let mut OUTPUT: i32 = 0;
    let mut UTLCK = StackArray::<bool, 23>::new(1..=UTSIZE);

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHCLU", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Empty table exceptional case.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    UTLCK[1] = true;
    NUT = 0;

    OUTPUT = 10;

    //
    // Invoke the module.
    //
    OUTPUT = spicelib::ZZDDHCLU(UTLCK.as_slice(), NUT);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"ZZDDHCLU", OUTPUT, b"=", 0, 0, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Non-empty table, nominal case.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    NUT = 10;

    for I in 1..=NUT {
        UTLCK[I] = false;
    }

    UTLCK[NUT] = true;
    UTLCK[1] = true;

    OUTPUT = 0;

    //
    // Invoke the module.
    //
    OUTPUT = spicelib::ZZDDHCLU(UTLCK.as_slice(), NUT);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"ZZDDHCLU", OUTPUT, b"=", 2, 0, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
