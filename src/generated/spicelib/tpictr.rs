//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Create a Time Format Picture
///
/// Create a time format picture suitable for use by the routine
/// TIMOUT from a given sample time string.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  SAMPLE     I   is a sample date time string
///  PICTUR     O   is a format picture that describes SAMPLE
///  OK         O   indicates success or failure to parse SAMPLE
///  ERRMSG     O   a diagnostic returned if SAMPLE cannot be parsed
/// ```
///
/// # Detailed Input
///
/// ```text
///  SAMPLE   is a representative time string that to use
///           as a model to format time strings.
/// ```
///
/// # Detailed Output
///
/// ```text
///  PICTUR   is a format picture suitable for use with the SPICE
///           routine TIMOUT. This picture when used to format
///           the appropriate  epoch via TIMOUT will yield the same
///           time components in the same order as the components
///           in SAMPLE.
///
///           Picture should be declared to be at least 80 characters
///           in length. If Picture is not sufficiently large
///           to contain the format picture, the picture will
///           be truncated on the right.
///
///  OK       is a logical flag. If all of the components of SAMPLE
///           are recognizable, OK will be returned with the value
///           .TRUE. If some part of PICTUR cannot be parsed,
///           OK will be returned with the value .FALSE.
///
///  ERRMSG   is a diagnostic message  that indicates what part of
///           SAMPLE was not recognizable. If SAMPLE can be
///           successfully parsed, OK will be .TRUE. and ERRMSG will
///           be returned as a blank string. If ERRMSG does not
///           have sufficient room (up to 400 characters) to
///           contain the full message, the message will be truncated
///           on the right.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  All problems with the inputs are reported via OK and ERRMSG.
///
///  2)  If a format picture can not be created from the sample
///      time string, PICTUR is returned as a blank string.
/// ```
///
/// # Particulars
///
/// ```text
///  Although the routine TIMOUT provides SPICE users with a great
///  deal of flexibility in formatting time strings, users must
///  master the means by which a time picture is constructed
///  suitable for use by TIMOUT.
///
///  This routine allows SPICE users to supply a sample time string
///  from which a corresponding time format picture can be created,
///  freeing users from the task of mastering the intricacies of
///  the routine TIMOUT.
///
///  Note that TIMOUT can produce many time strings whose patterns
///  can not be discerned by this routine. When such outputs are
///  called for, the user must consult TIMOUT and construct the
///  appropriate format picture "by hand." However, these exceptional
///  formats are not widely used and are not generally recognizable
///  to an uninitiated reader.
/// ```
///
/// # Examples
///
/// ```text
///  The numerical results shown for this example may differ across
///  platforms. The results depend on the SPICE kernels used as input,
///  the compiler and supporting libraries, and the machine specific
///  arithmetic implementation.
///
///  1) Given a sample with the format of the UNIX date string
///     local to California, create a SPICE time picture for use
///     in TIMOUT.
///
///     Using that SPICE time picture, convert a series of ephemeris
///     times to that picture format.
///
///     Use the LSK kernel below to load the leap seconds and time
///     constants required for the conversions.
///
///        naif0012.tls
///
///
///     Example code begins here.
///
///
///           PROGRAM TPICTR_EX1
///           IMPLICIT NONE
///
///     C
///     C     Local parameters.
///     C
///           INTEGER               ERRLEN
///           PARAMETER           ( ERRLEN  = 400 )
///
///           INTEGER               TIMLEN
///           PARAMETER           ( TIMLEN  = 64  )
///
///     C
///     C     Local variables
///     C
///           CHARACTER*(ERRLEN)    ERR
///           CHARACTER*(TIMLEN)    PICTUR
///           CHARACTER*(TIMLEN)    SAMPLE
///           CHARACTER*(TIMLEN)    TIMSTR
///           CHARACTER*(TIMLEN)    UTCSTR
///
///           DOUBLE PRECISION      ET
///
///           LOGICAL               OK
///
///     C
///     C     Load LSK file.
///     C
///           CALL FURNSH ( 'naif0012.tls' )
///
///     C
///     C     Create the required time picture.
///     C
///           SAMPLE = 'Thu Oct 01 11:11:11 PDT 1111'
///
///           CALL TPICTR ( SAMPLE, PICTUR, OK, ERR )
///
///           IF ( .NOT. OK ) THEN
///
///              WRITE(*,*) 'Invalid time picture.'
///              WRITE(*,*) ERR
///
///           ELSE
///
///     C
///     C        Convert the input UTC time to ephemeris time.
///     C
///              UTCSTR = '24 Mar 2018  16:23:00 UTC'
///              CALL STR2ET ( UTCSTR, ET )
///
///     C
///     C         Now convert ET to the desired output format.
///     C
///               CALL TIMOUT ( ET, PICTUR, TIMSTR )
///               WRITE (*,*) 'Sample format: ', SAMPLE
///               WRITE (*,*) 'Time picture : ', PICTUR
///               WRITE (*,*)
///               WRITE (*,*) 'Input UTC    : ', UTCSTR
///               WRITE (*,*) 'Output       : ', TIMSTR
///
///           END IF
///
///           END
///
///
///     When this program was executed on a Mac/Intel/gfortran/64-bit
///     platform, the output was:
///
///
///      Sample format: Thu Oct 01 11:11:11 PDT 1111
///      Time picture : Wkd Mon DD HR:MN:SC PDT YYYY ::UTC-7
///
///      Input UTC    : 24 Mar 2018  16:23:00 UTC
///      Output       : Sat Mar 24 09:23:00 PDT 2018
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  W.L. Taber         (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 1.1.0, 25-AUG-2021 (JDR)
///
///         Changed output argument name ERROR to ERRMSG for consistency
///         with other routines.
///
///         Edited the header to comply with NAIF standard.
///         Converted the existing code fragments into complete example
///         and added reference to required LSK.
///
/// -    SPICELIB Version 1.0.1, 16-MAR-1999 (WLT)
///
///         Corrected a minor spelling error in the header comments.
///
/// -    SPICELIB Version 1.0.0, 10-AUG-1996 (WLT)
/// ```
pub fn tpictr(
    ctx: &mut SpiceContext,
    sample: &str,
    pictur: &mut str,
    ok: &mut bool,
    errmsg: &mut str,
) {
    TPICTR(
        sample.as_bytes(),
        fstr::StrBytes::new(pictur).as_mut(),
        ok,
        fstr::StrBytes::new(errmsg).as_mut(),
        ctx.raw_context(),
    );
}

//$Procedure TPICTR ( Create a Time Format Picture )
pub fn TPICTR(
    SAMPLE: &[u8],
    PICTUR: &mut [u8],
    OK: &mut bool,
    ERRMSG: &mut [u8],
    ctx: &mut Context,
) {
    let mut TYPE = [b' '; 5 as usize];
    let mut MODIFY = ActualCharArray::new(8, 1..=5);
    let mut TVEC = StackArray::<f64, 10>::new(1..=10);
    let mut NTVEC: i32 = 0;
    let mut MODS: bool = false;
    let mut SUCCES: bool = false;
    let mut YABBRV: bool = false;

    //
    // Local variables
    //

    //
    // This routine is really just a front for one aspect of
    // the routine TPARTV.
    //
    fstr::assign(ERRMSG, b" ");

    TPARTV(
        SAMPLE,
        TVEC.as_slice_mut(),
        &mut NTVEC,
        &mut TYPE,
        MODIFY.as_arg_mut(),
        &mut MODS,
        &mut YABBRV,
        &mut SUCCES,
        PICTUR,
        ERRMSG,
        ctx,
    );

    if fstr::eq(PICTUR, b" ") {
        *OK = false;
    } else {
        *OK = true;
        fstr::assign(ERRMSG, b" ");
    }
}
