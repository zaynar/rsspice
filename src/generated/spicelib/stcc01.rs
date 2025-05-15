//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CNAMSZ: i32 = 32;
const TNAMSZ: i32 = 64;
const MXCLSG: i32 = 100;
const CAT1NC: i32 = 7;

struct SaveVars {
    CAT1DT: ActualCharArray,
    DTYPES: ActualCharArray,
    CAT1NM: ActualCharArray,
    CNAMES: ActualCharArray,
    TMPTNM: Vec<u8>,
    TNMPRV: Vec<u8>,
    J: i32,
    NCOLS: i32,
    NROWS: i32,
    NUMSEG: i32,
    SIZES: StackArray<i32, 100>,
    STRLNS: StackArray<i32, 100>,
    TMPHND: i32,
    FOUND: bool,
    INDEXD: StackArray<bool, 100>,
    NULLOK: StackArray<bool, 100>,
}

impl SaveInit for SaveVars {
    fn new() -> Self {
        let mut CAT1DT = ActualCharArray::new(4, 1..=CAT1NC);
        let mut DTYPES = ActualCharArray::new(4, 1..=MXCLSG);
        let mut CAT1NM = ActualCharArray::new(CNAMSZ, 1..=CAT1NC);
        let mut CNAMES = ActualCharArray::new(CNAMSZ, 1..=MXCLSG);
        let mut TMPTNM = vec![b' '; TNAMSZ as usize];
        let mut TNMPRV = vec![b' '; TNAMSZ as usize];
        let mut J: i32 = 0;
        let mut NCOLS: i32 = 0;
        let mut NROWS: i32 = 0;
        let mut NUMSEG: i32 = 0;
        let mut SIZES = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut STRLNS = StackArray::<i32, 100>::new(1..=MXCLSG);
        let mut TMPHND: i32 = 0;
        let mut FOUND: bool = false;
        let mut INDEXD = StackArray::<bool, 100>::new(1..=MXCLSG);
        let mut NULLOK = StackArray::<bool, 100>::new(1..=MXCLSG);

        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"CATALOG_NUMBER"),
                Val::C(b"RA"),
                Val::C(b"DEC"),
                Val::C(b"RA_SIGMA"),
                Val::C(b"DEC_SIGMA"),
                Val::C(b"VISUAL_MAGNITUDE"),
                Val::C(b"SPECTRAL_TYPE"),
            ]
            .into_iter();
            CAT1NM
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }
        {
            use f2rust_std::data::Val;

            let mut clist = [
                Val::C(b"INT"),
                Val::C(b"DP"),
                Val::C(b"DP"),
                Val::C(b"DP"),
                Val::C(b"DP"),
                Val::C(b"DP"),
                Val::C(b"CHR"),
            ]
            .into_iter();
            CAT1DT
                .iter_mut()
                .for_each(|n| fstr::assign(n, clist.next().unwrap().into_str()));

            debug_assert!(clist.next().is_none(), "DATA not fully initialised");
        }

        Self {
            CAT1DT,
            DTYPES,
            CAT1NM,
            CNAMES,
            TMPTNM,
            TNMPRV,
            J,
            NCOLS,
            NROWS,
            NUMSEG,
            SIZES,
            STRLNS,
            TMPHND,
            FOUND,
            INDEXD,
            NULLOK,
        }
    }
}

