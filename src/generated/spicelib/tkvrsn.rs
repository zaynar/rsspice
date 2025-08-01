//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Toolkit version strings
///
/// Return the latest version string of a given item such as the
/// Toolkit or a routine name.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  ITEM       I   Item for which a version string is desired.
///  VERSTR     O   Version string.
/// ```
///
/// # Detailed Input
///
/// ```text
///  ITEM     is the item for which a version string is to be returned.
///           ITEM may be 'TOOLKIT', entry point names, or program
///           names. ITEM is case insensitive.
///
///           Currently, the only ITEM supported is 'TOOLKIT' and it
///           will return the toolkit version number.
///
///           Any other ITEM will return 'No version found.'
/// ```
///
/// # Detailed Output
///
/// ```text
///  VERSTR   is the latest version string for the specified ITEM.
///
///           If ITEM is not one of the items having a version, the
///           value 'No version found.' will be returned.
/// ```
///
/// # Exceptions
///
/// ```text
///  Error free.
///
///  1)  If the ITEM whose version string is requested is not
///      recognized, the string 'No version found.' is returned.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you want to find out the recent Toolkit configuration
///  version number. Using the code fragment below:
///
///  CHARACTER*(80)        VERSN
///
///  CALL TKVRSN ( 'TOOLKIT', VERSN )
///
///  The variable VERSN would contain a string similar to the one
///  shown below:
///
///     'N0035'
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
///  H.A. Neilan        (JPL)
///  B.V. Semenov       (JPL)
///  W.L. Taber         (JPL)
///  F.S. Turner        (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 3.16.0, 31-DEC-2021 (WLT)
///
///         Version update, N0067
///
/// -    SPICELIB Version 3.15.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 3.14.0, 05-APR-2017 (BVS)
///
///         Version update, N0066
///
/// -    SPICELIB Version 3.13.0, 15-JUL-2014 (BVS)
///
///         Version update, N0065
///
/// -    SPICELIB Version 3.12.0, 09-JUN-2010 (BVS)
///
///         Version update, N0064
///
/// -    SPICELIB Version 3.11.0, 15-APR-2009 (BVS)
///
///         Version update, N0063
///
/// -    SPICELIB Version 3.10.0, 04-MAR-2008 (BVS)
///
///         Version update, N0062
///
/// -    SPICELIB Version 3.9.0, 27-NOV-2006 (BVS)
///
///         Version update, N0061
///
/// -    SPICELIB Version 3.8.0, 16-DEC-2005 (BVS)
///
///         Version update, N0060
///
/// -    SPICELIB Version 3.7.0, 17-NOV-2005 (BVS)
///
///         Version update, N0059
///
/// -    SPICELIB Version 3.6.0, 11-JAN-2005 (BVS)
///
///         Version update, N0058
///
/// -    SPICELIB Version 3.5.0, 02-MAR-2004 (BVS)
///
///         Version update, N0057
///
/// -    SPICELIB Version 3.4.0, 30-JUL-2003 (BVS)
///
///         Version update, N0056
///
/// -    SPICELIB Version 3.3.0, 26-FEB-2003 (BVS)
///
///         Version update, N0055
///
/// -    SPICELIB Version 3.2.0, 13-DEC-2002 (BVS)
///
///         Version update, N0054
///
/// -    SPICELIB Version 3.1.0, 05-SEP-2002 (BVS)
///
///         Version update, N0053
///
/// -    SPICELIB Version 3.0.0, 06-FEB-2002 (FST)
///
///         Version update, N0052a
///
/// -    SPICELIB Version 2.9.0, 17-JAN-2002 (WLT)
///
///         Version update, N0052
///
/// -    SPICELIB Version 2.8.0, 07-APR-2000 (WLT)
///
///         Version update, N0051
///
/// -    SPICELIB Version 2.7.0, 06-OCT-1999 (WLT)
///
///         Version update, N0050
///
/// -    SPICELIB Version 2.6.0, 04-SEP-1998 (WLT)
///
///         Version update, N0049
///
/// -    SPICELIB Version 2.5.0, 01-MAY-1998 (WLT)
///
///         Version update, N0048
///
/// -    SPICELIB Version 2.4.0, 31-JUL-1997 (WLT)
///
///         Version update, N0047
///
/// -    SPICELIB Version 2.3.0, 27-JAN-1997 (WLT)
///
///         Version update, N0046
///
/// -    SPICELIB Version 2.2.0, 15-OCT-1996 (WLT)
///
///         Version update, N0045
///
/// -    SPICELIB Version 2.1.0, 26-AUG-1996 (WLT)
///
///         Version update, N0044
///
/// -    SPICELIB Version 2.0.0, 09-MAY-1996 (KRG)
///
///         Removed the check of the SPICELIB function RETURN. This
///         routine is called by the error handling after an error
///         has been signaled to get the toolkit version, so it
///         cannot return on entry after an error.
///
///         The calls to CHKIN and CHKOUT have also been removed to
///         completely isolate this subroutine from the error handling.
///
///         Version update, N0043.
///
/// -    SPICELIB Version 1.7.0, 02-JAN-1996 (WLT)
///
///         Version update, N0042.
///
/// -    SPICELIB Version 1.6.0, 28-SEP-1995 (HAN)
///
///         Version update, N0041.
///
/// -    SPICELIB Version 1.5.0, 19-AUG-1995 (HAN)
///
///         Version update, N0040.
///
/// -    SPICELIB Version 1.4.0, 05-JUN-1995 (HAN)
///
///         Version update, N0039.
///
/// -    SPICELIB Version 1.3.0, 28-MAR-1995 (HAN)
///
///         Version update, N0038.
///
/// -    SPICELIB Version 1.2.0, 23-DEC-1994 (HAN)
///
///         Version update, N0037.
///
/// -    SPICELIB Version 1.1.0, 31-OCT-1994 (HAN)
///
///         Version update, N0036.
///
/// -    SPICELIB Version 1.0.0, 23-AUG-1994 (HAN)
/// ```
pub fn tkvrsn(item: &str, verstr: &mut str) {
    TKVRSN(item.as_bytes(), fstr::StrBytes::new(verstr).as_mut());
}

//$Procedure TKVRSN ( Toolkit version strings )
pub fn TKVRSN(ITEM: &[u8], VERSTR: &mut [u8]) {
    //
    // SPICELIB functions
    //

    //
    // At the current time only the TOOLKIT version number is
    // defined.
    //
    if EQSTR(ITEM, b"TOOLKIT") {
        fstr::assign(VERSTR, b"N0067");
    } else {
        fstr::assign(VERSTR, b"No version found.");
    }
}
