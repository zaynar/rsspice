//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

/// Get Explanation for Short Error Message
///
/// Return the explanation of a short error message.
///
/// # Required Reading
///
/// * [ERROR](crate::required_reading::error)
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  MSG        I   A short error message.
///  EXPL       O   The explanation of the short error message.
/// ```
///
/// # Detailed Input
///
/// ```text
///  MSG      is a "short" error message.
///           MSG indicates the type of error that has occurred.
///
///           The exact format that MSG must follow is
///           described in the required reading file, error.req.
/// ```
///
/// # Detailed Output
///
/// ```text
///  EXPL     is a character string containing an one-line
///           explanation of the short error message, MSG.
///
///           If there is no explanatory text corresponding
///           to the input string, MSG, EXPL is blank.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  This routine does not detect any errors.
///
///      However, this routine is part of the interface to the SPICELIB
///      error handling mechanism. For this reason, this routine does
///      not participate in the trace scheme, even though it has
///      external references.
/// ```
///
/// # Examples
///
/// ```text
///  C
///  C     We want to find the explanation corresponding to
///  C     the short message, SPICE(ZERORADIUS) :
///  C
///
///         CALL EXPLN ( SPICE(ZERORADIUS), EXPL )
///
///
///  Now, EXPL  =
///
///  'Invalid Radius--Equatorial or Polar Radius is Zero'
/// ```
///
/// # Author and Institution
///
/// ```text
///  N.J. Bachman       (JPL)
///  J. Diaz del Rio    (ODC Space)
///  B.V. Semenov       (JPL)
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
/// -    SPICELIB Version 1.0.2, 18-APR-2014 (BVS)
///
///         Minor header edits.
///
/// -    SPICELIB Version 1.0.1, 10-MAR-1992 (WLT)
///
///         Comment section for permuted index source lines was added
///         following the header.
///
/// -    SPICELIB Version 1.0.0, 31-JAN-1990 (NJB)
/// ```
///
/// # Revisions
///
/// ```text
/// -    Beta Version 1.1.0, 27-OCT-1988 (NJB)
///
///         Removed code used to create upper case, left-justified
///         copy of the short error message. The resulting message
///         was not used.
/// ```
pub fn expln(msg: &str, expl: &mut str) {
    EXPLN(msg.as_bytes(), fstr::StrBytes::new(expl).as_mut());
}

