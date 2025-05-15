//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const QUOTE: &[u8] = b"\'";
const BEGCOM: &[u8] = b"BEGIN_COMMENT_BLOCK";
const ENDCOM: &[u8] = b"END_COMMENT_BLOCK";
const TCMBLK: &[u8] = b"TOTAL_COMMENT_BLOCKS";
const BCBLK: &[u8] = b"BEGIN_CHARACTER_BLOCK";
const ECBLK: &[u8] = b"END_CHARACTER_BLOCK";
const TCBLKS: &[u8] = b"TOTAL_CHARACTER_BLOCKS";
const BDBLK: &[u8] = b"BEGIN_DP_BLOCK";
const EDBLK: &[u8] = b"END_DP_BLOCK";
const TDBLKS: &[u8] = b"TOTAL_DP_BLOCKS";
const BIBLK: &[u8] = b"BEGIN_INTEGER_BLOCK";
const EIBLK: &[u8] = b"END_INTEGER_BLOCK";
const TIBLKS: &[u8] = b"TOTAL_INTEGER_BLOCKS";
const FTYPID: &[u8] = b"DASETF";
const INFOLN: &[u8] = b"NAIF DAS ENCODED TRANSFER FILE";
const LINLEN: i32 = 80;
const CBFLEN: i32 = 4;
const IDWLEN: i32 = 8;
const IFNLEN: i32 = 60;
const CRLEN: i32 = 1024;
const BUFSIZ: i32 = 1024;
const BCBPOS: i32 = 1;
const ECBPOS: i32 = CBFLEN;

