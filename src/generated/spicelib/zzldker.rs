//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

const WDSIZE: i32 = 32;
const EOLTSZ: i32 = 5;
const KLINSZ: i32 = 132;

//$Procedure ZZLDKER ( Load a kernel )
pub fn ZZLDKER(
    FILE: &[u8],
    NOFILE: &[u8],
    FILTYP: &mut [u8],
    HANDLE: &mut i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut VERSN = [b' '; WDSIZE as usize];
    let mut ARCH = [b' '; WDSIZE as usize];
    let mut MYTYPE = [b' '; WDSIZE as usize];
    let mut TERMIN = [b' '; EOLTSZ as usize];
    let mut KERLIN = [b' '; KLINSZ as usize];

    //
    // SPICELIB Functions
    //

    //
    // Local Variables.
    //

    if RETURN(ctx) {
        return Ok(());
    }

    CHKIN(b"ZZLDKER", ctx)?;

    if !EXISTS(FILE, ctx)? {
        SETMSG(NOFILE, ctx);
        ERRCH(b"#", FILE, ctx);
        ERRCH(b"#", b"could not be located.", ctx);
        SIGERR(b"SPICE(NOSUCHFILE)", ctx)?;
        CHKOUT(b"ZZLDKER", ctx)?;
        return Ok(());
    }

    GETFAT(FILE, &mut ARCH, &mut MYTYPE, ctx)?;

    //
    // Possible values for the architecture are:
    //
    //    DAF -- The file is based on the DAF architecture.
    //    DAS -- The file is based on the DAS architecture.
    //    XFR -- The file is in a SPICE transfer file format.
    //    DEC -- The file is an old SPICE decimal text file.
    //    ASC -- An ASCII text file.
    //    KPL -- Kernel Pool File (i.e., a text kernel)
    //    TXT -- An ASCII text file.
    //    TE1 -- Text E-Kernel type 1.
    //     ?  -- The architecture could not be determined.
    //
    // Some of these are obviously losers.
    //
    if (fstr::eq(&ARCH, b"XFR") || fstr::eq(&ARCH, b"DEC")) {
        SETMSG(NOFILE, ctx);
        ERRCH(b"#", FILE, ctx);
        ERRCH(
            b"#",
            b"is a transfer format file. Transfer format files cannot be loaded. ",
            ctx,
        );
        SIGERR(b"SPICE(TRANSFERFILE)", ctx)?;
        CHKOUT(b"ZZLDKER", ctx)?;
        return Ok(());
    } else if fstr::eq(&ARCH, b"TE1") {
        SETMSG(NOFILE, ctx);
        ERRCH(b"#", FILE, ctx);
        ERRCH(
            b"#",
            b"is a type 1 text E-kernel.  These files are obsolete and cannot be loaded. ",
            ctx,
        );
        SIGERR(b"SPICE(TYPE1TEXTEK)", ctx)?;
        CHKOUT(b"ZZLDKER", ctx)?;
        return Ok(());
    }

    //
    // That takes care of the obvious errors.  Try loading the
    // kernel.
    //
    *HANDLE = 0;
    fstr::assign(FILTYP, b" ");

    if fstr::eq(&ARCH, b"DAF") {
        if fstr::eq(&MYTYPE, b"SPK") {
            SPKLEF(FILE, HANDLE, ctx)?;
        } else if fstr::eq(&MYTYPE, b"CK") {
            CKLPF(FILE, HANDLE, ctx)?;
        } else if fstr::eq(&MYTYPE, b"PCK") {
            PCKLOF(FILE, HANDLE, ctx)?;
        } else {
            TKVRSN(b"TOOLKIT", &mut VERSN);
            SETMSG(NOFILE, ctx);
            ERRCH(b"#", FILE, ctx);
            ERRCH(b"#", b"is a \"#\" DAF file. This kind of binary file is not supported in version # of the SPICE toolkit. Check with NAIF to see if your toolkit version is up to date. ", ctx);
            ERRCH(b"#", &MYTYPE, ctx);
            ERRCH(b"#", &VERSN, ctx);
            SIGERR(b"SPICE(UNKNOWNKERNELTYPE)", ctx)?;
            CHKOUT(b"ZZLDKER", ctx)?;
            return Ok(());
        }

        fstr::assign(FILTYP, &MYTYPE);
    } else if fstr::eq(&ARCH, b"DAS") {
        if fstr::eq(&MYTYPE, b"EK") {
            EKLEF(FILE, HANDLE, ctx)?;
        } else if fstr::eq(&MYTYPE, b"DSK") {
            ZZDSKLSF(FILE, HANDLE, ctx)?;
        } else {
            TKVRSN(b"TOOLKIT", &mut VERSN);
            SETMSG(NOFILE, ctx);
            ERRCH(b"#", FILE, ctx);
            ERRCH(b"#", b"is a \"#\" DAS file.  This kind of binary file is not supported in version # of the SPICE toolkit. Check with NAIF to see if your toolkit version is up to date. ", ctx);
            ERRCH(b"#", &MYTYPE, ctx);
            ERRCH(b"#", &VERSN, ctx);
            SIGERR(b"SPICE(UNKNOWNKERNELTYPE)", ctx)?;
            CHKOUT(b"ZZLDKER", ctx)?;
            return Ok(());
        }

        fstr::assign(FILTYP, &MYTYPE);
    } else {
        //
        // Check for line terminator compatibility with this platform.
        // .TRUE. means that ZZASCII will compare terminator
        // detected with the one native to this platform and will
        // stop if they don't match.
        //
        ZZASCII(FILE, &mut KERLIN, true, &mut TERMIN, ctx)?;

        //
        // Load the file using the text file loader.
        //
        LDPOOL(FILE, ctx)?;

        if !FAILED(ctx) {
            fstr::assign(FILTYP, b"TEXT");
            //
            // Cause the kernel pool mechanism to perform
            // the standard error checks on the pool
            // data.
            //
            ZZBODKIK(ctx)?;
        }
    }

    CHKOUT(b"ZZLDKER", ctx)?;
    Ok(())
}
