//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const NWC: i32 = 1024;

/// DAS, Fortran I/O, character
///
/// Perform Fortran reads and writes of DAS character records.
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
///  ACTION     I   Action to take (read or write).
///  UNIT       I   Fortran unit connected to DAS file.
///  RECNO      I   Number of record to read or write.
///  RECORD    I-O  DAS character record.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ACTION   is a character string specifying whether to read
///           from or write to the specified DAS file. Possible
///           values are:
///
///              'READ'
///              'WRITE'
///
///           Case and leading or trailing blanks are not
///           significant.
///
///
///  UNIT     is the Fortran unit number connected to the DAS
///           file that is to be read or written. Given the
///           handle of the DAS file, the unit number can be
///           obtained using ZZDDHHLU.
///
///  RECNO    is the Fortran record number of the record to be
///           read or written.
///
///  RECORD   is a character array whose contents are to be
///           written to record RECNO, if ACTION is WRITE.
/// ```
///
/// # Detailed Output
///
/// ```text
///  RECORD   is a character array whose contents are to be
///           set equal to those of record RECNO, if ACTION is
///           READ.
/// ```
///
/// # Parameters
///
/// ```text
///  NWC      is the number of characters in a DAS character
///           record.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the value of ACTION is not recognized, the error
///      SPICE(UNRECOGNIZEDACTION) is signaled.
///
///  2)  If a Fortran read error occurs, the error
///      SPICE(DASFILEREADFAILED) is signaled.
///
///  3)  If a Fortran write error occurs, the error
///      SPICE(DASFILEWRITEFAILED) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  See the description of the argument UNIT in $Detailed_Input.
/// ```
///
/// # Particulars
///
/// ```text
///  Normally, routines outside of SPICELIB will not need to call this
///  routine directly. Writes to DAS files should be performed using
///  the DASADx and DASUDx routines; reads should be performed using
///  the DASRDx routines.
///
///  This routines centralizes I/O and the concomitant error handling
///  for DAS character records.
///
///  Although most DAS routines use file handles to identify DAS
///  files, this routine uses Fortran logical units for this purpose.
///  Using unit numbers allows the DASIOx routines to be called from
///  any DAS routine, including entry points of DASFM.  (DASFM
///  contains as entry points the routines DASHLU and DASLUH, which
///  map between handles and unit numbers.)
/// ```
///
/// # Examples
///
/// ```text
///  1)  Read and print to the screen character records number 10
///      through 20 from the DAS file designated by HANDLE.
///
///         CHARACTER*(NWC)       RECORD
///
///                        .
///                        .
///                        .
///
///         CALL ZZDDHHLU ( HANDLE, 'DAS', .FALSE., UNIT )
///         CALL DASHFN   ( HANDLE, NAME )
///
///         DO I = 1, 20
///
///            CALL DASIOC ( 'READ', UNIT, 10, RECORD )
///
///            LABEL = 'Contents of the # record in DAS file #: '
///
///            CALL REPMOT ( LABEL,  '#',  I,  'L',   LABEL )
///            CALL REPMC  ( LABEL,  '#',      NAME,  LABEL )
///
///            WRITE (*,*) LABEL
///            WRITE (*,*) ' '
///            WRITE (*,*) RECORD
///
///         END DO
///
///
///
///  2)  Write the contents of the string RECORD to record number
///      10 in the DAS file designated by HANDLE.
///
///
///         CHARACTER*(NWC)       RECORD
///
///                        .
///                        .
///                        .
///
///         CALL ZZDDHHLU ( HANDLE,  'DAS', .FALSE., UNIT   )
///         CALL DASIOC   ( 'WRITE', UNIT,  10,      RECORD )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 12-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.1, 05-FEB-2015 (NJB)
///
///         Header was updated to refer to ZZDDHHLU.
///
/// -    SPICELIB Version 1.0.0, 30-JUN-1992 (NJB) (WLT)
/// ```
pub fn dasioc(
    ctx: &mut SpiceContext,
    action: &str,
    unit: i32,
    recno: i32,
    record: &mut [u8; 1024],
) -> crate::Result<()> {
    DASIOC(action.as_bytes(), unit, recno, record, ctx.raw_context())?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure DASIOC ( DAS, Fortran I/O, character )
pub fn DASIOC(
    ACTION: &[u8],
    UNIT: i32,
    RECNO: i32,
    RECORD: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let RECORD = &mut RECORD[..NWC as usize];
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB functions
    //

    //
    // Local variables
    //

    //
    // Use discovery check-in.
    //
    if RETURN(ctx) {
        return Ok(());
    }

    if EQSTR(ACTION, b"READ") {
        //
        // We're supposed to read the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Reader},
            };

            let mut reader = io::UnformattedReader::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                reader.start()?;
                reader.read_str(RECORD)?;
                reader.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            CHKIN(b"DASIOC", ctx)?;
            SETMSG(
                b"Could not read DAS character record.  File = #  Record number = #.  IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", RECNO, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DASFILEREADFAILED)", ctx)?;
            CHKOUT(b"DASIOC", ctx)?;
            return Ok(());
        }
    } else if EQSTR(ACTION, b"WRITE") {
        //
        // We're supposed to write to the file.
        //
        {
            use f2rust_std::{
                data::Val,
                io::{self, Writer},
            };

            let mut writer = io::UnformattedWriter::new(ctx.io_unit(UNIT)?, Some(RECNO))?;
            IOSTAT = io::capture_iostat(|| {
                writer.start()?;
                writer.write_str(RECORD)?;
                writer.finish()?;
                Ok(())
            })?;
        }

        if (IOSTAT != 0) {
            CHKIN(b"DASIOC", ctx)?;
            SETMSG(
                b"Could not write DAS character record.  File = #  Record number = #.  IOSTAT = #.",
                ctx,
            );
            ERRFNM(b"#", UNIT, ctx)?;
            ERRINT(b"#", RECNO, ctx);
            ERRINT(b"#", IOSTAT, ctx);
            SIGERR(b"SPICE(DASFILEWRITEFAILED)", ctx)?;
            CHKOUT(b"DASIOC", ctx)?;
            return Ok(());
        }
    } else {
        //
        // The requested action is a little too weird.
        //
        CHKIN(b"DASIOC", ctx)?;
        SETMSG(b"Action was #; should be READ or WRITE", ctx);
        ERRCH(b"#", ACTION, ctx);
        SIGERR(b"SPICE(UNRECOGNIZEDACTION)", ctx)?;
        CHKOUT(b"DASIOC", ctx)?;
        return Ok(());
    }

    Ok(())
}
