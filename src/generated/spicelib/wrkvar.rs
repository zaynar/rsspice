//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

pub const LBCELL: i32 = -5;
const BLANK: &[u8; 1] = &fstr::extend_const::<1>(b" ");
const LPAREN: &[u8; 1] = &fstr::extend_const::<1>(b"(");
const RPAREN: &[u8; 3] = &fstr::extend_const::<3>(b"  )");
const COMMA: &[u8; 2] = &fstr::extend_const::<2>(b",");

/// Write a variable to a kernel file
///
/// Write the value of a variable in a double precision symbol
/// table to a NAIF ASCII kernel file.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [SYMBOLS](crate::required_reading::symbols)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  UNIT       I   Output logical unit.
///  NAME       I   Name of the variable.
///  DIRCTV     I   Kernel directive: '=' or '+='.
///  TABSYM,
///  TABPTR,
///  TABVAL     I   Double precision symbol table.
/// ```
///
/// # Detailed Input
///
/// ```text
///  UNIT     is the logical unit to which the variable will be
///           written. This is usually the logical unit to which the
///           output kernel file is connected.
///
///  NAME     is the name of the variable to be written to UNIT.
///
///  DIRCTV   is the directive linking NAME and its associated values
///           in the kernel file. This may be any of the directives
///           recognized by RDKVAR.
///
///  TABSYM,
///  TABPTR,
///  TABVAL   are the components of a double precision symbol table.
///
///           On input, the table may or may not contain any variables.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If an error occurs writing the variable to UNIT, the error
///      SPICE(WRITEERROR) is signaled.
/// ```
///
/// # Files
///
/// ```text
///  If the variable is to be written to an output kernel file, the
///  file should be opened with a logical unit determined by the
///  calling program.
/// ```
///
/// # Particulars
///
/// ```text
///  If the table symbol table does not contain any variables, nothing
///  will be written to UNIT.
/// ```
///
/// # Examples
///
/// ```text
///  If  NAME   = 'MEAN_ANOM'
///      DIRCTV = '='
///
///  And the contents of the symbol table are:
///
///       DELTA_T_A  -->   32.184
///       K          -->    0.D0
///       MEAN_ANOM  -->    6.239996D0
///                         1.99096871D-7
///       ORBIT_ECC  -->    1.671D-2
///
///  The output to UNIT might look like this, depending on the
///  length of the symbol table variables:
///
///     MEAN_ANOM   = ( 6.239996D0,
///                     1.99096871D-7  )
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 08-APR-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
///         Updated $Brief_I/O entry for symbol table (TABSYM, TABPTR,
///         TABVAR) to indicate that these arguments only are an input to
///         the routine, and not an output. Removed $Detailed_Output
///         documentation, as the routine operates by side-effects and has
///         no output arguments.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (IMU) (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 20-DEC-1988 (NJB)
///
///         Call to IOERR changed to be consistent with new calling
///         protocol. SETMSG call deleted, since IOERR now calls SETMSG.
/// ```
pub fn wrkvar(
    ctx: &mut SpiceContext,
    unit: i32,
    name: &str,
    dirctv: &str,
    tabsym: CharArray,
    tabptr: &[i32],
    tabval: &[f64],
) -> crate::Result<()> {
    WRKVAR(
        unit,
        name.as_bytes(),
        dirctv.as_bytes(),
        tabsym,
        tabptr,
        tabval,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure WRKVAR ( Write a variable to a kernel file )
pub fn WRKVAR(
    UNIT: i32,
    NAME: &[u8],
    DIRCTV: &[u8],
    TABSYM: CharArray,
    TABPTR: &[i32],
    TABVAL: &[f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let TABSYM = DummyCharArray::new(TABSYM, None, LBCELL..);
    let TABPTR = DummyArray::new(TABPTR, LBCELL..);
    let TABVAL = DummyArray::new(TABVAL, LBCELL..);
    let mut VARLEN: i32 = 0;
    let mut MARGIN: i32 = 0;
    let mut VARDIM: i32 = 0;
    let mut LINE = [b' '; 132];
    let mut DVALUE: f64 = 0.0;
    let mut FOUND: bool = false;
    let mut IOSTAT: i32 = 0;

    //
    // SPICELIB functions
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
        CHKIN(b"WRKVAR", ctx)?;
    }

    //
    // Preliminary measurements.
    //
    VARLEN = intrinsics::LEN(&TABSYM[1]);
    MARGIN = (VARLEN + 6);
    VARDIM = SYDIMD(
        NAME,
        TABSYM.as_arg(),
        TABPTR.as_slice(),
        TABVAL.as_slice(),
        ctx,
    )?;

    //
    // One value per line.
    //
    for I in 1..=VARDIM {
        SYNTHD(
            NAME,
            I,
            TABSYM.as_arg(),
            TABPTR.as_slice(),
            TABVAL.as_slice(),
            &mut DVALUE,
            &mut FOUND,
            ctx,
        )?;

        //
        // The first line contains the variable name, the directive,
        // an optional left parenthesis, and the first value. The values
        // of a multi-dimensional variable are separated by commas.
        //
        if (I == 1) {
            LJUST(NAME, &mut LINE);
            RJUST(
                DIRCTV,
                fstr::substr_mut(&mut LINE, (MARGIN - 4)..=(MARGIN - 3)),
            );

            if (VARDIM > 1) {
                fstr::assign(
                    fstr::substr_mut(&mut LINE, (MARGIN - 1)..=(MARGIN - 1)),
                    LPAREN,
                );
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_str(fstr::substr(&LINE, 1..=MARGIN))?;
                        writer.write_f64(DVALUE)?;
                        writer.write_str(COMMA)?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }
            } else {
                {
                    use f2rust_std::{
                        data::Val,
                        io::{self, Writer},
                    };

                    let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
                    IOSTAT = io::capture_iostat(|| {
                        writer.start()?;
                        writer.write_str(fstr::substr(&LINE, 1..=MARGIN))?;
                        writer.write_f64(DVALUE)?;
                        writer.finish()?;
                        Ok(())
                    })?;
                }
            }

        //
        // The last line of a multi-dimensional variable ends with a
        // right parenthesis.
        //
        } else if ((I > 1) && (I == VARDIM)) {
            fstr::assign(&mut LINE, b" ");
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=MARGIN))?;
                    writer.write_f64(DVALUE)?;
                    writer.write_str(RPAREN)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }
        } else {
            fstr::assign(&mut LINE, b" ");
            {
                use f2rust_std::{
                    data::Val,
                    io::{self, Writer},
                };

                let mut writer = io::ListDirectedWriter::new(ctx.io_unit(UNIT)?, None)?;
                IOSTAT = io::capture_iostat(|| {
                    writer.start()?;
                    writer.write_str(fstr::substr(&LINE, 1..=MARGIN))?;
                    writer.write_f64(DVALUE)?;
                    writer.write_str(COMMA)?;
                    writer.finish()?;
                    Ok(())
                })?;
            }
        }
    }

    if (IOSTAT != 0) {
        IOERR(
            b"writing a variable to the output kernel file",
            b" ",
            IOSTAT,
            ctx,
        );

        SIGERR(b"SPICE(WRITEERROR)", ctx)?;
    }

    CHKOUT(b"WRKVAR", ctx)?;
    Ok(())
}
