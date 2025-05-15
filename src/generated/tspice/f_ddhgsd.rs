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
const LINLEN: i32 = 32;

//$Procedure F_DDHGSD ( ZZDDHGSD Test Family )
pub fn F_DDHGSD(OK: &mut bool, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut OUTARY = ActualCharArray::new(LINLEN, 1..=4);
    let mut OUTPUT = [b' '; LINLEN as usize];

    //
    // Local Parameters
    //

    //
    // Local Variables
    //

    //
    // Start the test family with an open call.
    //
    testutil::TOPEN(b"F_DDHGSD", ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Unknown Class and ID Exceptions", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    fstr::assign(&mut OUTPUT, b" ");

    //
    // Invoke the module.
    //
    spicelib::ZZDDHGSD(b"UNKNOWN", 0, &mut OUTPUT, ctx);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSC(b"OUTPUT", &OUTPUT, b"=", b" ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Abnormal Input (case) and (justification)", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    fstr::assign(&mut OUTPUT, b" ");

    //
    // Invoke the module.
    //
    spicelib::ZZDDHGSD(b"   MEthOd", READ, &mut OUTPUT, ctx);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSC(b"OUTPUT", &OUTPUT, b"=", b"READ", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Method Label Lookups", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    fstr::assign(OUTARY.get_mut(1), b" ");
    fstr::assign(OUTARY.get_mut(2), b" ");
    fstr::assign(OUTARY.get_mut(3), b" ");
    fstr::assign(OUTARY.get_mut(4), b" ");

    //
    // Invoke the module.
    //
    spicelib::ZZDDHGSD(b"METHOD", READ, &mut OUTARY[1], ctx);
    spicelib::ZZDDHGSD(b"METHOD", WRITE, &mut OUTARY[2], ctx);
    spicelib::ZZDDHGSD(b"METHOD", SCRTCH, &mut OUTARY[3], ctx);
    spicelib::ZZDDHGSD(b"METHOD", NEW, &mut OUTARY[4], ctx);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSC(b"METHOD(READ)", &OUTARY[1], b"=", b"READ", OK, ctx)?;
    testutil::CHCKSC(b"METHOD(WRITE)", &OUTARY[2], b"=", b"WRITE", OK, ctx)?;
    testutil::CHCKSC(b"METHOD(SCRTCH)", &OUTARY[3], b"=", b"SCRATCH", OK, ctx)?;
    testutil::CHCKSC(b"METHOD(NEW)", &OUTARY[4], b"=", b"NEW", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Architecture Label Lookups", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    fstr::assign(OUTARY.get_mut(1), b" ");
    fstr::assign(OUTARY.get_mut(2), b" ");

    //
    // Invoke the module.
    //
    spicelib::ZZDDHGSD(b"ARCH", DAF, &mut OUTARY[1], ctx);
    spicelib::ZZDDHGSD(b"ARCH", DAS, &mut OUTARY[2], ctx);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSC(b"ARCH(DAF)", &OUTARY[1], b"=", b"DAF", OK, ctx)?;
    testutil::CHCKSC(b"ARCH(DAS)", &OUTARY[2], b"=", b"DAS", OK, ctx)?;

    //
    // --- Case: ------------------------------------------------------
    //
    testutil::TCASE(b"Nominal Binary File Format Label Lookups", ctx)?;

    //
    // Prepare the inputs and output default values.
    //
    fstr::assign(OUTARY.get_mut(1), b" ");
    fstr::assign(OUTARY.get_mut(2), b" ");
    fstr::assign(OUTARY.get_mut(3), b" ");
    fstr::assign(OUTARY.get_mut(4), b" ");

    //
    // Invoke the module.
    //
    spicelib::ZZDDHGSD(b"BFF", BIGI3E, &mut OUTARY[1], ctx);
    spicelib::ZZDDHGSD(b"BFF", LTLI3E, &mut OUTARY[2], ctx);
    spicelib::ZZDDHGSD(b"BFF", VAXGFL, &mut OUTARY[3], ctx);
    spicelib::ZZDDHGSD(b"BFF", VAXDFL, &mut OUTARY[4], ctx);

    //
    // Check for the absence of an exception.
    //
    testutil::CHCKXC(false, b" ", OK, ctx)?;

    //
    // Check outputs.
    //
    testutil::CHCKSC(b"BFF(BIGI3E)", &OUTARY[1], b"=", b"BIG-IEEE", OK, ctx)?;
    testutil::CHCKSC(b"BFF(LTLI3E)", &OUTARY[2], b"=", b"LTL-IEEE", OK, ctx)?;
    testutil::CHCKSC(b"BFF(VAXGFL)", &OUTARY[3], b"=", b"VAX-GFLT", OK, ctx)?;
    testutil::CHCKSC(b"BFF(VAXDFL)", &OUTARY[4], b"=", b"VAX-DFLT", OK, ctx)?;

    //
    // Close out the test family.
    //
    testutil::T_SUCCESS(OK, ctx);
    Ok(())
}
