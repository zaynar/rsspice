//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const KWINTV: &[u8] = b"INTERVAL";
const KWSEG: &[u8] = b"SEGMENT";
const LNSIZE: i32 = 80;
const ND: i32 = 2;
const NI: i32 = 6;

//$Procedure      T_CKCOV ( CK coverage, test version )
pub fn T_CKCOV(
    CK: &[u8],
    IDCODE: i32,
    NEEDAV: bool,
    LEVEL: &[u8],
    TOL: f64,
    TIMSYS: &[u8],
    COVER: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut COVER = DummyArrayMut::new(COVER, LBCELL..);
    let mut ARCH = [b' '; LNSIZE as usize];
    let mut KERTYP = [b' '; LNSIZE as usize];
    let mut DC = StackArray::<f64, 2>::new(1..=ND);
    let mut DCTOL = StackArray::<f64, 2>::new(1..=ND);
    let mut DESCR = StackArray::<f64, 5>::new(1..=(ND + ((NI + 1) / 2)));
    let mut ET: f64 = 0.0;
    let mut CLKID: i32 = 0;
    let mut DTYPE: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut IC = StackArray::<i32, 6>::new(1..=NI);
    let mut SEGBEG: i32 = 0;
    let mut SEGEND: i32 = 0;
    let mut AVOK: bool = false;
    let mut FOUND: bool = false;
    let mut ISTDB: bool = false;
    let mut SEGLVL: bool = false;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //

    //
    // Local variables
    //

    //
    // Standard SPICE error handling.
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_CKCOV", ctx)?;

    //
    // Check tolerance value.
    //
    if (TOL < 0.0) {
        spicelib::SETMSG(b"Tolerance must be non-negative; actual value was #.", ctx);
        spicelib::ERRDP(b"#", TOL, ctx);
        spicelib::SIGERR(b"SPICE(VALUEOUTOFRANGE)", ctx)?;
        spicelib::CHKOUT(b"T_CKCOV", ctx)?;
        return Ok(());
    }

    //
    // Use a logical flag to indicate whether this is a segment-level
    // coverage description.
    //
    SEGLVL = spicelib::EQSTR(LEVEL, KWSEG);

    //
    // Check coverage level keyword.
    //
    if !(SEGLVL || spicelib::EQSTR(LEVEL, KWINTV)) {
        spicelib::SETMSG(
            b"Allowed values of LEVEL are # and #; actual value was #.",
            ctx,
        );
        spicelib::ERRCH(b"#", KWSEG, ctx);
        spicelib::ERRCH(b"#", KWINTV, ctx);
        spicelib::ERRCH(b"#", LEVEL, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDOPTION)", ctx)?;
        spicelib::CHKOUT(b"T_CKCOV", ctx)?;
        return Ok(());
    }

    //
    // See whether GETFAT thinks we've got a CK file.
    //
    spicelib::GETFAT(CK, &mut ARCH, &mut KERTYP, ctx)?;

    if fstr::eq(&ARCH, b"XFR") {
        spicelib::SETMSG(b"Input file # has architecture #. The file must be a binary CK file to be readable by this routine.  If the input file is an CK file in transfer format, run TOBIN on the file to convert it to binary format.", ctx);
        spicelib::ERRCH(b"#", CK, ctx);
        spicelib::ERRCH(b"#", &ARCH, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDFORMAT)", ctx)?;
        spicelib::CHKOUT(b"T_CKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&ARCH, b"DAF") {
        spicelib::SETMSG(b"Input file # has architecture #. The file must be a binary CK file to be readable by this routine.  Binary CK files have DAF architecture.  If you expected the file to be a binary CK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        spicelib::ERRCH(b"#", CK, ctx);
        spicelib::ERRCH(b"#", &ARCH, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDARCHTYPE)", ctx)?;
        spicelib::CHKOUT(b"T_CKCOV", ctx)?;
        return Ok(());
    } else if fstr::ne(&KERTYP, b"CK") {
        spicelib::SETMSG(b"Input file # has file type #. The file must be a binary CK file to be readable by this routine. If you expected the file to be a binary CK file, the problem may be due to the file being an old non-native file lacking binary file format information. It\'s also possible the file has been corrupted.", ctx);
        spicelib::ERRCH(b"#", CK, ctx);
        spicelib::ERRCH(b"#", &KERTYP, ctx);
        spicelib::SIGERR(b"SPICE(INVALIDFILETYPE)", ctx)?;
        spicelib::CHKOUT(b"T_CKCOV", ctx)?;
        return Ok(());
    }

    //
    // Set a logical flag indicating whether the time system is SCLK.
    //
    ISTDB = spicelib::EQSTR(TIMSYS, b"TDB");

    //
    // Check time system.
    //
    if !ISTDB {
        if !spicelib::EQSTR(TIMSYS, b"SCLK") {
            spicelib::SETMSG(
                b"Time system spec TIMSYS was #; allowed values are SCLK and TDB.",
                ctx,
            );
            spicelib::ERRCH(b"#", TIMSYS, ctx);
            spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
            spicelib::CHKOUT(b"T_CKCOV", ctx)?;
            return Ok(());
        }
    }

    //
    // If the output time system is TDB, find the clock ID associated
    // with IDCODE.
    //
    if ISTDB {
        spicelib::CKMETA(IDCODE, b"SCLK", &mut CLKID, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_CKCOV", ctx)?;
            return Ok(());
        }
    }

    //
    // Open the file for reading.
    //
    spicelib::DAFOPR(CK, &mut HANDLE, ctx)?;

    if spicelib::FAILED(ctx) {
        spicelib::CHKOUT(b"T_CKCOV", ctx)?;
        return Ok(());
    }

    //
    // We will examine each segment descriptor in the file, and
    // we'll update our coverage bounds according to the data found
    // in these descriptors.
    //
    // If TOL > 0, we'll apply TOL after we've found the coverage
    // for the zero-tolerance case.
    //
    // If the time system is TDB, we'll convert the times to TDB
    // at the end of this routine.

    //
    // Start a forward search.
    //
    spicelib::DAFBFS(HANDLE, ctx)?;

    //
    // Find the next DAF array.
    //
    spicelib::DAFFNA(&mut FOUND, ctx)?;

    while FOUND {
        //
        // Note:  we check FAILED() at the bottom of this loop; this
        // routine returns if FAILED() returns .TRUE. at that point.
        //
        // Fetch and unpack the segment descriptor.
        //
        spicelib::DAFGS(DESCR.as_slice_mut(), ctx)?;
        spicelib::DAFUS(
            DESCR.as_slice(),
            ND,
            NI,
            DC.as_slice_mut(),
            IC.as_slice_mut(),
        );

        //
        // Let AVOK indicate whether the segment satisfies the
        // angular velocity restriction.
        //
        AVOK = ((IC[4] == 1) || !NEEDAV);

        if ((IC[1] == IDCODE) && AVOK) {
            //
            // This segment is for the body of interest.  If angular
            // velocity is needed, this segment has it.
            //
            if SEGLVL {
                //
                // This is a segment-level summary.
                //
                // Insert the coverage bounds into the coverage window.
                // Adjust the interval using the tolerance.
                //
                DCTOL[1] = intrinsics::DMAX1(&[(DC[1] - TOL), 0.0]);
                DCTOL[2] = (DC[2] + TOL);

                //
                // Convert the time to TDB if necessary.
                //
                if ISTDB {
                    //
                    // Convert the time bounds to TDB before inserting
                    // into the window.
                    //
                    for I in 1..=2 {
                        spicelib::SCT2E(CLKID, DCTOL[I], &mut ET, ctx)?;
                        DCTOL[I] = ET;
                    }
                }

                if (DCTOL[1] <= DCTOL[2]) {
                    spicelib::WNINSD(DCTOL[1], DCTOL[2], COVER.as_slice_mut(), ctx)?;
                }
            } else {
                //
                // We're looking for an interval-level coverage window.
                // This information must be retrieved in a
                // data-type-dependent fashion.  The coverage routines
                // we'll call will, if necessary, adjust intervals by TOL
                // and convert interval times to TDB.
                //
                DTYPE = IC[3];
                SEGBEG = IC[5];
                SEGEND = IC[6];

                if (DTYPE == 1) {
                    spicelib::ZZCKCV01(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 2) {
                    spicelib::ZZCKCV02(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 3) {
                    spicelib::ZZCKCV03(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 4) {
                    spicelib::ZZCKCV04(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 5) {
                    spicelib::ZZCKCV05(
                        HANDLE,
                        SEGBEG,
                        SEGEND,
                        CLKID,
                        DC.as_slice(),
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else if (DTYPE == 6) {
                    T_ZZCKCV06(
                        HANDLE,
                        DESCR.as_slice(),
                        CLKID,
                        TOL,
                        TIMSYS,
                        COVER.as_slice_mut(),
                        ctx,
                    )?;
                } else {
                    spicelib::SETMSG(b"Supported CK data types are 1, 2, 3, 4, 5.  Data type of segment: #. This problem may indicate that you need to update your SPICE Toolkit.", ctx);
                    spicelib::ERRINT(b"#", DTYPE, ctx);
                    spicelib::SIGERR(b"SPICE(NOTSUPPORTED)", ctx)?;
                    spicelib::CHKOUT(b"T_CKCOV", ctx)?;
                    return Ok(());
                }
            }
        }

        spicelib::DAFFNA(&mut FOUND, ctx)?;

        if spicelib::FAILED(ctx) {
            spicelib::CHKOUT(b"T_CKCOV", ctx)?;
            return Ok(());
        }
    }

    //
    // COVER now represents the coverage of the entire file at the
    // granularity indicated by LEVEL, combined with the coverage
    // contained in COVER on input.
    //
    // Release the file.
    //
    spicelib::DAFCLS(HANDLE, ctx)?;

    spicelib::CHKOUT(b"T_CKCOV", ctx)?;
    Ok(())
}
