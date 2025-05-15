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

//$Procedure T_DAFOPN ( Open a new DAF file for testing )
pub fn T_DAFOPN(
    FNAME: &[u8],
    OUTBFF: i32,
    UNIT: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    } else {
        spicelib::CHKIN(b"T_DAFOPN", ctx)?;
    }

    //
    // First kill FNAME.
    //
    testutil::KILFIL(FNAME, ctx)?;

    //
    // Now fetch a logical unit.
    //
    spicelib::GETLUN(UNIT, ctx)?;

    //
    // Open the file.
    //
    {
        use f2rust_std::io;

        let specs = io::OpenSpecs {
            unit: Some(*UNIT),
            file: Some(fstr::substr(FNAME, 1..=spicelib::RTRIM(FNAME))),
            access: Some(b"DIRECT"),
            recl: Some(RECL),
            status: Some(b"NEW"),
            ..Default::default()
        };
        IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
    }

    //
    // Check IOSTAT.
    //
    if (IOSTAT != 0) {
        spicelib::SETMSG(b"Unable to open file, #.  IOSTAT was #.", ctx);
        spicelib::ERRCH(b"#", FNAME, ctx);
        spicelib::ERRINT(b"#", IOSTAT, ctx);
        spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
        spicelib::CHKOUT(b"T_DAFOPN", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"T_DAFOPN", ctx)?;
    Ok(())
}
