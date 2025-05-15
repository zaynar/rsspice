//
// GENERATED FILE
//

use super::*;
use f2rust_std::*;

//$Procedure T_BGODAF ( BINGO: Process DAF files to alternate BFFs )
pub fn T_BGODAF(
    INAME: &[u8],
    ONAME: &[u8],
    OBFF: i32,
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut CREC = [b' '; 1024 as usize];
    let mut IDWORD = [b' '; 8 as usize];
    let mut IFNAME = [b' '; 60 as usize];
    let mut DPREC = StackArray::<f64, 128>::new(1..=128);
    let mut BWARD: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut I: i32 = 0;
    let mut INHAN: i32 = 0;
    let mut INUNIT: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NXTDSC: i32 = 0;
    let mut STPREC: i32 = 0;
    let mut UNIT: i32 = 0;
    let mut WORD: i32 = 0;
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
    } else {
        spicelib::CHKIN(b"T_BGODAF", ctx)?;
    }

    //
    // Open INAME for read access.
    //
    spicelib::DAFOPR(INAME, &mut INHAN, ctx)?;

    //
    // Create ONAME.
    //
    T_DAFOPN(ONAME, OBFF, &mut UNIT, ctx)?;

    //
    // Read INAME's file record, we need everything so use the very low
    // level reader.
    //
    spicelib::ZZDAFGFR(
        INHAN,
        &mut IDWORD,
        &mut ND,
        &mut NI,
        &mut IFNAME,
        &mut FWARD,
        &mut BWARD,
        &mut FREE,
        &mut FOUND,
        ctx,
    )?;

    if !FOUND {
        spicelib::DAFCLS(INHAN, ctx)?;
        testutil::KILFIL(ONAME, ctx)?;
        spicelib::SETMSG(b"Unable to find file record.", ctx);
        spicelib::SIGERR(b"SPICE(DAFREADFAILURE)", ctx)?;
        spicelib::CHKOUT(b"T_BGODAF", ctx)?;
        return Ok(());
    }

    //
    // Dump the file record into ONAME.  We add the FTP string
    // regardless... all new toolkits create files with the FTP
    // string.
    //
    T_DAFWFR(
        UNIT, OBFF, &IDWORD, ND, NI, &IFNAME, FWARD, BWARD, FREE, true, ctx,
    )?;

    //
    // Now just copy all the records between the file record and
    // FWARD.  This is valid at the moment for all supported
    // platforms, but may need to change in the future.
    //
    spicelib::ZZDDHHLU(INHAN, b"DAF", false, &mut INUNIT, ctx)?;

    {
        let m1__: i32 = 2;
        let m2__: i32 = (FWARD - 1);
        let m3__: i32 = 1;
        I = m1__;
        for _ in 0..((m2__ - m1__ + m3__) / m3__) as i32 {
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::UnformattedReader::new(ctx.io_unit(INUNIT)?, Some(I))?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut CREC)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                spicelib::DAFCLS(INHAN, ctx)?;
                testutil::KILFIL(ONAME, ctx)?;
                spicelib::SETMSG(b"Unable to read comment record, #.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(DAFREADFAILURE)", ctx)?;
                spicelib::CHKOUT(b"T_BGODAF", ctx)?;
                return Ok(());
            }

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(I))?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&CREC)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                spicelib::DAFCLS(INHAN, ctx)?;
                testutil::KILFIL(ONAME, ctx)?;
                spicelib::SETMSG(b"Unable to write comment record, #.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(DAFWRITEFAILURE)", ctx)?;
                spicelib::CHKOUT(b"T_BGODAF", ctx)?;
                return Ok(());
            }

            I += m3__;
        }
    }

    //
    // Now process the summary, name, and data records.  The record
    // FREE points to is the last record to be processed.
    //
    spicelib::DAFARW(FREE, &mut STPREC, &mut WORD, ctx)?;

    I = FWARD;
    NXTDSC = FWARD;

    while (I < STPREC) {
        //
        // See if the record we are currently processing is a
        // descriptor record.
        //
        if (I == NXTDSC) {
            spicelib::ZZDAFGSR(INHAN, I, ND, NI, DPREC.as_slice_mut(), &mut FOUND, ctx)?;

            if !FOUND {
                spicelib::DAFCLS(INHAN, ctx)?;
                testutil::KILFIL(ONAME, ctx)?;
                spicelib::SETMSG(b"Unable to read summary record, #.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(DAFREADFAILURE)", ctx)?;
                spicelib::CHKOUT(b"T_BGODAF", ctx)?;
                return Ok(());
            }

            NXTDSC = (DPREC[1] as i32);

            T_DAFWSR(
                UNIT,
                I,
                OBFF,
                ND,
                NI,
                (DPREC[1] as i32),
                (DPREC[2] as i32),
                (DPREC[3] as i32),
                DPREC.subarray(4),
                ctx,
            )?;

            //
            // Every summary record is followed by a name record...
            //
            I = (I + 1);

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Reader},
                };

                let mut reader = io::UnformattedReader::new(ctx.io_unit(INUNIT)?, Some(I))?;
                IOSTAT = io::capture_iostat(|| {
                    reader.start()?;
                    reader.read_str(&mut CREC)?;
                    reader.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                spicelib::DAFCLS(INHAN, ctx)?;
                testutil::KILFIL(ONAME, ctx)?;
                spicelib::SETMSG(b"Unable to read name record, #.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(DAFREADFAILURE)", ctx)?;
                spicelib::CHKOUT(b"T_BGODAF", ctx)?;
                return Ok(());
            }

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(I))?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&CREC)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                spicelib::DAFCLS(INHAN, ctx)?;
                testutil::KILFIL(ONAME, ctx)?;
                spicelib::SETMSG(b"Unable to write name record, #.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(DAFWRITEFAILURE)", ctx)?;
                spicelib::CHKOUT(b"T_BGODAF", ctx)?;
                return Ok(());
            }

        //
        // Otherwise we are dealing with a data record.
        //
        } else {
            spicelib::ZZDAFGDR(INHAN, I, DPREC.as_slice_mut(), &mut FOUND, ctx)?;

            if !FOUND {
                spicelib::DAFCLS(INHAN, ctx)?;
                testutil::KILFIL(ONAME, ctx)?;
                spicelib::SETMSG(b"Unable to read data record, #.", ctx);
                spicelib::ERRINT(b"#", I, ctx);
                spicelib::SIGERR(b"SPICE(DAFREADFAILURE)", ctx)?;
                spicelib::CHKOUT(b"T_BGODAF", ctx)?;
                return Ok(());
            }

            T_DAFWDR(UNIT, I, OBFF, 128, DPREC.as_slice(), ctx)?;
        }

        //
        // Next record.
        //
        I = (I + 1);
    }

    //
    // Process the last data record, only translate up to (FREE-1)'s
    // address.
    //
    spicelib::ZZDAFGDR(INHAN, STPREC, DPREC.as_slice_mut(), &mut FOUND, ctx)?;

    //
    // Do not worry if STPREC was not found, it is possible (but
    // very unlikely) that FREE points to a record that has yet
    // to be created.
    //
    if FOUND {
        T_DAFWDR(UNIT, I, OBFF, (WORD - 1), DPREC.as_slice(), ctx)?;
    }

    //
    // Clean up.
    //
    spicelib::DAFCLS(INHAN, ctx)?;

    //
    // Check FAILED() in case something has gone awry.  If it has delete
    // ONAME.
    //
    if spicelib::FAILED(ctx) {
        testutil::KILFIL(ONAME, ctx)?;
    } else {
        {
            use f2rust_std::io;

            let specs = io::CloseSpecs {
                unit: Some(UNIT),
                ..Default::default()
            };
            ctx.close(specs)?;
        }
    }

    spicelib::CHKOUT(b"T_BGODAF", ctx)?;
    Ok(())
}
