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
const DPSIZE: i32 = 8;

//$Procedure F_DDHIVF ( ZZDDHIVF Test Family )
pub fn F_DDHIVF(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut NSUM = [b' '; DPSIZE as usize];
    let mut BFF: i32 = 0;
    let mut FOUND: bool = false;

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHIVF", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"VAX D-Floating Value \'1.0D0\'", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    BFF = 0;
    FOUND = false;

    //
    // We need to store the following sequence of characters in NSUM:
    //
    //    128 64 0 0 0 0 0 0
    //
    fstr::assign(fstr::substr_mut(&mut NSUM, 1..=1), &intrinsics::CHAR(128));
    fstr::assign(fstr::substr_mut(&mut NSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut NSUM, 3..=3), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 4..=4), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 5..=5), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 6..=6), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 7..=7), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 8..=8), &intrinsics::CHAR(0));

    //
    // Invoke the module.
    //
    spicelib::ZZDDHIVF(&NSUM, &mut BFF, &mut FOUND);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"VAX D-Floating Value \'3.0D0\'", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    BFF = 0;
    FOUND = false;

    //
    // We need to store the following sequence of characters in NSUM:
    //
    //    64 65 0 0 0 0 0 0
    //
    fstr::assign(fstr::substr_mut(&mut NSUM, 1..=1), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut NSUM, 2..=2), &intrinsics::CHAR(65));
    fstr::assign(fstr::substr_mut(&mut NSUM, 3..=3), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 4..=4), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 5..=5), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 6..=6), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 7..=7), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 8..=8), &intrinsics::CHAR(0));

    //
    // Invoke the module.
    //
    spicelib::ZZDDHIVF(&NSUM, &mut BFF, &mut FOUND);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXDFL, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"VAX G-Floating Value \'3.0D0\'", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    BFF = 0;
    FOUND = false;

    //
    // We need to store the following sequence of characters in NSUM:
    //
    //    40 64 0 0 0 0 0 0
    //
    fstr::assign(fstr::substr_mut(&mut NSUM, 1..=1), &intrinsics::CHAR(40));
    fstr::assign(fstr::substr_mut(&mut NSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut NSUM, 3..=3), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 4..=4), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 5..=5), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 6..=6), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 7..=7), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 8..=8), &intrinsics::CHAR(0));

    //
    // Invoke the module.
    //
    spicelib::ZZDDHIVF(&NSUM, &mut BFF, &mut FOUND);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", VAXGFL, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, true, OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Bogus fall through pattern.", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    BFF = 0;
    FOUND = false;

    //
    // We need to store the following sequence of characters in NSUM:
    //
    //   144 64 0 0 0 0 0 0
    //
    fstr::assign(fstr::substr_mut(&mut NSUM, 1..=1), &intrinsics::CHAR(144));
    fstr::assign(fstr::substr_mut(&mut NSUM, 2..=2), &intrinsics::CHAR(64));
    fstr::assign(fstr::substr_mut(&mut NSUM, 3..=3), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 4..=4), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 5..=5), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 6..=6), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 7..=7), &intrinsics::CHAR(0));
    fstr::assign(fstr::substr_mut(&mut NSUM, 8..=8), &intrinsics::CHAR(0));

    //
    // Invoke the module.
    //
    spicelib::ZZDDHIVF(&NSUM, &mut BFF, &mut FOUND);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSI(b"BFF", BFF, b"=", 0, 0, OK, ctx)?;
    testutil::CHCKSL(b"FOUND", FOUND, false, OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
