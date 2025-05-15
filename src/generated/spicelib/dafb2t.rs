//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const BSIZE: i32 = 100;
const IDWLEN: i32 = 8;
const QUOTE: &[u8] = b"\'";

/// DAF, binary to text
///
/// Deprecated: This routine has been superseded by the SPICELIB
/// routine DAFBT. NAIF supports this routine only to provide backward
/// compatibility.
///
/// Write the contents of a binary DAF to a text file opened by
/// the calling program.
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
///  BINARY     I   Name of an existing binary DAF.
///  TEXT       I   Logical unit connected to text file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BINARY   is the name of an existing binary DAF.
///
///  TEXT     is a logical unit number, to which a text file has
///           been connected by the calling program, and into
///           which the contents of BINARY are to be written
///           (in a form more suitable for transfer between
///           heterogeneous computing environments).
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If for some reason the text file cannot be written,
///      the error SPICE(DAFWRITEFAIL) is signaled.
///
///  2)  If for some reason the ID word cannot be read from the DAF
///      file, the error SPICE(DAFREADFAIL) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See arguments BINARY, TEXT.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine has been made obsolete by the new DAF binary to text
///  conversion routine DAFBT. This routine remains available for
///  reasons of backward compatibility. We strongly recommend that you
///  use the new conversion routines for any new software development.
///  Please see the header of the routine DAFBT for details.
///
///  Any binary DAF may be transferred between heterogeneous
///  Fortran environments by converting it to an equivalent file
///  containing only ASCII characters. Such a file can be transferred
///  almost universally, using any number of established protocols
///  (Kermit, FTP, and so on). Once transferred, the ASCII file can
///  be converted to a binary file, using the representations
///  native to the new host environment.
///
///  There are two pairs of routines that can be used to convert
///  DAFs between binary and text. The first pair, DAFB2A
///  and DAFA2B, works with complete files. That is, DAFB2A creates
///  a complete ASCII file containing all of the information in
///  a particular binary file, and nothing else; this file can
///  be fed directly into DAFA2B to produce a complete binary file.
///  In each case, the names of the files are specified.
///
///  A related pair of routines, DAFB2T and DAFT2B, assume that
///  the ASCII data are to be stored in the midst of a text file.
///  This allows the calling program to surround the data with
///  standardized labels, to append several binary files into a
///  single text file, and so on.
///
///  Note that the contents of reserved records in the binary file
///  are not written by this routine (although they may be stored
///  in the ASCII file by the calling program).
/// ```
///
/// # Examples
///
/// ```text
///  DAFB2A and DAFA2B are typically used for simple file transfers.
///  If file A.DAF is a binary DAF in environment 1, it can be
///  transferred to environment 2 in three steps.
///
///     1) Convert it to ASCII:
///
///           CALL DAFB2A ( 'A.DAF', 'A.ASCII' )
///
///     2) Transfer the ASCII file, using FTP, Kermit, or some other
///        file transfer utility:
///
///           ftp> put a.ascii
///
///     3) Convert it to binary on the new machine,
///
///           CALL DAFA2B ( 'A.ASCII', 'A.DAF', RESV )
///
///  Note that DAFB2A and DAFA2B work in any standard Fortran-77
///  environment.
///
///  If the file needs to contain other information---a standard
///  label, for instance---the first and third steps must be modified
///  to use DAFB2T and DAFT2B. The first step becomes
///
///     (Open a text file)
///     (Write the label)
///     CALL DAFB2T ( BINARY, UNIT  )
///     (Close the text file)
///
///  The third step becomes
///
///     (Open the text file)
///     (Read the label)
///     CALL DAFT2B ( UNIT, BINARY, RESV )
///     (Close the text file)
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
///  I.M. Underwood     (JPL)
///  E.D. Wright        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.1.0, 26-OCT-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard. Moved DAF
///         required reading from $Literature_References to
///         $Required_Reading section.
///
/// -    SPICELIB Version 3.0.1, 26-JUL-2012 (EDW)
///
///         Edited $Abstract section to use "Deprecated" keyword
///         and state replacement routine.
///
///         Eliminated unneeded $Revisions section.
///
/// -    SPICELIB Version 3.0.0, 16-NOV-2001 (FST)
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
///         Added the variable IDWORD to the routine for storing the ID
///         word from the file being converted. This replaces a hard coded
///         value of 'NAIF/DAF', and supports the new interpretation of the
///         ID word.
///
///         Removed the error SPICE(DAFNOIDWORD) as it was no longer
///         relevant.
///
///         There were no checks of the IOSTAT variable after attempting to
///         write to the text file, a single test of the IOSTAT variable
///         was made at the end of the routine. This was not adequate to
///         detect errors when writing to the text file. So after all of
///         these write statements, an IF ... END IF block was added to
///         signal an error if IOSTAT .NE. 0.
///
///            IF ( IOSTAT .NE. 0 ) THEN
///
///               CALL DAFCLS ( HANDLE                                )
///               CALL SETMSG ( 'The attempt to write to file ''#''' //
///         .                   ' failed. IOSTAT = #.'                )
///               CALL ERRFNM ( '#', TEXT                             )
///               CALL SIGERR ( SPICE(DAFWRITEFAIL)                 )
///               CALL CHKOUT ( 'DAFB2T'                              )
///               RETURN
///
///            END IF
///
///         Removed the code from the end of the routine that purported to
///         check for read errors:
///
///            C
///            C     If any write screws up, they should all screw up. Why
///            C     make a billion separate checks?
///            C
///                  IF ( IOSTAT .NE. 0 ) THEN
///                     CALL SETMSG ( 'Value of IOSTAT was: #. ' )
///                     CALL ERRINT ( '#', IOSTAT                )
///                     CALL SIGERR ( SPICE(DAFWRITEFAIL)      )
///                   END IF
///
///         The answer to the question is:
///
///            You have to do a billion separate checks because the IOSTAT
///            value is only valid for the most recently executed write.
///
///         Added the following error message to the routine:
///
///            C     2) If for some reason the ID word cannot be read from
///            C        the DAF file, the error SPICE(DAFREADFAIL) will be
///            C        signaled.
///
///         because the file ID word is now read from the binary DAF file
///         rather than being hard coded as 'NAIF/DAF' in this routine.
///
///         Added a statement to the $Particulars section to the effect
///         that this routine has been made obsolete by the introduction of
///         the routine DAFBT, and that we strongly recommend the use of
///         the new routine.
///
///         Modified the $Abstract section to reflect the fact that this
///         routine is obsolete.
///
/// -    SPICELIB Version 1.0.2, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.1, 22-MAR-1990 (HAN)
///
///         Literature references added to the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU)
/// ```
pub fn dafb2t(ctx: &mut SpiceContext, binary: &str, text: i32) -> crate::Result<()> {
    DAFB2T(binary.as_bytes(), text, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DAFB2T ( DAF, binary to text )
pub fn DAFB2T(BINARY: &[u8], TEXT: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; 60 as usize];
    let mut NAME = [b' '; 1000 as usize];
    let mut BUFFER = StackArray::<f64, 100>::new(1..=BSIZE);
    let mut DC = StackArray::<f64, 125>::new(1..=125);
    let mut SUM = StackArray::<f64, 125>::new(1..=125);
    let mut BEGIN: i32 = 0;
    let mut BWARD: i32 = 0;
    let mut CHUNK: i32 = 0;
    let mut CSIZE: i32 = 0;
    let mut DAFLUN: i32 = 0;
    let mut END: i32 = 0;
    let mut FREE: i32 = 0;
    let mut FWARD: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut IC = StackArray::<i32, 250>::new(1..=250);
    let mut IOSTAT: i32 = 0;
    let mut ISIZE: i32 = 0;
    let mut LSIZE: i32 = 0;
    let mut ND: i32 = 0;
    let mut NI: i32 = 0;
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
        CHKIN(b"DAFB2T", ctx)?;
    }
    //
    // Initialize the IDWORD.
    //
    fstr::assign(&mut IDWORD, b" ");

    //
    // Open the binary file for reading and read the ID word from the
    // first record of the file.
    //
    DAFOPR(BINARY, &mut HANDLE, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }
    //
    // At this point, we know that we have a DAF file, because we were
    // able to successfully open it, so we will attempt to proceed with
    // the file conversion process.
    //
    // Convert the DAF file handle to its equivalent Fortran logical
    // unit. We need to do this in order to accurately move the file
    // ID word to the text file.
    //
    ZZDDHHLU(HANDLE, b"DAF", false, &mut DAFLUN, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Reader},
        };

        let mut reader = io::UnformattedReader::new(ctx.io_unit(DAFLUN)?, Some(1))?;
        IOSTAT = io::capture_iostat(|| {
            reader.start()?;
            reader.read_str(&mut IDWORD)?;
            reader.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        SETMSG(b"Could not read ID word from file \'#\'. IOSTAT = #.", ctx);
        ERRCH(b"#", BINARY, ctx);
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFREADFAIL)", ctx)?;
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }
    //
    // Get the contents of the file record. The ASCII file begins
    // with the ID word which is followed by the summary format,
    // which is followed by the internal file name.
    //
    DAFRFR(
        HANDLE,
        &mut ND,
        &mut NI,
        &mut IFNAME,
        &mut FWARD,
        &mut BWARD,
        &mut FREE,
        ctx,
    )?;

    if FAILED(ctx) {
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&fstr::concat(&fstr::concat(QUOTE, &IDWORD), QUOTE))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        DAFCLS(HANDLE, ctx)?;
        SETMSG(
            b"The attempt to write to file \'#\' failed. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", TEXT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_i32(ND)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        DAFCLS(HANDLE, ctx)?;
        SETMSG(
            b"The attempt to write to file \'#\' failed. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", TEXT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_i32(NI)?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        DAFCLS(HANDLE, ctx)?;
        SETMSG(
            b"The attempt to write to file \'#\' failed. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", TEXT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&fstr::concat(&fstr::concat(QUOTE, &IFNAME), QUOTE))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        DAFCLS(HANDLE, ctx)?;
        SETMSG(
            b"The attempt to write to file \'#\' failed. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", TEXT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }
    //
    // Each array is preceded by a '1', which indicates that more
    // arrays are to come. The array itself begins with the name
    // and the summary components, and ends with the name again.
    // The elements are written in arbitrary chunks. The final
    // chunk is followed by a '0', which indicates that no chunks
    // remain.
    //
    // Write the arrays in forward order.
    //
    LSIZE = ((ND + ((NI - 1) / 2)) + 1);
    ISIZE = (LSIZE * 8);

    DAFBFS(HANDLE, ctx)?;
    DAFFNA(&mut FOUND, ctx)?;

    if FAILED(ctx) {
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    while FOUND {
        DAFGS(SUM.as_slice_mut(), ctx)?;
        DAFGN(&mut NAME, ctx)?;
        DAFUS(SUM.as_slice(), ND, NI, DC.as_slice_mut(), IC.as_slice_mut());

        if FAILED(ctx) {
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(b"1")?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            DAFCLS(HANDLE, ctx)?;
            SETMSG(
                b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", TEXT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&fstr::concat(
                    &fstr::concat(QUOTE, fstr::substr(&NAME, 1..=ISIZE)),
                    QUOTE,
                ))?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            DAFCLS(HANDLE, ctx)?;
            SETMSG(
                b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", TEXT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                for I in intrinsics::range(1, ND, 1) {
                    writer.write_f64(DC[I])?;
                }
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            DAFCLS(HANDLE, ctx)?;
            SETMSG(
                b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", TEXT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                for I in intrinsics::range(1, (NI - 2), 1) {
                    writer.write_i32(IC[I])?;
                }
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            DAFCLS(HANDLE, ctx)?;
            SETMSG(
                b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", TEXT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }

        BEGIN = IC[(NI - 1)];
        END = IC[NI];

        while (BEGIN <= END) {
            CHUNK = intrinsics::MIN0(&[((BEGIN + BSIZE) - 1), END]);
            CSIZE = ((CHUNK - BEGIN) + 1);

            DAFGDA(HANDLE, BEGIN, CHUNK, BUFFER.as_slice_mut(), ctx)?;

            if FAILED(ctx) {
                CHKOUT(b"DAFB2T", ctx)?;
                return Ok(());
            }

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_i32(CSIZE)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                DAFCLS(HANDLE, ctx)?;
                SETMSG(
                    b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", TEXT, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
                CHKOUT(b"DAFB2T", ctx)?;
                return Ok(());
            }

            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    for I in intrinsics::range(1, CSIZE, 1) {
                        writer.write_f64(BUFFER[I])?;
                    }
                    writer.finish()?;
                    Ok(())
                })?;
            }

            if (IOSTAT != 0) {
                DAFCLS(HANDLE, ctx)?;
                SETMSG(
                    b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", TEXT, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
                CHKOUT(b"DAFB2T", ctx)?;
                return Ok(());
            }

            BEGIN = (BEGIN + BSIZE);
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(b"0")?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            DAFCLS(HANDLE, ctx)?;
            SETMSG(
                b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", TEXT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }

        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(&fstr::concat(
                    &fstr::concat(QUOTE, fstr::substr(&NAME, 1..=ISIZE)),
                    QUOTE,
                ))?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            DAFCLS(HANDLE, ctx)?;
            SETMSG(
                b"The attempt to write to file \'#\' failed. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", TEXT, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }

        DAFFNA(&mut FOUND, ctx)?;

        if FAILED(ctx) {
            CHKOUT(b"DAFB2T", ctx)?;
            return Ok(());
        }
    }

    //
    // A final '0' indicates that no arrays remain. The first shall be
    // last: the internal file name brings up the rear.
    //
    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(b"0")?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        DAFCLS(HANDLE, ctx)?;
        SETMSG(
            b"The attempt to write to file \'#\' failed. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", TEXT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    {
        use f2rust_std::{
            data::Val,
            io::{self, Writer},
        };

        let mut writer = io::ListDirectedWriter::new(ctx.io_unit(TEXT)?, None)?;
        IOSTAT = io::capture_iostat(|| {
            writer.start()?;
            writer.write_str(&fstr::concat(&fstr::concat(QUOTE, &IFNAME), QUOTE))?;
            writer.finish()?;
            Ok(())
        })?;
    }

    if (IOSTAT != 0) {
        DAFCLS(HANDLE, ctx)?;
        SETMSG(
            b"The attempt to write to file \'#\' failed. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", TEXT, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(DAFWRITEFAIL)", ctx)?;
        CHKOUT(b"DAFB2T", ctx)?;
        return Ok(());
    }

    //
    // Close only the binary file.
    //
    DAFCLS(HANDLE, ctx)?;

    CHKOUT(b"DAFB2T", ctx)?;
    Ok(())
}
