//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const CDELEN: i32 = 16;
const NAMLEN: i32 = 32;

/// Return d.p. values from the kernel pool
///
/// Fetch from the kernel pool the double precision values
/// of an item associated with a body, where the body is
/// specified by an integer ID code.
///
/// # Required Reading
///
/// * [KERNEL](crate::required_reading::kernel)
/// * [NAIF_IDS](crate::required_reading::naif_ids)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  BODYID     I   Body ID code.
///  ITEM       I   Item for which values are desired. ('RADII',
///                 'NUT_PREC_ANGLES', etc. )
///  MAXN       I   Maximum number of values that may be returned.
///  DIM        O   Number of values returned.
///  VALUES     O   Values.
/// ```
///
/// # Detailed Input
///
/// ```text
///  BODYID   is the NAIF integer ID code for a body of interest.
///           For example, if the body is the earth, the code is
///           399.
///
///  ITEM     is the item to be returned. Together, the NAIF ID
///           code of the body and the item name combine to form a
///           kernel variable name, e.g.,
///
///                 'BODY599_RADII'
///                 'BODY401_POLE_RA'
///
///           The values associated with the kernel variable having
///           the name constructed as shown are sought. Below
///           we'll take the shortcut of calling this kernel variable
///           the "requested kernel variable."
///
///           Note that ITEM *is* case-sensitive. This attribute
///           is inherited from the case-sensitivity of kernel
///           variable names.
///
///  MAXN     is the maximum number of values that may be returned.
///           The output array VALUES must be declared with size at
///           least MAXN. It's an error to supply an output array
///           that is too small to hold all of the values associated
///           with the requested kernel variable.
/// ```
///
/// # Detailed Output
///
/// ```text
///  DIM      is the number of values returned; this is always the
///           number of values associated with the requested kernel
///           variable unless an error has been signaled.
///
///  VALUES   is the array of values associated with the requested
///           kernel variable. If VALUES is too small to hold all
///           of the values associated with the kernel variable, the
///           returned values of DIM and VALUES are undefined.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the requested kernel variable is not found in the kernel
///      pool, the error SPICE(KERNELVARNOTFOUND) is signaled.
///
///  2)  If the requested kernel variable is found but the associated
///      values aren't numeric, the error SPICE(TYPEMISMATCH) is
///      signaled.
///
///  3)  If the dimension of VALUES indicated by MAXN is too small to
///      contain the requested values, the error SPICE(ARRAYTOOSMALL)
///      is signaled. The output array VALUES must be declared with
///      sufficient size to contain all of the values associated with
///      the requested kernel variable.
///
///  4)  If the input dimension MAXN indicates there is more room
///      in VALUES than there really is---for example, if MAXN is
///      10 but values is declared with dimension 5---and the dimension
///      of the requested kernel variable is larger than the actual
///      dimension of VALUES, then this routine may overwrite
///      memory. The results are unpredictable.
/// ```
///
/// # Particulars
///
/// ```text
///  This routine simplifies looking up PCK kernel variables by
///  constructing names of requested kernel variables and by
///  performing error checking.
///
///  This routine is intended for use in cases where the maximum number
///  of values that may be returned is known at compile time. The
///  caller fetches all of the values associated with the specified
///  kernel variable via a single call to this routine. If the number
///  of values to be fetched cannot be known until run time, the
///  lower-level routine GDPOOL should be used instead. GDPOOL
///  supports fetching arbitrary amounts of data in multiple "chunks."
///
///  This routine is intended for use in cases where the requested
///  kernel variable is expected to be present in the kernel pool. If
///  the variable is not found or has the wrong data type, this
///  routine signals an error. In cases where it is appropriate to
///  indicate absence of an expected kernel variable by returning a
///  boolean "found flag" with the value .FALSE., again the routine
///  GDPOOL should be used.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as
///  input, the compiler and supporting libraries, and the machine
///  specific arithmetic implementation.
///
///  1) Retrieve the radii of the Earth from the kernel pool, using
///     both 'RADII' and 'radii' as the item name to return. Since
///     the ITEM variable possesses case sensitivity, the later case
///     should fail. Trap the error and print it to the output.
///
///     Use the PCK kernel below to load the required triaxial
///     ellipsoidal shape model for the Earth.
///
///        pck00008.tpc
///
///
///     Example code begins here.
///
///
///           PROGRAM BODVCD_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               NVALS
///           PARAMETER           ( NVALS = 3 )
///
///
///     C
///     C     Local variables.
///     C
///           DOUBLE PRECISION      VALUES (NVALS)
///
///           INTEGER               DIM
///
///     C
///     C     Load a PCK.
///     C
///           CALL FURNSH ( 'pck00008.tpc' )
///
///     C
///     C     When the kernel variable
///     C
///     C        BODY399_RADII
///     C
///     C     is present in the kernel pool---normally because a PCK
///     C     defining this variable has been loaded (as is the case
///     C     here)---the call
///     C
///           CALL BODVCD ( 399, 'RADII', 3, DIM, VALUES )
///
///     C
///     C     returns the dimension and values associated with the
///     C     variable 'BODY399_RADII'
///     C
///           WRITE(*,'(A,3F10.3)') '399 RADII: ', VALUES
///
///     C
///     C     The ITEM variable possesses case sensitivity. This
///     C     call should cause an error.
///     C
///           CALL BODVRD ( 'EARTH', 'radii', 3, DIM, VALUES )
///           WRITE(*,'(A,3F10.3)') '399 radii: ', VALUES
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///     399 RADII:   6378.140  6378.140  6356.750
///
///     ============================================================***
///
///     Toolkit version: N0066
///
///     SPICE(KERNELVARNOTFOUND) -- The Variable Was not Found in th***
///     Pool.
///
///     The variable BODY399_radii could not be found in the kernel ***
///
///     A traceback follows.  The name of the highest level module i***
///     BODVRD
///
///     Oh, by the way:  The SPICELIB error handling actions are USER-
///     TAILORABLE.  You can choose whether the Toolkit aborts or co***
///     when errors occur, which error messages to output, and where***
///     the output.  Please read the ERROR "Required Reading" file, ***
///     the routines ERRACT, ERRDEV, and ERRPRT.
///
///     ============================================================***
///
///
///     Warning: incomplete output. 8 lines extended past the right
///     margin of the header and have been truncated. These lines are
///     marked by "***" at the end of each line.
///
///
///     Note that, usually, the last call will cause a
///     SPICE(KERNELVARNOTFOUND) error to be signaled, because this
///     call will attempt to look up the values associated with a
///     kernel variable of the name
///
///        'BODY399_radii'
///
///     Since kernel variable names are case sensitive, this
///     name is not considered to match the name
///
///        'BODY399_RADII'
///
///     which normally would be present after a text PCK
///     containing data for all planets and satellites has
///     been loaded.
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  I.M. Underwood     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.0.1, 07-SEP-2021 (JDR)
///
///         Edited the header to comply with NAIF standard.
///         Added complete example code based on the existing fragments.
///
///         Removed note about GDPOOL being entry point of POOL from
///         $Particulars section.
///
/// -    SPICELIB Version 1.0.0, 24-OCT-2004 (NJB) (BVS) (WLT) (IMU)
/// ```
pub fn bodvcd(
    ctx: &mut SpiceContext,
    bodyid: i32,
    item: &str,
    maxn: i32,
    dim: &mut i32,
    values: &mut [f64],
) -> crate::Result<()> {
    BODVCD(
        bodyid,
        item.as_bytes(),
        maxn,
        dim,
        values,
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure BODVCD ( Return d.p. values from the kernel pool )
pub fn BODVCD(
    BODYID: i32,
    ITEM: &[u8],
    MAXN: i32,
    DIM: &mut i32,
    VALUES: &mut [f64],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut VALUES = DummyArrayMut::new(VALUES, 1..);
    let mut CODE = [b' '; CDELEN as usize];
    let mut TYPE = [b' '; 1];
    let mut VARNAM = [b' '; NAMLEN as usize];
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
        CHKIN(b"BODVCD", ctx)?;
    }

    //
    // Construct the variable name from BODY and ITEM.
    //
    fstr::assign(&mut VARNAM, b"BODY");

    INTSTR(BODYID, &mut CODE, ctx);
    SUFFIX(&CODE, 0, &mut VARNAM);
    SUFFIX(b"_", 0, &mut VARNAM);
    SUFFIX(ITEM, 0, &mut VARNAM);

    //
    // Make sure the item is present in the kernel pool.
    //
    DTPOOL(&VARNAM, &mut FOUND, DIM, &mut TYPE, ctx)?;

    if !FOUND {
        SETMSG(
            b"The variable # could not be found in the kernel pool.",
            ctx,
        );
        ERRCH(b"#", &VARNAM, ctx);
        SIGERR(b"SPICE(KERNELVARNOTFOUND)", ctx)?;
        CHKOUT(b"BODVCD", ctx)?;
        return Ok(());
    }

    //
    // Make sure the item's data type is numeric.
    //
    if fstr::ne(&TYPE, b"N") {
        SETMSG(
            b"The data associated with variable # are not of numeric type.",
            ctx,
        );
        ERRCH(b"#", &VARNAM, ctx);
        SIGERR(b"SPICE(TYPEMISMATCH)", ctx)?;
        CHKOUT(b"BODVCD", ctx)?;
        return Ok(());
    }

    //
    // Make sure there's enough room in the array VALUES to hold
    // the requested data.
    //
    if (MAXN < *DIM) {
        SETMSG(b"The data array associated with variable # has dimension #, which is larger than the available space # in the output array.", ctx);
        ERRCH(b"#", &VARNAM, ctx);
        ERRINT(b"#", *DIM, ctx);
        ERRINT(b"#", MAXN, ctx);
        SIGERR(b"SPICE(ARRAYTOOSMALL)", ctx)?;
        CHKOUT(b"BODVCD", ctx)?;
        return Ok(());
    }

    //
    // Grab the values.  We know at this point they're present in
    // the kernel pool, so we don't check the FOUND flag.
    //
    GDPOOL(
        &VARNAM,
        1,
        MAXN,
        DIM,
        VALUES.as_slice_mut(),
        &mut FOUND,
        ctx,
    )?;

    CHKOUT(b"BODVCD", ctx)?;
    Ok(())
}