//$Procedure EXPLN ( Get Explanation for Short Error Message )
pub fn EXPLN(MSG: &[u8], EXPL: &mut [u8]) {
    //
    // Executable Code:
    //

    //
    // Note: the short error messages should be ordered
    // alphabetically.
    //

    if fstr::eq(MSG, b"SPICE(BADENDPOINTS)") {
        fstr::assign(
            EXPL,
            b"Invalid Endpoints--Left Endpoint Exceeds Right Endpoint",
        );
    } else if fstr::eq(MSG, b"SPICE(BADGEFVERSION)") {
        fstr::assign(EXPL, b"Version Identification of GEF File is Invalid");
    } else if fstr::eq(MSG, b"SPICE(BLANKMODULENAME)") {
        fstr::assign(EXPL, b"A blank string was used as a module name");
    } else if fstr::eq(MSG, b"SPICE(BOGUSENTRY)") {
        fstr::assign(EXPL, b"This Entry Point Contains No Executable Code");
    } else if fstr::eq(MSG, b"SPICE(CELLTOOSMALL)") {
        fstr::assign(EXPL, b"Cardinality of Output Cell is Too Small");
    } else if fstr::eq(MSG, b"SPICE(CLUSTERWRITEERROR)") {
        fstr::assign(EXPL, b"Error Writing to Ephemeris File");
    } else if fstr::eq(MSG, b"SPICE(DATATYPENOTRECOG)") {
        fstr::assign(
            EXPL,
            b"Unrecognized Data Type Specification was Encountered",
        );
    } else if fstr::eq(MSG, b"SPICE(DATEEXPECTED)") {
        fstr::assign(
            EXPL,
            b"The Value in the Kernel File was Expected to be a date.",
        );
    } else if fstr::eq(MSG, b"SPICE(DEVICENAMETOOLONG)") {
        fstr::assign(EXPL, b"Name of Device Exceeds 128-Character Limit");
    } else if fstr::eq(MSG, b"SPICE(EMBEDDEDBLANK)") {
        fstr::assign(
            EXPL,
            b"Invalid embedded blank was found in character string",
        );
    } else if fstr::eq(MSG, b"SPICE(FILEALREADYOPEN)") {
        fstr::assign(EXPL, b"File Open Failed Because the File was Already Open");
    } else if fstr::eq(MSG, b"SPICE(FILEOPENFAILED)") {
        fstr::assign(EXPL, b"An Attempt to Open a File Failed");
    } else if fstr::eq(MSG, b"SPICE(FILEREADFAILED)") {
        fstr::assign(EXPL, b"An Attempt to Read a File Failed");
    } else if fstr::eq(MSG, b"SPICE(FILEWRITEFAILED)") {
        fstr::assign(EXPL, b"An Attempt to Write a File Failed");
    } else if fstr::eq(MSG, b"SPICE(INCOMPATIBLEUNITS)") {
        fstr::assign(EXPL, b"The Input and Output Units are Incompatible");
    } else if fstr::eq(MSG, b"SPICE(INVALIDACTION)") {
        fstr::assign(EXPL, b"An Invalid Action Value Was Supplied");
    } else if fstr::eq(MSG, b"SPICE(INVALIDARGUMENT)") {
        fstr::assign(EXPL, b"An Invalid Function Argument was Supplied");
    } else if fstr::eq(MSG, b"SPICE(INVALIDCHECKOUT)") {
        fstr::assign(
            EXPL,
            b"Checkout Was Attempted When No Routines Were Checked In",
        );
    } else if fstr::eq(MSG, b"SPICE(INVALIDCLUSTERNUM)") {
        fstr::assign(
            EXPL,
            b"Invalid Cluster Number -- Cluster Numbers Must Exceed 1 ",
        );
    } else if fstr::eq(MSG, b"SPICE(INVALIDEPOCH)") {
        fstr::assign(EXPL, b"An Invalid Epoch Type Specification Was Supplied");
    } else if fstr::eq(MSG, b"SPICE(INVALIDINDEX)") {
        fstr::assign(
            EXPL,
            b"There Is No Element Corresponding to the Supplied Index",
        );
    } else if fstr::eq(MSG, b"SPICE(INVALIDTIMESTRING)") {
        fstr::assign(EXPL, b"Time String Could Not Be Parsed");
    } else if fstr::eq(MSG, b"SPICE(INVALIDLISTITEM)") {
        fstr::assign(EXPL, b"An Invalid Item Was Found in a List");
    } else if fstr::eq(MSG, b"SPICE(INVALIDMSGTYPE)") {
        fstr::assign(EXPL, b"An Invalid Error Message Type Was Specified");
    } else if fstr::eq(MSG, b"SPICE(INVALIDOPERATION)") {
        fstr::assign(EXPL, b"An Invalid Operation Value Was Supplied");
    } else if fstr::eq(MSG, b"SPICE(INVALIDOPTION)") {
        fstr::assign(EXPL, b"An Invalid Option Value Was Supplied");
    } else if fstr::eq(MSG, b"SPICE(INVALIDTIMEFORMAT)") {
        fstr::assign(
            EXPL,
            b"Specification of Time String Format Was Not Recognized",
        );
    } else if fstr::eq(MSG, b"SPICE(KERNELVARNOTFOUND)") {
        fstr::assign(EXPL, b"The Variable Was not Found in the Kernel Pool.");
    } else if fstr::eq(MSG, b"SPICE(NAMETABLEFULL)") {
        fstr::assign(
            EXPL,
            b"No Further Symbols Can be Inserted; the Name Table is Full",
        );
    } else if fstr::eq(MSG, b"SPICE(NOFREELOGICALUNIT)") {
        fstr::assign(EXPL, b"No More Logical Units are Available for Allocation");
    } else if fstr::eq(MSG, b"SPICE(NOINTERVAL)") {
        fstr::assign(
            EXPL,
            b"Window Does Not Contain Interval Corresponding to the Supplied Index",
        );
    } else if fstr::eq(MSG, b"SPICE(NOSEGMENT)") {
        fstr::assign(EXPL, b"No Applicable Segment Found in Ephemeris File");
    } else if fstr::eq(MSG, b"SPICE(NOSUCHSYMBOL)") {
        fstr::assign(EXPL, b"The Symbol Does Not Exist in the Symbol Table");
    } else if fstr::eq(MSG, b"SPICE(NOTDISTINCT)") {
        fstr::assign(EXPL, b"The Elements Must Be Distinct");
    } else if fstr::eq(MSG, b"SPICE(NUMBEREXPECTED)") {
        fstr::assign(
            EXPL,
            b"The Value in the Kernel File was Expected to be a Number.",
        );
    } else if fstr::eq(MSG, b"SPICE(POINTERTABLEFULL)") {
        fstr::assign(
            EXPL,
            b"No Further Symbols Can be Inserted; the Pointer Table is Full",
        );
    } else if fstr::eq(MSG, b"SPICE(REFNOTREC)") {
        fstr::assign(EXPL, b"A Reference Frame Specification was Not Recognized");
    } else if fstr::eq(MSG, b"SPICE(SETEXCESS)") {
        fstr::assign(
            EXPL,
            b"Cardinality of Set Is Too Small to Contain Result of the Requested Operation",
        );
    } else if fstr::eq(MSG, b"SPICE(TOOMANYFILESOPEN)") {
        fstr::assign(
            EXPL,
            b"The SPICELIB Limit for Number of Open Files Has Already Been Reached",
        );
    } else if fstr::eq(MSG, b"SPICE(TRACEBACKOVERFLOW)") {
        fstr::assign(
            EXPL,
            b"No More Entries Can Be Added to the Traceback Representation",
        );
    } else if fstr::eq(MSG, b"SPICE(UNITSNOTREC)") {
        fstr::assign(EXPL, b"The Input or Output Units Were Not Recognized");
    } else if fstr::eq(MSG, b"SPICE(UNMATCHENDPTS)") {
        fstr::assign(EXPL, b"Window Does Not Have an Even Number of Endpoints");
    } else if fstr::eq(MSG, b"SPICE(VALUETABLEFULL)") {
        fstr::assign(
            EXPL,
            b"No Further Symbols Can be Inserted; the Value Table is Full",
        );
    } else if fstr::eq(MSG, b"SPICE(WINDOWEXCESS)") {
        fstr::assign(
            EXPL,
            b"Cardinality of Window Is Too Small to Contain Result of the Requested Operation",
        );
    } else if fstr::eq(MSG, b"SPICE(WINDOWTOOSMALL)") {
        fstr::assign(EXPL, b"Cardinality of Output Window is Too Small");
    } else if fstr::eq(MSG, b"SPICE(WRITEERROR)") {
        fstr::assign(EXPL, b"An Attempt to write to a specified unit failed.");
    } else if fstr::eq(MSG, b"SPICE(ZERORADIUS)") {
        fstr::assign(EXPL, b"Invalid Radius--Equatorial or Polar Radius is Zero");
    } else if fstr::eq(MSG, b"SPICE(ZEROVECTOR)") {
        fstr::assign(EXPL, b"Input Vector is the Zero Vector");
    } else if fstr::eq(MSG, b"SPICE(ZEROAXISLENGTH)") {
        fstr::assign(EXPL, b"Input Axis Length is Zero");
    } else {
        fstr::assign(EXPL, b" ");
    }
}
