//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const QUOTE: &[u8] = b"\'";
const BEGARR: &[u8] = b"BEGIN_ARRAY";
const ENDARR: &[u8] = b"END_ARRAY";
const TOTARR: &[u8] = b"TOTAL_ARRAYS";
const FTYPID: &[u8] = b"DAFETF";
const INFOLN: &[u8] = b"NAIF DAF ENCODED TRANSFER FILE";
const BUFSIZ: i32 = 1024;
const CRECL: i32 = 1000;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const LINLEN: i32 = 80;

/// DAF, convert binary file to transfer file
///
/// Convert the contents of a binary DAF file to an equivalent DAF
/// transfer file.
///
/// # Required Reading
///
/// * [DAF](crate::required_reading::daf)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BINFIL     I   The name of a binary DAF file to be converted.
///  XFRLUN     I   Logical unit of a previously opened file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BINFIL   is the name of a binary DAF file which is to be converted
///           to an equivalent DAF transfer file.
///
///  XFRLUN   is the Fortran logical unit number of a previously opened
///           file. The DAF transfer file will be written to the
///           file attached to this logical unit beginning at the
///           current position in the file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the binary DAF file specified by the filename BINFIL cannot
///      be opened for read access, an error is signaled by a routine
///      in the call tree of this routine.
///
///  2)  If for some reason the DAF transfer file cannot be written
///      to, the error SPICE(FILEWRITEFAILED) is signaled.
///
///  3)  If, for any reason, the DAF file cannot be read, an error is
///      signaled by a routine in the call tree of this routine.
///
///  4)  If the ID word cannot be read from the binary file, the error
///      SPICE(FILEREADFAILED) is signaled.
///
///  5)  The binary DAF file opened by this routine, BINFIL, is only
///      GUARANTEED to be closed upon successful completion of the
///      conversion process. In the event of an error, the caller of
///      this routine is required to close the binary DAF file BINFIL.
/// ```
///
/// # Files
///
/// ```text
///  See arguments BINFIL, XFRLUN.
/// ```
///
/// # Particulars
///
/// ```text
///  Any binary DAF file may be transferred between heterogeneous
///  Fortran environments by converting it to an equivalent file
///  containing only ASCII characters. Such a file can be transferred
///  almost universally, using any number of established protocols.
///  Once transferred, the ASCII file can be converted to a binary
///  file, using the representations native to the new host
///  environment.
///
///  This routine provides a mechanism for converting a binary DAF
///  file into an equivalent encoded ASCII file called a DAF transfer
///  file. It is one of a pair of routines for performing conversions
///  between the binary format of a DAF file and the DAF transfer file.
///  The inverse of this routine is the routine DAFTB.
///
///  The contents of the reserved records in a binary DAF file are
///  ignored by this routine. They are not written to the DAF transfer
///  file. The reserved records must be dealt with separately from the
///  data in a DAF file.
///
///  Upon successful completion, the DAF transfer file attached to
///  Fortran logical unit XFRLUN will contain the same data as the
///  binary DAF file BINFIL. The binary DAF file BINFIL will be closed
///  when this routine exits. The DAF transfer file will remain open,
///  as it was on entry, and it will be positioned to write on the
///  first line following the encoded DAF data.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///     BINFIL   be the name of a binary DAF file which is to be
///              converted to an equivalent DAF transfer file.
///
///     XFRLUN   be the Fortran logical unit to which the DAF transfer
///              file is to be written.
///
///  The following subroutine call would read the binary DAF
///  file with the name BINFIL, convert its data into an encoded
///  format, and write that data to the DAF transfer file attached
///  to the Fortran logical unit XFRLUN, beginning at the current
///  position in the file.
///
///     CALL DAFBT( BINFIL, XFRLUN )
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 4.1.0, 13-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 4.0.0, 16-NOV-2001 (FST)
///
///         Updated the routine to utilize the new handle manager
///         interfaces.
///
/// -    SPICELIB Version 3.0.0, 25-JAN-1995 (KRG)
///
///         Updated the header and in line comments to reflect the change
///         from calling files text files to calling them transfer files.
///
///         Changed the variable name TXTLUN to XFRLUN to make it
///         compatible with the change in terminology.
///
/// -    SPICELIB Version 2.0.0, 04-OCT-1993 (KRG)
///
///         No changes to this routine were necessary to incorporate the
///         new file ID word format. This routine already read and copied
///         the ID word to the text file being created.
///
///         Also, all list directed writes in this routine were replaced by
///         formatted writes with FMT = '(A)'. This routine only writes
///         character data.
///
///         Added a test of FAILED() after the call to DAFHLU for
///         completeness.
///
/// -    SPICELIB Version 1.0.1, 24-JUN-1993 (KRG)
///
///         Modified the description of the DAF encoded text file format
///         appearing before the program code.
///
/// -    SPICELIB Version 1.0.0, 29-OCT-1992 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -    SPICELIB Version 4.0.0, 16-NOV-2001 (FST)
///
///         This routine still uses a naked READ to retrieve the
///         file IDWORD from the first 8 characters stored in the
///         file record. It may be that future environments
///         will have characters whose storage exceeds 1 byte,
///         in which case this routine will require modification.
///         One possibility is to call the private file record
///         reader ZZDAFGFR, which must address the translation
///         for all supported non-native binary file formats on this
///         platform.
///
///         The existing call to DAFHLU was replaced with ZZDDHHLU.
///         The call to DAFRDA was replaced with a call to the new,
///         translation-aware routine DAFGDA.
///
/// -    SPICELIB Version 2.0.0, 04-OCT-1993 (KRG)
///
///         No changes to this routine were necessary to incorporate the
///         new file ID word format. This routine already read and copied
///         the ID word to the text file being created.
///
///         Also, all list directed writes in this routine were replaced by
///         formatted writes with FMT = '(A)'. This routine only writes
///         character data.
///
///         Added a test of FAILED() after the call to DAFHLU for
///         completeness.
///
/// -    SPICELIB Version 1.0.1, 24-JUN-1993 (KRG)
///
///         Modified the description of the DAF encoded text file format
///         appearing before the program code. Changed the line:
///
///            C        < DAF ND value > < DAF NI value >
///
///         to the lines:
///
///            C        < DAF ND value >
///            C        < DAF NI value >
///
///         This change was necessary because the output format for the
///         low level routines which encode and write the data were
///         modified to fix a problem. See the routines WRENCD and WRENCI
///         for details of the modification.
/// ```
pub fn dafbt(ctx: &mut SpiceContext, binfil: &str, xfrlun: i32) -> crate::Result<()> {
    DAFBT(binfil.as_bytes(), xfrlun, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFBT ( DAF, convert binary file to transfer file )
pub fn DAFBT(BINFIL: &[u8], XFRLUN: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut LINE = [b' '; LINLEN as usize];
    let mut NAME = [b' '; CRECL as usize];
    let mut BUFFER = ActualArray::<f64>::new(1..=BUFSIZ);
    let mut DSUMRY = StackArray::<f64, 125>::new(1..=125);
    let mut SUMMRY = StackArray::<f64, 125>::new(1..=125);
    let mut BINHDL: i32 = 0;
    let mut BINLUN: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut DTABEG: i32 = 0;
    let mut DTACNT: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut IOSTAT: i32 = 0;
    let mut ISUMRY = StackArray::<i32, 250>::new(1..=250);
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
    let mut NUMARR: i32 = 0;
    let mut NUMDTA: i32 = 0;
    let mut NUMLFT: i32 = 0;
    let mut SNMLEN: i32 = 0;
    let mut FOUND: bool = false;

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
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"DAFBT", ctx)?;
    }
    //
    //  A brief description of the DAF transfer file format and its
    //  intended use follows. This description is intended to provide a
    //  simple ``picture'' of the DAF transfer file format to aid in the
    //  understanding of this routine. This description is NOT intended to
    //  be a detailed specification of the file format.
    //
    //  A DAF transfer file contains all of the data from a binary
    //  DAF file, except for the reserved record area, in an encoded
    //  ASCII format. The file also contains some bookkeeping information
    //  for maintaining the integrity of the data. The DAF transfer file
    //  format allows the full precision of both integer and floating
    //  point numeric data to be maintained in a portable fashion. The DAF
    //  transfer file format is intended to provide a reliable and
    //  accurate means for porting data among multiple computer systems
    //  and for the archival storage of data.
    //
    //  A DAF transfer file is not intended to be used directly to
    //  provide data to a program, the equivalent binary DAF file is
    //  to be used for this purpose. In no way should any program, other
    //  than a DAF binary <-> transfer conversion program, rely on the DAF
    //  encoded transfer file format.
    //
    //  To correctly understand the DAF transfer file description
    //  the reader should be familiar with the DAF file architecture.
    //  Items enclosed in angle brackets, '<' and '>', are used to
    //  represent the data which is to be placed at that position in
    //  the file. The bookkeeping information is represented exactly
    //  as it would appear in a DAF transfer file.
    //
    //  Let
    //
    //    BOF    denote the beginning of the file
    //    EOF    denote the end of the file
    //
    // and
    //
    //    n      denote the total number of arrays in a DAF file
    //    NA(i)  denote the number of double precision numbers in array i
    //    m(i)   denote the number of blocks of encoded data for array i
    //    N(i,j) denote the number of encoded double precision numbers
    //           in block j of array i
    //
    //  and
    //
    //           m(i)
    //          -----
    //          \
    //           >   N(i,k) = NA(i),   i = 1, ..., n.
    //          /
    //          -----
    //           k=1
    //
    //  A DAF encoded transfer file has the following format:
    //
    //     <BOF>
    //     < Information line >
    //     < DAF file ID word >
    //     < DAF ND value >
    //     < DAF NI value >
    //     < DAF internal file name >
    //     BEGIN_ARRAY 1 NA(1)
    //     < Name for array 1 >
    //     < ND double precision summary values >
    //     < NI-2 integer summary values >
    //     N(1,1)
    //     < N(1,1) Encoded double precision numbers >
    //     N(1,2)
    //     < N(1,2) Encoded double precision numbers >
    //                       .
    //                       .
    //                       .
    //     N(1,m(1))
    //     < N(1,m(1)) Encoded double precision numbers >
    //     END_ARRAY 1 NA(1)
    //     BEGIN_ARRAY 2 NA(2)
    //     < Name for array 2 >
    //     < ND double precision summary values >
    //     < NI-2 integer summary values >
    //     N(2,1)
    //     < N(2,1) Encoded double precision numbers >
    //     N(2,2)
    //     < N(2,2) Encoded double precision numbers >
    //                       .
    //                       .
    //                       .
    //     N(2,m(2))
    //     < N(2,m(2)) Encoded double precision numbers >
    //     END_ARRAY 2 NA(2)
    //                       .
    //                       .
    //                       .
    //     BEGIN_ARRAY n NA(n)
    //     < Name for array n >
    //     < ND double precision summary values >
    //     < NI-2 integer summary values >
    //     N(n,1)
    //     < N(n,1) Encoded double precision numbers >
    //     N(n,2)
    //     < N(n,2) Encoded double precision numbers >
    //                       .
    //                       .
    //                       .
    //     N(n,m(n))
    //     < N(n,m(n)) Encoded double precision numbers >
    //     END_ARRAY n NA(n)
    //     TOTAL_ARRAYS  n
    //     <EOF>
    //
    //  This routine will check the SPICELIB function FAILED() after
    //  each call, or consecutive sequence of calls, to data encoding
    //  routines, and if an error was signaled it will simply check out
    //  and return to the caller.
    //
    //  This routine will check the SPICELIB function FAILED() after
    //  each DAF file access call, and if an error was signaled it will
    //  simply check out and return to the caller.
    //
    //  We begin by opening the binary DAF file specified by BINFIL for
    //  read access, obtaining a DAF file handle.
    //
    DAFOPR(BINFIL, &mut BINHDL, ctx)?;
    //
    // If the open failed, check out and return, as an appropriate error
    // message should have already been set.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // At this point, we know that we have a DAF file, because we were
    // able to successfully open it, so we will attempt to proceed with
    // the file conversion process.
    //
    // Convert the DAF file handle to its equivalent Fortran logical
    // unit. We need to do this in order to accurately move the file
    // ID word to the DAF transfer file.
    //
    ZZDDHHLU(BINHDL, b"DAF", false, &mut BINLUN, ctx)?;

    //
    // If the translation failed, checkout and return, as an appropriate
    // error message should have already been set.
    //
    if FAILED(ctx) {
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Read the ID word from the binary file. It should be the first 8
    // characters on the first record in the file.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(BINLUN)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error reading the file ID word from the binary DAF file \'#\'. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", BINLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEREADFAILED)", ctx)?;
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Get the contents of the file record: the number of double
    // precision numbers in the summary (ND), the number of integers
    // in the summary (NI), the internal filename (IFNAME), and some
    // data pointer information (FWARD, BWARD, FREE).
    //
    DAFRFR(
        BINHDL,
        &mut ND,
        &mut NI,
        &mut IFNAME,
        &mut FWARD,
        &mut BWARD,
        &mut FREE,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Write the information line containing the file type information
    // for the DAF transfer file format to the current position in the
    // DAF transfer file. The file type information must be the first
    // ``word'' on the information line. The rest of the line may be used
    // for other purposes. Right now, it simply contains an expanded
    // description of the file type information ``word.''
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&fstr::concat(&fstr::concat(FTYPID, b" "), INFOLN))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error writing to the DAF transfer file \'#\'.IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Write the ID word to the DAF transfer file.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&fstr::concat(&fstr::concat(QUOTE, &IDWORD), QUOTE))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error writing to the DAF transfer file \'#\'. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Write out the ND and NI values for the DAF file architecture.
    //
    ISUMRY[1] = ND;
    ISUMRY[2] = NI;

    WRENCI(XFRLUN, 2, ISUMRY.as_slice(), ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Write out the internal file name.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&fstr::concat(&fstr::concat(QUOTE, &IFNAME), QUOTE))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error writing to the DAF transfer file \'#\'. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Calculate the length of the segment names.
    //
    SNMLEN = (8 * (ND + ((NI + 1) / 2)));
    //
    // Get ready to begin a forward search through the DAF file for the
    // data.
    //
    DAFBFS(BINHDL, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Initialize the number of arrays processed to zero.
    //
    NUMARR = 0;
    //
    // We'll assume that we will find some data, until proven otherwise.
    //
    FOUND = true;
    //
    // Begin looking for and processing the arrays in the binary DAF
    // file.
    //
    while FOUND {
        //
        // Look for a DAF array.
        //
        DAFFNA(&mut FOUND, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DAFBT", ctx)?;
            return Ok(());
        }
        //
        // If we found an array, then we need to process it. Start
        // by incrementing the number of arrays processed. If not,
        // we just skip to the bottom of the loop.
        //
        if FOUND {
            NUMARR = (NUMARR + 1);
            //
            // Get and unpack the summary information for the current
            // array.
            //
            DAFGS(SUMMRY.as_slice_mut(), ctx)?;
            DAFUS(
                SUMMRY.as_slice(),
                ND,
                NI,
                DSUMRY.as_slice_mut(),
                ISUMRY.as_slice_mut(),
            );
            //
            // Get the name of the current array.
            //
            DAFGN(&mut NAME, ctx)?;

            if FAILED(ctx) {
                //
                // If an error occurred on any of the DAF system calls
                // above, return to the caller. An appropriate error
                // message will have already been set by the routine which
                // signaled the error.
                //
                CHKOUT(b"DAFBT", ctx)?;
                return Ok(());
            }
            //
            // Get the beginning address for the data in the current array.
            //
            DTABEG = ISUMRY[(NI - 1)];
            //
            // Set the number of double precision numbers in the current
            // array.
            //
            DTACNT = ((ISUMRY[NI] - ISUMRY[(NI - 1)]) + 1);

            fstr::assign(
                &mut LINE,
                &fstr::concat(&fstr::concat(BEGARR, b" #"), b" #"),
            );
            REPMI(&LINE.clone(), b"#", NUMARR, &mut LINE, ctx);
            REPMI(&LINE.clone(), b"#", DTACNT, &mut LINE, ctx);
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=RTRIM(&LINE)))?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(
                    b"Error writing to the DAF transfer file \'#\'. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DAFBT", ctx)?;
                return Ok(());
            }
            //
            // Write the name of the current array.
            //
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(&fstr::concat(
                        &fstr::concat(QUOTE, fstr::substr(&NAME, 1..=SNMLEN)),
                        QUOTE,
                    ))?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(
                    b"Error writing to the DAF transfer file \'#\'. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DAFBT", ctx)?;
                return Ok(());
            }
            //
            // Write out the double precision part of the summary.
            //
            WRENCD(XFRLUN, ND, DSUMRY.as_slice(), ctx)?;
            //
            // Write out the integer part of the summary, excluding the
            // beginning and ending addresses of the data in the array,
            // ISUMRY(NI-1) and ISUMRY(NI), since these values vary with
            // the number of reserved records allocated.
            //
            WRENCI(XFRLUN, (NI - 2), ISUMRY.as_slice(), ctx)?;

            if FAILED(ctx) {
                //
                // If an error occurred on any of the data encoding calls
                // above, return to the caller. An appropriate error message
                // will have already been set by the routine which signaled
                // the error.
                //
                CHKOUT(b"DAFBT", ctx)?;
                return Ok(());
            }

            NUMLFT = DTACNT;

            while (NUMLFT > 0) {
                if (NUMLFT >= BUFSIZ) {
                    NUMDTA = BUFSIZ;
                } else {
                    NUMDTA = NUMLFT;
                }
                //
                // Read in NUMDTA numbers from the current array. The
                // desired data are specified by beginning and ending
                // indices into the array, inclusive: thus the subtraction
                // of 1 in the call.
                //
                DAFGDA(
                    BINHDL,
                    DTABEG,
                    ((DTABEG + NUMDTA) - 1),
                    BUFFER.as_slice_mut(),
                    ctx,
                )?;

                if FAILED(ctx) {
                    //
                    // We want to check failed here because were in a loop.
                    // We should exit the loop, and the routine, as soon as
                    // an error is detected, so we don't continue doing
                    // things for a long time.
                    //
                    CHKOUT(b"DAFBT", ctx)?;
                    return Ok(());
                }
                //
                // Write out the count of double precision numbers which are
                // in the buffer.
                //
                fstr::assign(&mut LINE, b"#");
                REPMI(&LINE.clone(), b"#", NUMDTA, &mut LINE, ctx);
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_str(fstr::substr(&LINE, 1..=RTRIM(&LINE)))?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }

                if (IOSTAT != 0) {
                    SETMSG(
                        b"Error writing to the DAF transfer file \'#\'. IOSTAT = #.",
                        ctx,
                    );
                    ERRFNM(b"#", XFRLUN, ctx)?;
                    ERRINT(b"#", IOSTAT, ctx);
                    SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                    CHKOUT(b"DAFBT", ctx)?;
                    return Ok(());
                }
                //
                // Encode and write out a buffer of double precision
                // numbers.
                //
                WRENCD(XFRLUN, NUMDTA, BUFFER.as_slice(), ctx)?;

                if FAILED(ctx) {
                    //
                    // We want to check failed here because were in a loop.
                    // We should exit the loop, and the routine, as soon as
                    // an error is detected, so we don't continue doing
                    // things for a long time.
                    //
                    CHKOUT(b"DAFBT", ctx)?;
                    return Ok(());
                }

                NUMLFT = (NUMLFT - NUMDTA);
                DTABEG = (DTABEG + NUMDTA);
            }

            fstr::assign(
                &mut LINE,
                &fstr::concat(&fstr::concat(ENDARR, b" #"), b" #"),
            );
            REPMI(&LINE.clone(), b"#", NUMARR, &mut LINE, ctx);
            REPMI(&LINE.clone(), b"#", DTACNT, &mut LINE, ctx);
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=RTRIM(&LINE)))?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                SETMSG(
                    b"Error writing to the DAF transfer file \'#\'. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DAFBT", ctx)?;
                return Ok(());
            }
        }
        //
        // At this point, one complete DAF array has been written to the
        // DAF transfer file.
        //
    }
    //
    // Write out the number of arrays processed.
    //
    fstr::assign(&mut LINE, &fstr::concat(TOTARR, b" #"));
    REPMI(&LINE.clone(), b"#", NUMARR, &mut LINE, ctx);
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::FormattedWriter::new(ctx.io_unit(XFRLUN)?, None, b"(A)")?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(fstr::substr(&LINE, 1..=RTRIM(&LINE)))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(
            b"Error writing to the DAF transfer file \'#\'. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DAFBT", ctx)?;
        return Ok(());
    }
    //
    // Close only the binary file.
    //
    DAFCLS(BINHDL, ctx)?;

    CHKOUT(b"DAFBT", ctx)?;
    Ok(())
}
