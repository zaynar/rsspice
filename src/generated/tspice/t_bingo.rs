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

//$Procedure T_BINGO ( BINGO: Process binary kernels to alternate BFFs )
pub fn T_BINGO(INAME: &[u8], ONAME: &[u8], OBFF: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut ARCH = [b' '; 8 as usize];
    let mut FNAME = [b' '; FILEN as usize];
    let mut IDWORD = [b' '; 8 as usize];
    let mut TYPE = [b' '; 8 as usize];
    let mut HANDLE: i32 = 0;
    let mut IAMH: i32 = 0;
    let mut IARC: i32 = 0;
    let mut IBFF: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut FOUND: bool = false;

    //
    // SPICELIB Functions
    //

    //
    // Local Variables
    //

    //
    // Standard SPICE error handling
    //
    if spicelib::RETURN(ctx) {
        return Ok(());
    }

    spicelib::CHKIN(b"T_BINGO", ctx)?;

    //
    // First things first, check to see if the handle manager has
    // an entry for ONAME.
    //
    spicelib::ZZDDHFNH(ONAME, &mut HANDLE, &mut FOUND, ctx)?;

    if FOUND {
        spicelib::SETMSG(b"Can not create file, \'#\'.  It is currently loaded and associated with handle, #.  This routine may only create files not currently loaded.", ctx);
        spicelib::ERRCH(b"#", ONAME, ctx);
        spicelib::ERRINT(b"#", HANDLE, ctx);
        spicelib::SIGERR(b"SPICE(CANTCREATEFILE)", ctx)?;
        spicelib::CHKOUT(b"T_BINGO", ctx)?;
        return Ok(());
    }

    //
    // Now, see if IFNAME is loaded in the handle manager.
    //
    spicelib::ZZDDHFNH(INAME, &mut HANDLE, &mut FOUND, ctx)?;

    //
    // If it is loaded, then use the value of IARC to indicate the
    // file architecture.
    //
    if FOUND {
        spicelib::ZZDDHNFO(
            HANDLE, &mut FNAME, &mut IARC, &mut IBFF, &mut IAMH, &mut FOUND, ctx,
        )?;

    //
    // Otherwise, open the file and extract the 8 character IDWORD.
    //
    } else {
        spicelib::GETLUN(&mut UNIT, ctx)?;

        {
            use f2rust_std::io;

            let specs = io::OpenSpecs {
                unit: Some(UNIT),
                file: Some(fstr::substr(INAME, 1..=spicelib::RTRIM(INAME))),
                access: Some(b"DIRECT"),
                recl: Some(RECL),
                status: Some(b"OLD"),
                ..Default::default()
            };
            IOSTAT = io::capture_iostat(|| ctx.open(specs))?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(b"Unable to open file, \'#\'. IOSTAT = #.", ctx);
            spicelib::ERRCH(b"#", INAME, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEOPENFAILED)", ctx)?;
            spicelib::CHKOUT(b"T_BINGO", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(1))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(&mut IDWORD)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            spicelib::SETMSG(
                b"Unable to read ID word from file, \'#\'.  IOSTAT = #.",
                ctx,
            );
            spicelib::ERRCH(b"#", INAME, ctx);
            spicelib::ERRINT(b"#", IOSTAT, ctx);
            spicelib::SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
            spicelib::CHKOUT(b"T_BINGO", ctx)?;
            return Ok(());
        }

        spicelib::IDW2AT(&IDWORD, &mut ARCH, &mut TYPE, ctx)?;

        if spicelib::EQSTR(&ARCH, b"DAF") {
            IARC = DAF;
        } else if spicelib::EQSTR(&ARCH, b"DAS") {
            IARC = DAS;
        } else {
            IARC = 0;
        }

        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UNIT),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    //
    // IARC is set properly, branch appropriately.
    //
    if (IARC == DAF) {
        T_BGODAF(INAME, ONAME, OBFF, ctx)?;
    } else if (IARC == DAS) {
        T_BGODAS(INAME, ONAME, OBFF, ctx)?;
    } else {
        spicelib::SETMSG(
            b"The architecture of file, \'#\', is unknown.  No conversion possible.",
            ctx,
        );
        spicelib::SIGERR(b"SPICE(UNKNOWNARCH)", ctx)?;
        spicelib::CHKOUT(b"T_BINGO", ctx)?;
        return Ok(());
    }

    spicelib::CHKOUT(b"T_BINGO", ctx)?;
    Ok(())
}