/// STAR catalog type 1, check whether type 1
///
/// Check whether a file is a type 1 star catalog and return the
/// catalog's table name if it is.
///
/// # Required Reading
///
/// * [EK](crate::required_reading::ek)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  CATFNM     I   Catalog file name.
///  TABNAM     O   Catalog table name.
///  ISTYP1     O   .TRUE. when file is type 1 star catalog.
///  ERRMSG     O   Error message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  CATFNM   is the name of the catalog file.
/// ```
///
/// # Detailed Output
///
/// ```text
///  TABNAM   is the name of the data table contained in the
///           catalog. Set to blank if file is not a type 1 star
///           catalog.
///
///  ISTYP1   is .TRUE. when the file is a type 1 star catalog. .FALSE.
///           otherwise.
///
///  ERRMSG   is a diagnostic message indicating why the file is
///           not a type 1 star catalog. Set to blank if the file
///           is a type 1 star catalog.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the indicated file cannot be opened, an error is signaled
///      by a routine in the call tree of this routine.
///
///  2)  If the indicated file has the wrong architecture version, an
///      error is signaled by a routine in the call tree of this
///      routine.
///
///  3)  If an I/O error occurs while reading the indicated file, the
///      error is signaled by a routine in the call tree of this
///      routine.
/// ```
///
/// # Files
///
/// ```text
///  This routine checks whether file is really SPICE type 1 star
///  catalog file.
///
///  SPICE type 1 star catalog files MUST contain a single data table.
///  It can occupy a single segment or it can spread across multiple
///  segments. This table MUST include the following columns:
///
///     column name                data type          units
///  -------------------------------------------------------
///     RA                   DOUBLE PRECISION        DEGREES
///     DEC                  DOUBLE PRECISION        DEGREES
///     RA_SIGMA             DOUBLE PRECISION        DEGREES
///     DEC_SIGMA            DOUBLE PRECISION        DEGREES
///     CATALOG_NUMBER       INTEGER
///     SPECTRAL_TYPE        CHARACTER*(4)
///     VISUAL_MAGNITUDE     DOUBLE PRECISION
///
///  Nulls are not allowed in any of the columns.
///  Other columns can also be present in the table but their data
///  will NOT be accessible through type 1 star catalog access
///  routines. Note that the names and attributes of these additional
///  columns must be identical for all segments containing this table.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine does not need to be called by the user's program.
///  It is used by star catalog loader routines to check
///  whether a particular file is a type 1 star catalog before loading
///  the file.
/// ```
///
/// # Examples
///
/// ```text
///  In the following code fragment, STCC01 is used to determine
///  whether a file is a SPICE type 1 star catalog.
///
///  C
///  C     Call STCC01 to determine whether the file is type 1 star
///  C     catalog file.
///  C
///        CALL STCC01 ( CATFNM, TABNAM, ISTYP1, ERRMSG )
///
///  C
///  C     Check ISTYP1 flag and stop execution and report an
///  C     error if file is not type 1 star catalog file.
///  C
///        IF ( .NOT. ISTYP1 ) THEN
///       .   WRITE (*,*) 'The file:'
///       .   WRITE (*,*) '  ',CATFNM(1:RTRIM(CATFNM))
///       .   WRITE (*,*) 'is not a type 1 star catalog.'
///       .   WRITE (*,*) ERRMSG
///           STOP
///        END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 16-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 1.0.0, 15-MAY-1996 (BVS)
/// ```
pub fn stcc01(
    ctx: &mut SpiceContext,
    catfnm: &str,
    tabnam: &mut str,
    istyp1: &mut bool,
    errmsg: &mut str,
) -> crate::Result<()> {
    STCC01(
        catfnm.as_bytes(),
        fstr::StrBytes::new(tabnam).as_mut(),
        istyp1,
        fstr::StrBytes::new(errmsg).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure STCC01 ( STAR catalog type 1, check whether type 1 )
pub fn STCC01(
    CATFNM: &[u8],
    TABNAM: &mut [u8],
    ISTYP1: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let save = ctx.get_vars::<SaveVars>();
    let save = &mut *save.borrow_mut();

    //
    //
    // SPICELIB functions
    //

    //
    // Local parameters.
    //

    //
    // Local variables
    //

    //
    // Initial values.
    //

    //
    // Standard SPICE error handling.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"STCC01", ctx)?;
    }

    //
    // More initial values.
    //
    fstr::assign(TABNAM, b" ");
    fstr::assign(ERRMSG, b" ");
    *ISTYP1 = true;

    //
    // Open star catalog file with low level "open for read access"
    // EK routine.
    //
    EKOPR(CATFNM, &mut save.TMPHND, ctx)?;

    //
    // Get the number of segments in the file and check whether it is
    // greater than 0 (i.e. some data are is present in the file). If
    // not then set an error message and return to the calling routine.
    //
    save.NUMSEG = EKNSEG(save.TMPHND, ctx)?;

    if (save.NUMSEG <= 0) {
        fstr::assign(ERRMSG, b"File contains no data.");
        *ISTYP1 = false;
        CHKOUT(b"STCC01", ctx)?;
        return Ok(());
    }

    //
    // Loop through the segments to find out whether all of them
    // contain pieces of the same table. If not then set
    // an error message and return to the calling routine.
    //
    for I in 1..=save.NUMSEG {
        EKSSUM(
            save.TMPHND,
            I,
            &mut save.TMPTNM,
            &mut save.NROWS,
            &mut save.NCOLS,
            save.CNAMES.as_arg_mut(),
            save.DTYPES.as_arg_mut(),
            save.SIZES.as_slice_mut(),
            save.STRLNS.as_slice_mut(),
            save.INDEXD.as_slice_mut(),
            save.NULLOK.as_slice_mut(),
            ctx,
        )?;

        if (I > 1) {
            if fstr::ne(&save.TMPTNM, &save.TNMPRV) {
                fstr::assign(ERRMSG, b"File contains more than one data table.");
                *ISTYP1 = false;
                CHKOUT(b"STCC01", ctx)?;
                return Ok(());
            }
        }

        fstr::assign(&mut save.TNMPRV, &save.TMPTNM);
    }

    //
    // Check whether the  number of columns is less than it
    // is supposed to be in type 1 star catalogs. If so then set
    // an error message and return to a calling routine.
    //

    if (save.NCOLS < CAT1NC) {
        fstr::assign(ERRMSG, b"File contains too few data columns.");
        *ISTYP1 = false;
        CHKOUT(b"STCC01", ctx)?;
        return Ok(());
    }

    //
    // Check whether all columns that will be used in catalog search and
    // star data fetching are present in the data table. If not
    // then set an error message and return to a calling routine.
    //
    for I in 1..=CAT1NC {
        save.FOUND = false;

        save.J = ISRCHC(&save.CAT1NM[I], save.NCOLS, save.CNAMES.as_arg());

        if (save.J > 0) {
            save.FOUND =
                (fstr::eq(save.CAT1DT.get(I), save.DTYPES.get(save.J)) && !save.NULLOK[save.J]);
        }

        if !save.FOUND {
            fstr::assign(
                ERRMSG,
                &fstr::concat(
                    &fstr::concat(
                        &fstr::concat(
                            b" Column ",
                            fstr::substr(save.CAT1NM.get(I), 1..=NBLEN(&save.CAT1NM[I])),
                        ),
                        b" is not found or",
                    ),
                    b" improperly declared in the file.",
                ),
            );
            *ISTYP1 = false;
            CHKOUT(b"STCC01", ctx)?;
            return Ok(());
        }
    }

    //
    // If we got to this point then all checks were passed successfully
    // and the file can be processed as a type 1 star catalog. We
    // "return" the table name and close the file with the EK close
    // routine.
    //
    fstr::assign(TABNAM, &save.TMPTNM);

    EKCLS(save.TMPHND, ctx)?;

    CHKOUT(b"STCC01", ctx)?;
    Ok(())
}