/// DAS, convert binary file to transfer file
///
/// Convert the contents of a binary DAS file to an equivalent DAS
/// transfer file.
///
/// # Required Reading
///
/// * [DAS](crate::required_reading::das)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BINFIL     I   Name of the binary DAS file to be converted.
///  XFRLUN     I   Logical unit of a previously opened file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BINFIL   is the name of a binary DAS file which is to be converted
///           to an equivalent DAS transfer file.
///
///  XFRLUN   is the Fortran logical unit number of a previously opened
///           file. The DAS transfer file will be written to the
///           file attached to this logical unit beginning at the
///           current position in the file.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the binary DAS file specified by the filename BINFIL cannot
///      be opened for read access, an error is signaled by a routine
///      in the call tree of this routine.
///
///  2)  If for some reason the DAS transfer file cannot be written
///      to, the error SPICE(FILEWRITEFAILED) is signaled.
///
///  3)  If, for any reason, the DAS file cannot be read, an error is
///      signaled by a routine in the call tree of this routine.
///
///  4)  The binary DAS file opened by this routine, BINFIL, is only
///      GUARANTEED to be closed upon successful completion of the
///      binary to transfer conversion process. In the event of an
///      error, the caller of this routine is required to close the
///      binary DAS file BINFIL.
///
///  5)  If the values for the number of reserved records or the
///      number of reserved characters in a DAS file is nonzero,
///      the error SPICE(BADDASFILE) is signaled. THIS ERROR
///      IS SIGNALED ONLY BECAUSE THE RESERVED RECORD AREA HAS
///      NOT YET BEEN IMPLEMENTED.
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
///  Any binary DAS file may be transferred between heterogeneous
///  Fortran environments by converting it to an equivalent file
///  containing only ASCII characters called a DAS transfer file.
///  Such a file can be transferred almost universally using any number
///  of established protocols. Once transferred, the DAS transfer file
///  can be converted to a binary file using the representations native
///  to the new host environment.
///
///  This routine provides a mechanism for converting a binary DAS
///  file into an equivalent DAS transfer file. It is one of a pair of
///  routines for performing conversions between the binary format of a
///  DAS file and the DAS transfer file. The inverse of this routine is
///  the routine DASTB.
///
///  Upon successful completion, the DAS transfer file attached to
///  Fortran logical unit XFRLUN will contain the same data as the
///  binary DAS file BINFIL in an encoded ASCII format. The binary DAS
///  file BINFIL will be closed when this routine exits successfully.
///  The DAS transfer file will remain open, as it was on entry, and it
///  will be positioned to write on the first line following the
///  encoded data from the binary DAS file.
/// ```
///
/// # Examples
///
/// ```text
///  Let
///
///     BINFIL   be the name of a binary DAS file which is to be
///              converted to an equivalent DAS transfer file. This
///              could be for purposes of porting the data to a
///              different computer platform, or possibly for
///              archival storage of the data.
///
///     XFRLUN   be the Fortran logical unit to which the DAS transfer
///              file is to be written.
///
///  Then, the following subroutine call would read the binary DAS
///  file BINFIL, convert its contents into an encoded format, and
///  then write that data to the DAS transfer file attached to XFRLUN,
///  beginning at the current position in that file.
///
///     CALL DASBT ( BINFIL, XFRLUN )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.2.0, 02-JUN-2021 (JDR)
///
///         Added IMPLICIT NONE standard.
///
///         Edited the header to comply with NAIF standard. Removed
///         unnecessary $Revisions section.
///
/// -    SPICELIB Version 3.1.0, 05-FEB-1995 (NJB)
///
///         Updated to support integration with the handle manager
///         subsystem.
///
/// -    SPICELIB Version 3.0.0, 13-AUG-1994 (KRG)
///
///         Updated the header and in line comments to reflect the change
///         from calling files text files to calling them transfer files.
///
///         Changed the variable name TXTLUN to XFRLUN to make it
///         compatible with the change in terminology.
///
/// -    SPICELIB Version 2.0.0, 13-AUG-1994 (KRG)
///
///         A potential problem with list directed writes was fixed. Some
///         compilers have list directed writes that write multiple comma
///         separated items to one line and other compilers write these to
///         multiple lines even when all of the output will fit on a single
///         line. This was fixed by replacing all of the affected list
///         directed write statements with code to put the desired data
///         into a character string and then write the character string.
///
/// -    SPICELIB Version 1.0.0, 29-OCT-1992 (KRG)
/// ```
pub fn dasbt(ctx: &mut SpiceContext, binfil: &str, xfrlun: i32) -> crate::Result<()> {
    DASBT(binfil.as_bytes(), xfrlun, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASBT ( DAS, convert binary file to transfer file )
pub fn DASBT(BINFIL: &[u8], XFRLUN: i32, ctx: &mut Context) -> f2rust_std::Result<()> {
    let mut CBUFFR = ActualCharArray::new(CBFLEN, 1..=BUFSIZ);
    let mut CRECRD = [b' '; CRLEN as usize];
    let mut IDWORD = [b' '; IDWLEN as usize];
    let mut IFNAME = [b' '; IFNLEN as usize];
    let mut LINE = [b' '; LINLEN as usize];
    let mut DBUFFR = ActualArray::<f64>::new(1..=BUFSIZ);
    let mut DASLUN: i32 = 0;
    let mut DTABEG: i32 = 0;
    let mut HANDLE: i32 = 0;
    let mut IBUFFR = ActualArray::<i32>::new(1..=BUFSIZ);
    let mut IOSTAT: i32 = 0;
    let mut NCDATA: i32 = 0;
    let mut NDDATA: i32 = 0;
    let mut NIDATA: i32 = 0;
    let mut NRESVR: i32 = 0;
    let mut NRESVC: i32 = 0;
    let mut NCOMR: i32 = 0;
    let mut NCOMC: i32 = 0;
    let mut NUMBLK: i32 = 0;
    let mut NUMDTA: i32 = 0;
    let mut NUMLFT: i32 = 0;
    let mut RECNO: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local parameters
    //
    //
    // CHARACTER*(*)         BEGRES
    // PARAMETER           ( BEGRES = 'BEGIN_RESERVED_BLOCK'      )
    //
    // CHARACTER*(*)         ENDRES
    // PARAMETER           ( ENDRES = 'END_RESERVED_BLOCK'        )
    //
    // CHARACTER*(*)         TRRBLK
    // PARAMETER           ( TRRBLK = 'TOTAL_RESERVED_BLOCKS'     )
    //

    //
    // Some parameters for writing the array markers
    //
    //
    // Length of a character buffer array element.
    //
    //
    // Length of a DAS file ID word.
    //
    //
    // Length of a DAS internal filename.
    //
    //
    // Length of a DAS comment record, in characters.
    //
    //
    // Size of the character, double precision, and integer data buffers.
    //
    //
    // Beginning and ending string positions for reading/writing
    // character data from/to a DAS file using the character data
    // buffer.
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
        CHKIN(b"DASBT", ctx)?;
    }
    //
    // When converting a binary DAS file into its DAS transfer file
    // equivalent, all of the data contained in the binary file is
    // placed into the DAS transfer file by this routine. This includes
    // the reserved record area, the comment area, and the character,
    // double precision, and integer data arrays as well.
    //
    // Currently, the reserved record area has not been implemented, as
    // there is no need for it at this time. If, or when, the reserved
    // record area is implemented, this routine will need to be modified
    // in order to support it. See the code for details.
    //
    // The data from the binary file are written to the DAS transfer
    // file as sequences of small blocks of data. This is to provide
    // a means for performing some error detection when converting a
    // DAS transfer file into its binary equivalent. Each block of
    // data is enclosed within begin and end block markers which hold
    // the count of data items in a data block. When all of the data
    // blocks for a data area have been written, a total blocks line is
    // written to the DAS transfer file.
    //
    // The data from the binary DAS file MUST appear in the following
    // order in the DAS transfer file.
    //
    //       1) Reserved records (when/if implemented)
    //       2) Comment area
    //       3) Character data array
    //       4) Double precision data array
    //       5) Integer data array
    //
    // If the data count for any of these DAS data areas is zero, no
    // data or markers for it are placed into the DAS transfer file.
    // Conversion proceeds with the next DAS data area in the list.
    //
    // For example, suppose that we have a binary DAS file where there
    // are 0 reserved characters in the reserved record area, 5000
    // comment characters in the comment area, and that the character,
    // double precision, and integer array counts are 0, 2300, and
    // 6900, respectively. Then, the DAS transfer file will contain
    // no reserved record data blocks, 2 comment data blocks, no
    // character data blocks, 3 double precision data blocks, and 7
    // integer data blocks, in that order.
    //
    // DAS transfer file description.
    // ----------------------------------
    //
    // A brief description of the DAS encoded file format and its
    // intended use follows. This description is intended to provide a
    // simple ``picture'' of the DAS transfer file format to aid in the
    // understanding of this routine. This description is NOT intended to
    // be a detailed specification of the file format.
    //
    // A DAS transfer file contains all of the data from a binary
    // DAS file in an encoded ASCII format. It also contains some
    // bookkeeping information for maintaining the integrity of the
    // data. The DAS transfer file format allows the full precision of
    // character, integer, and floating point numeric data to be
    // maintained in a portable fashion. The DAS transfer file format is
    // intended to provide a reliable and accurate means for porting data
    // among multiple computer systems and for the archival storage of
    // data.
    //
    // A DAS transfer file is not intended to be used directly to provide
    // data to a program. The equivalent binary DAS file is to be used
    // for this purpose. In no way should any program, other than a DAS
    // binary <-> transfer conversion program, rely on the DAS transfer
    // file format.
    //
    // To correctly understand the DAS transfer file description  the
    // reader should be familiar with the DAS file architecture. Items
    // enclosed in angle brackets, '<' and '>', are used to represent the
    // data which are to be placed at that position in the file. The
    // bookkeeping information which appears is represented exactly as it
    // would appear in a DAS transfer file.
    //
    // Let
    //
    //    <BOF>  denote the beginning of the file
    //    <EOF>  denote the end of the file
    //
    // and
    //
    //    nresvb  denote the number of encoded reserved record data
    //            blocks generated
    //    nresvc  denote the total number of reserved record characters
    //            in the  reserved record area of a DAS file
    //    ncomb   denote the number of encoded comment data blocks
    //            generated
    //    ncomc   denote the total number of comment characters in the
    //            comment area of a DAS file
    //    nchrb   denote the number of encoded character data blocks
    //            generated
    //    nchrs   denote the count of characters in the DAS character
    //            data array
    //    ndpb    denote the number of encoded double precision data
    //            blocks generated
    //    ndps    denote the count of double precision numbers in the DAS
    //            double precision data array
    //    nintb   denote the number of encoded integer data blocks
    //            generated
    //    nints   denote the count of integers in the DAS integer data
    //            array
    //
    // A DAS encoded transfer file has the following format:
    //
    //    <BOF>
    //    < Information line >
    //    < DAS file ID word >
    //    < Internal filename >
    //    < Encoded count of reserved records >
    //    < Encoded count of reserved characters >
    //    < Encoded count of comment records >
    //    < Encoded count of comment characters >
    //    < Blocks of encoded reserved record data, if nresvc > 0 >
    //    TOTAL_RESERVED_BLOCKS nresvb nresvc
    //    < Blocks of encoded comment data, if ncomc > 0 >
    //    TOTAL_COMMENT_BLOCKS ncomb ncomc
    //    < Encoded count of character data >
    //    < Encoded count of double precision data >
    //    < Encoded count of integer data >
    //    < Blocks of encoded character data, if nchrs > 0 >
    //    TOTAL_CHARACTER_BLOCKS nchrb nchrs
    //    < Blocks of encoded double precision data, if ndps > 0 >
    //    TOTAL_DP_BLOCKS ndpb ndps
    //    < Blocks of encoded integer data, if nints > 0 >
    //    TOTAL_INTEGER_BLOCKS nintb nints
    //    <EOF>
    //
    // This routine will check the SPICELIB function FAILED() after
    // each call, or consecutive sequence of calls, to data encoding
    // routines, and if an error was signaled it will simply check out
    // and return to the caller.
    //
    // This routine will check the SPICELIB function FAILED() after
    // each DAS file access call, and if an error was signaled it will
    // simply check out and return to the caller.
    //
    // We begin by opening the binary DAS file specified by BINFIL for
    // read access, obtaining a file handle.
    //
    DASOPR(BINFIL, &mut HANDLE, ctx)?;

    if FAILED(ctx) {
        //
        // If an error occurred while opening the file check out and
        // return to the caller.
        //
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Get the contents of the DAS file record.
    //
    DASRFR(
        HANDLE,
        &mut IDWORD,
        &mut IFNAME,
        &mut NRESVR,
        &mut NRESVC,
        &mut NCOMR,
        &mut NCOMC,
        ctx,
    )?;
    //
    // Convert the DAS file handle into its equivalent Fortran logical
    // unit. We need the logical unit so that we can read the reserved
    // records and the comment records.
    //
    ZZDDHHLU(HANDLE, b"DAS", false, &mut DASLUN, ctx)?;

    if FAILED(ctx) {
        //
        // If an error occurred while converting the DAS file handle to
        // a logical unit, attempt to close the binary file, then check
        // out and return.
        //
        DASCLS(HANDLE, ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Check to be sure that the number of reserved records and the
    // number of reserved characters are not being used. The DAS
    // reserved record area is not currently implemented, so nobody
    // should be using it.
    //
    if (NRESVC != 0) {
        //
        // Set the error message, close the file, signal the error, and
        // exit.
        //
        SETMSG(b"The number of reserved characters was nonzero (#) in file: #, but the DAS reserved record area has NOT been implemented yet!", ctx);
        ERRINT(b"#", NRESVC, ctx);
        ERRFNM(b"#", DASLUN, ctx)?;
        DASCLS(HANDLE, ctx)?;
        SIGERR(b"SPICE(BADDASFILE)", ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }

    if (NRESVR != 0) {
        //
        // Set the error message, close the file, signal the error, and
        // exit.
        //
        SETMSG(b"The number of reserved records was nonzero (#) in file: #, but the DAS reserved record area has NOT been implemented yet!", ctx);
        ERRINT(b"#", NRESVR, ctx);
        ERRFNM(b"#", DASLUN, ctx)?;
        DASCLS(HANDLE, ctx)?;
        SIGERR(b"SPICE(BADDASFILE)", ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Write the information line containing the file type information
    // and format version for the DAS transfer to the current position in
    // the file. The file format version information must be the first
    // ``word'' on the information line. The rest of the line may be used
    // for other purposes. Right now, it simply contains an expanded
    // description of the file format version information ``word.''
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
        //
        // An error occurred, so close the binary DAS file, set an
        // appropriate error message, and return to the caller.
        //
        DASCLS(HANDLE, ctx)?;
        SETMSG(
            b"Error writing to the DAS transfer file: #. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Write the DAS ID word to the DAS transfer file.
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
        //
        // An error occurred, so close the binary DAS file, set an
        // appropriate error message, and return to the caller.
        //
        DASCLS(HANDLE, ctx)?;
        SETMSG(
            b"Error writing to the DAS transfer file: #. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Write the internal file name of the DAS file to the DAS transfer
    // file.
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
        //
        // An error occurred, so close the binary DAS file, set an
        // appropriate error message, and return to the caller.
        //
        DASCLS(HANDLE, ctx)?;
        SETMSG(
            b"Error writing to the DAS transfer file: #. IOSTAT = #.",
            ctx,
        );
        ERRFNM(b"#", XFRLUN, ctx)?;
        ERRINT(b"#", IOSTAT, ctx);
        SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Write the number of reserved records and reserved characters to
    // the DAS transfer file.
    //
    WRENCI(XFRLUN, 1, &[NRESVR], ctx)?;
    WRENCI(XFRLUN, 1, &[NRESVC], ctx)?;

    if FAILED(ctx) {
        //
        // If an error occurred while writing the number of reserved
        // records or number of reserved characters, attempt to close
        // the binary file, then check out and return.
        //
        DASCLS(HANDLE, ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Write the number of comment records and comment characters to
    // the DAS transfer file.
    //
    WRENCI(XFRLUN, 1, &[NCOMR], ctx)?;
    WRENCI(XFRLUN, 1, &[NCOMC], ctx)?;

    if FAILED(ctx) {
        //
        // If an error occurred while writing the number of comment
        // records or number of comment characters, attempt to close
        // the binary file, then check out and return.
        //
        DASCLS(HANDLE, ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // **************************************************************
    // When/if the reserved record area is implemented, the code to
    // convert it and place it into the DAS transfer file should go
    // here. It should be possible to simply copy the code for the
    // comment area, making all of the necessary variable name changes,
    // etc., since the reserved record area is going to contain ONLY
    // character data.
    // **************************************************************
    //
    // Write out the comment area of the DAS file, if there are any
    // comment characters stored in it.
    //
    if (NCOMC > 0) {
        //
        // Write out the comment records, one at a time.
        //
        fstr::assign(&mut CRECRD, b" ");
        NUMLFT = NCOMC;
        NUMBLK = 0;
        RECNO = (1 + NRESVR);

        while (NUMLFT > 0) {
            NUMBLK = (NUMBLK + 1);
            RECNO = (RECNO + 1);

            if (NUMLFT > CRLEN) {
                NUMDTA = CRLEN;
            } else {
                NUMDTA = NUMLFT;
            }
            //
            // Write out the begin comment block marker and the number of
            // comment characters.
            //
            fstr::assign(
                &mut LINE,
                &fstr::concat(&fstr::concat(BEGCOM, b" #"), b" #"),
            );
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Read a comment record and then encode and write it.
            //
            DASIOC(b"READ", DASLUN, RECNO, &mut CRECRD, ctx)?;
            WRENCC(XFRLUN, NUMDTA, CharArray::from_ref(&CRECRD), ctx)?;

            if FAILED(ctx) {
                //
                // We want to check failed here because were in a loop.
                // We should exit the loop, and the routine, as soon as
                // an error is detected, so we don't continue doing things
                // for a long time. Attempt to close the binary DAS file
                // that we opened and then return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Write out the end comment block marker and the number of
            // comment characters.
            //
            fstr::assign(
                &mut LINE,
                &fstr::concat(&fstr::concat(ENDCOM, b" #"), b" #"),
            );
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Update the number of comment characters remaining to be
            // written.
            //
            NUMLFT = (NUMLFT - NUMDTA);
        }
        //
        // Write out the number of comment blocks processed, and the
        // count of comment characters
        //
        fstr::assign(
            &mut LINE,
            &fstr::concat(&fstr::concat(TCMBLK, b" #"), b" #"),
        );
        REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
        REPMI(&LINE.clone(), b"#", NCOMC, &mut LINE, ctx);
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
            //
            // An error occurred, so close the binary DAS file, set an
            // appropriate error message, and return to the caller.
            //
            DASCLS(HANDLE, ctx)?;
            SETMSG(
                b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", XFRLUN, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            CHKOUT(b"DASBT", ctx)?;
            return Ok(());
        }
    }
    //
    // Read in the data counts for each of the data types from the binary
    // DAS file.
    //
    DASLLA(HANDLE, &mut NCDATA, &mut NDDATA, &mut NIDATA, ctx)?;
    //
    // Write the data counts to the DAS transfer file. These will be
    // useful in determining which data types to expect in the DAS
    // transfer file when converting it back to binary.
    //
    WRENCI(XFRLUN, 1, &[NCDATA], ctx)?;
    WRENCI(XFRLUN, 1, &[NDDATA], ctx)?;
    WRENCI(XFRLUN, 1, &[NIDATA], ctx)?;

    if FAILED(ctx) {
        //
        // If an error occurred while writing any of the data counts to
        // the DAS transfer file, attempt to close the binary file, then
        // check out and return.
        //
        DASCLS(HANDLE, ctx)?;
        CHKOUT(b"DASBT", ctx)?;
        return Ok(());
    }
    //
    // Encode and write the CHARACTER data to the DAS transfer file, if
    // there is any character data.
    //
    if (NCDATA > 0) {
        NUMBLK = 0;
        DTABEG = 1;
        NUMLFT = NCDATA;

        while (NUMLFT > 0) {
            NUMBLK = (NUMBLK + 1);

            if (NUMLFT >= (CBFLEN * BUFSIZ)) {
                NUMDTA = (CBFLEN * BUFSIZ);
            } else {
                NUMDTA = NUMLFT;
            }
            //
            // Write out the begin data block identifier, the block
            // number, and the data count for the block.
            //
            fstr::assign(&mut LINE, &fstr::concat(&fstr::concat(BCBLK, b" #"), b" #"));
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Read in NUMDTA characters. The desired data are specified by
            // beginning and ending indices into the array, inclusive: thus
            // the subtraction of 1 in the call.
            //
            DASRDC(
                HANDLE,
                DTABEG,
                ((DTABEG + NUMDTA) - 1),
                BCBPOS,
                ECBPOS,
                CBUFFR.as_arg_mut(),
                ctx,
            )?;
            //
            // Encode and write out a buffer of characters.
            //
            WRENCC(XFRLUN, NUMDTA, CBUFFR.as_arg(), ctx)?;

            if FAILED(ctx) {
                //
                // We want to check failed here because were in a loop.
                // We should exit the loop, and the routine, as soon as
                // an error is detected, so we don't continue doing things
                // for a long time. Attempt to close the binary DAS file
                // that we opened and then return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Write out the end data block identifier, the block number,
            // and the data count for the block.
            //
            fstr::assign(&mut LINE, &fstr::concat(&fstr::concat(ECBLK, b" #"), b" #"));
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Increment the data pointer and decrement the amount of data
            // left to move.
            //
            DTABEG = (DTABEG + NUMDTA);
            NUMLFT = (NUMLFT - NUMDTA);
        }
        //
        // Write out the number of character data blocks processed
        // processed, and the count of double precision data items.
        //
        fstr::assign(
            &mut LINE,
            &fstr::concat(&fstr::concat(TCBLKS, b" #"), b" #"),
        );
        REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
        REPMI(&LINE.clone(), b"#", NCDATA, &mut LINE, ctx);
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
            //
            // An error occurred, so close the binary DAS file, set an
            // appropriate error message, and return to the caller.
            //
            DASCLS(HANDLE, ctx)?;
            SETMSG(
                b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", XFRLUN, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            CHKOUT(b"DASBT", ctx)?;
            return Ok(());
        }
    }
    //
    // Encode and write the DOUBLE PRECISION data to the DAS transfer
    // file.
    //
    if (NDDATA > 0) {
        NUMBLK = 0;
        DTABEG = 1;
        NUMLFT = NDDATA;

        while (NUMLFT > 0) {
            NUMBLK = (NUMBLK + 1);

            if (NUMLFT >= BUFSIZ) {
                NUMDTA = BUFSIZ;
            } else {
                NUMDTA = NUMLFT;
            }
            //
            // Write out the begin data block identifier, the block
            // number, and the data count for the block.
            //
            fstr::assign(&mut LINE, &fstr::concat(&fstr::concat(BDBLK, b" #"), b" #"));
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Read in NUMDTA double precision numbers.The desired data are
            // specified by beginning and ending indices into the array,
            // inclusive: thus the subtraction of 1 in the call.
            //
            DASRDD(
                HANDLE,
                DTABEG,
                ((DTABEG + NUMDTA) - 1),
                DBUFFR.as_slice_mut(),
                ctx,
            )?;
            //
            // Encode and write out a buffer of double precision numbers.
            //
            WRENCD(XFRLUN, NUMDTA, DBUFFR.as_slice(), ctx)?;

            if FAILED(ctx) {
                //
                // We want to check failed here because were in a loop.
                // We should exit the loop, and the routine, as soon as
                // an error is detected, so we don't continue doing things
                // for a long time. Attempt to close the binary DAS file
                // that we opened and then return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Write out the end data block identifier, the block number,
            // and the data count for the block.
            //
            fstr::assign(&mut LINE, &fstr::concat(&fstr::concat(EDBLK, b" #"), b" #"));
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Increment the data pointer and decrement the amount of data
            // left to move.
            //
            DTABEG = (DTABEG + NUMDTA);
            NUMLFT = (NUMLFT - NUMDTA);
        }
        //
        // Write out the number of double precision processed data blocks
        // processed, and the count of double precision data items.
        //
        fstr::assign(
            &mut LINE,
            &fstr::concat(&fstr::concat(TDBLKS, b" #"), b" #"),
        );
        REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
        REPMI(&LINE.clone(), b"#", NDDATA, &mut LINE, ctx);
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
            //
            // An error occurred, so close the binary DAS file, set an
            // appropriate error message, and return to the caller.
            //
            DASCLS(HANDLE, ctx)?;
            SETMSG(
                b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", XFRLUN, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            CHKOUT(b"DASBT", ctx)?;
            return Ok(());
        }
    }
    //
    // Encode and write the INTEGER data to the DAS transfer file, if
    // there is any.
    //
    if (NIDATA > 0) {
        NUMBLK = 0;
        DTABEG = 1;
        NUMLFT = NIDATA;

        while (NUMLFT > 0) {
            NUMBLK = (NUMBLK + 1);

            if (NUMLFT >= BUFSIZ) {
                NUMDTA = BUFSIZ;
            } else {
                NUMDTA = NUMLFT;
            }
            //
            // Write out the begin data block identifier, the block number,
            // and the data count for the block.
            //
            fstr::assign(&mut LINE, &fstr::concat(&fstr::concat(BIBLK, b" #"), b" #"));
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Read in NUMDTA integers. The desired data are specified by
            // beginning and ending indices into the array,inclusive: thus
            // the subtraction of 1 in the call.
            //
            DASRDI(
                HANDLE,
                DTABEG,
                ((DTABEG + NUMDTA) - 1),
                IBUFFR.as_slice_mut(),
                ctx,
            )?;
            //
            // Encode and write out a buffer of integers.
            //
            WRENCI(XFRLUN, NUMDTA, IBUFFR.as_slice(), ctx)?;

            if FAILED(ctx) {
                //
                // We want to check failed here because were in a loop.
                // We should exit the loop, and the routine, as soon as
                // an error is detected, so we don't continue doing things
                // for a long time. Attempt to close the binary DAS file
                // that we opened and then return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Write out the end data block identifier, the block number,
            // and the data count for the block.
            //
            fstr::assign(&mut LINE, &fstr::concat(&fstr::concat(EIBLK, b" #"), b" #"));
            REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
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
                //
                // An error occurred, so close the binary DAS file, set an
                // appropriate error message, and return to the caller.
                //
                DASCLS(HANDLE, ctx)?;
                SETMSG(
                    b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                    ctx,
                );
                ERRFNM(b"#", XFRLUN, ctx)?;
                ERRINT(b"#", IOSTAT, ctx);
                SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
                CHKOUT(b"DASBT", ctx)?;
                return Ok(());
            }
            //
            // Increment the data pointers and decrement the amount of data
            // left.
            //
            DTABEG = (DTABEG + NUMDTA);
            NUMLFT = (NUMLFT - NUMDTA);
        }
        //
        // Write out the number of processed integer data blocks
        // processed, and the count of double precision data items.
        //
        fstr::assign(
            &mut LINE,
            &fstr::concat(&fstr::concat(TIBLKS, b" #"), b" #"),
        );
        REPMI(&LINE.clone(), b"#", NUMBLK, &mut LINE, ctx);
        REPMI(&LINE.clone(), b"#", NIDATA, &mut LINE, ctx);
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
            //
            // An error occurred, so close the binary DAS file, set an
            // appropriate error message, and return to the caller.
            //
            DASCLS(HANDLE, ctx)?;
            SETMSG(
                b"Error writing to the DAS transfer file: #. IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", XFRLUN, ctx)?;
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(FILEWRITEFAILED)", ctx)?;
            CHKOUT(b"DASBT", ctx)?;
            return Ok(());
        }
    }
    //
    // Close only the binary DAS file.
    //
    DASCLS(HANDLE, ctx)?;

    CHKOUT(b"DASBT", ctx)?;
    Ok(())
}
