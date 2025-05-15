//
// GENERATED FILE
//

use super::*;
use crate::SpiceContext;
use f2rust_std::*;

const IDWLEN: i32 = 8;

/// Get file architecture and type from ID word
///
/// Extract the architecture and type of a SPICE binary kernel file
/// from a file ID word.
///
/// # Brief I/O
///
/// ```text
///  VARIABLE  I/O  DESCRIPTION
///  --------  ---  --------------------------------------------------
///  IDWORD     I   The IDWORD to be examined.
///  ARCH       O   The file architecture DAS or DAF.
///  TYPE       O   The type of the file.
/// ```
///
/// # Detailed Input
///
/// ```text
///  IDWORD   is the ID word from a SPICE binary kernel file or a
///           text version of a binary kernel file whose
///           architecture and type are to be extracted.
/// ```
///
/// # Detailed Output
///
/// ```text
///  ARCH     is the file architecture used to store the data in
///           a SPICE binary kernel file. If the architecture cannot
///           be extracted or is not recognized the value '?' is
///           returned.
///
///           The possible architectures are:
///
///              ASC -- An ASCII text file.
///              DAF -- A DAF based file.
///              DAS -- A DAS based file.
///              KPL -- Kernel Pool File (i.e., a text kernel)
///              TXT -- An ASCII text file.
///              TE1 -- Text E-Kernel type 1.
///
///
///  TYPE     is the type of the SPICE file. If the type can not be
///           extracted or if it is blank, the value '?' is
///           returned.
///
///           The type can only be extracted by this routine if
///           the ID word follows the convention
///
///              <architecture>/<type>
///
///           where <architecture> is one of the file architectures
///           specified above, and
///
///              <type> = 'xxxx'
///
///           where 'xxxx' represents a four character mnemonic or
///           code for the file type.
///
///           This subroutine does not do any checking of the file
///           types. If a valid architecture is found and the type
///           is non-blank, that is what will be returned. It is up
///           to a higher level authority to determine whether a type
///           is valid.
/// ```
///
/// # Exceptions
///
/// ```text
///  1)  If the variable ID word is blank, both the architecture and
///      type will be unknown, specified by '?'.
/// ```
///
/// # Particulars
///
/// ```text
///  This subroutine is a support utility routine that attempts
///  to extract the architecture and type of a file from its ID word.
///  It may not be possible to determine the type of the file from the
///  ID word alone. Older files which contain the ID words 'NAIF/NIP',
///  or 'NAIF/DAF' do not have sufficient information in the ID word to
///  determine the type of the file. A type for the ID word 'NAIF/DAS'
///  is always 'PRE ', since files with this ID word were pre-release
///  DAS files.
///
///  A file architecture can always be extracted from a valid SPICE
///  ID word.
///
///  This subroutine and the subroutine GETFAT (get file architecture
///  and type) are intimately related. Whenever one of them is modified
///  the other should be checked to see if the modifications affect it.
///  Whenever a new architecture is added, both of the subroutines are
///  affected.
/// ```
///
/// # Examples
///
/// ```text
///  Suppose you wish to write a single routine for converting files
///  between text and binary formats. You can use this routine to
///  determine the architecture and type of the file and then pass the
///  file to the appropriate low level file conversion routine to
///  handle the actual conversion.
///
///     CALL IDW2AT ( IDWORD, ARCH, TYPE )
///
///     IF ( ARCH .EQ. 'DAF' ) THEN
///
///        convert a DAF file
///
///     ELSE IF ( ARCH .EQ. 'DAS' ) THEN
///
///        convert a DAS file
///
///     ELSE
///
///        WRITE(*,*) 'File architecture not supported.'
///
///     END IF
/// ```
///
/// # Author and Institution
///
/// ```text
///  J. Diaz del Rio    (ODC Space)
///  K.R. Gehringer     (JPL)
/// ```
///
/// # Version
///
/// ```text
/// -    SPICELIB Version 2.1.0, 20-AUG-2021 (JDR)
///
///         Added IMPLICIT NONE statement.
///
///         Edited the header to comply with NAIF standard.
///
/// -    SPICELIB Version 2.0.0, 26-OCT-1995 (KRG)
///
///         Changed the Version line from "Beta" to "SPICELIB" for the
///         current revisions. The subroutine was already in SPICELIB,
///         but the Version line said "Beta."
///
///         Added several new architectures:
///
///            KPL -- Kernel Pool File (i.e., a text kernel)
///            TXT -- An ASCII text file.
///            ASC -- An ASCII text file.
///            TE1 -- Text E-Kernel type 1.
///
///         Changed the response foe the ID word 'NAIF/DAS' to be
///         consistent with GETFAT. It now sets the architecture to 'DAS'
///         and the type to 'PRE', for pre-release version.
///
/// -    SPICELIB Version 1.0.0, 30-SEP-1993 (KRG)
/// ```
///
/// # Revisions
///
/// ```text
/// -     SPICELIB Version 2.0.0, 26-OCT-1995 (KRG)
///
///          Changed the Version line from "Beta" to "SPICELIB" for the
///          current revisions. The subroutine was already in SPICELIB,
///          but the Version line said "Beta."
///
///          Added several new architectures:
///
///             KPL -- Kernel Pool File (i.e., a text kernel)
///             TXT -- An ASCII text file.
///             ASC -- An ASCII text file.
///             TE1 -- Text E-Kernel type 1.
///
///          Changed the response foe the ID word 'NAIF/DAS' to be
///          consistent with GETFAT. It now sets the architecture to 'DAS'
///          and the type to 'PRE', for pre-release version.
/// ```
pub fn idw2at(
    ctx: &mut SpiceContext,
    idword: &str,
    arch: &mut str,
    type_: &mut str,
) -> crate::Result<()> {
    IDW2AT(
        idword.as_bytes(),
        fstr::StrBytes::new(arch).as_mut(),
        fstr::StrBytes::new(type_).as_mut(),
        ctx.raw_context(),
    )?;
    ctx.handle_errors()?;
    Ok(())
}

//$Procedure IDW2AT ( Get file architecture and type from ID word )
pub fn IDW2AT(
    IDWORD: &[u8],
    ARCH: &mut [u8],
    TYPE: &mut [u8],
    ctx: &mut Context,
) -> f2rust_std::Result<()> {
    let mut PART1 = [b' '; IDWLEN as usize];
    let mut PART2 = [b' '; IDWLEN as usize];
    let mut SLASH: i32 = 0;

    //
    // Spicelib Routines
    //

    //
    // Set the length of a SPICE file ID word.
    //
    //
    // Local Variables
    //

    //
    // Standard obligatory error handling stuff.
    //
    if RETURN(ctx) {
        return Ok(());
    } else {
        CHKIN(b"IDW2AT", ctx)?;
    }
    //
    // Check to see if we got a blank string for the ID word. If we did,
    // set the architecture and type to unknown.
    //

    if fstr::eq(IDWORD, b" ") {
        fstr::assign(ARCH, b"?");
        fstr::assign(TYPE, b"?");

        CHKOUT(b"IDW2AT", ctx)?;
        return Ok(());
    }
    //
    // Initialize the temporary storage variables that we use.
    //
    fstr::assign(&mut PART1, b" ");
    fstr::assign(&mut PART2, b" ");
    //
    // See if we can get the architecture and type from the ID word.
    //
    // Look for a '/' in the string. If we can't find it, we don't
    // recognize the architecture or the type, so set the architecture
    // and type to unknown.
    //
    SLASH = POS(IDWORD, b"/", 1);

    if (SLASH == 0) {
        fstr::assign(ARCH, b"?");
        fstr::assign(TYPE, b"?");

        CHKOUT(b"IDW2AT", ctx)?;
        return Ok(());
    }
    //
    // The part before the slash is the architecture or the word 'NAIF'
    // in older files and the part after the slash is the type of file or
    // the architecture in older files.
    //
    fstr::assign(&mut PART1, fstr::substr(IDWORD, 1..=(SLASH - 1)));
    fstr::assign(&mut PART2, fstr::substr(IDWORD, (SLASH + 1)..));
    //
    //  Let's now do some testing to try and figure out what's going on.
    //
    //  First we look for the information in the ID word format:
    //
    //     <architecture>/<type>,
    //
    // then we look for the things that begin with the word 'NAIF'
    //
    if fstr::eq(&PART1, b"DAF") {
        //
        // We have a DAF file, so set the architecture and type.
        //
        fstr::assign(ARCH, b"DAF");

        if fstr::ne(&PART2, b" ") {
            fstr::assign(TYPE, &PART2);
        } else {
            fstr::assign(TYPE, b"?");
        }
    } else if fstr::eq(&PART1, b"DAS") {
        //
        // We have a DAS file, so set the architecture and type.
        //
        fstr::assign(ARCH, b"DAS");

        if fstr::ne(&PART2, b" ") {
            fstr::assign(TYPE, &PART2);
        } else {
            fstr::assign(TYPE, b"?");
        }
    } else if fstr::eq(&PART1, b"TXT") {
        //
        // We have an ASCII text file, so set the architecture and type.
        //
        fstr::assign(ARCH, b"TXT");

        if fstr::ne(&PART2, b" ") {
            fstr::assign(TYPE, &PART2);
        } else {
            fstr::assign(TYPE, b"?");
        }
    } else if fstr::eq(&PART1, b"ASC") {
        //
        // We have an ASCII text file, so set the architecture and type.
        //
        fstr::assign(ARCH, b"TXT");

        if fstr::ne(&PART2, b" ") {
            fstr::assign(TYPE, &PART2);
        } else {
            fstr::assign(TYPE, b"?");
        }
    } else if fstr::eq(&PART1, b"KPL") {
        //
        // We have a kernel pool file, so set the architecture and type.
        //
        fstr::assign(ARCH, b"KPL");

        if fstr::ne(&PART2, b" ") {
            fstr::assign(TYPE, &PART2);
        } else {
            fstr::assign(TYPE, b"?");
        }
    } else if fstr::eq(&PART1, b"NAIF") {
        //
        // We have a DAF (or NIP, these are equivalent) or DAS file,
        // identified by the value of PART2, but we have no idea what the
        // type is, unless the file is a DAS file, in which case it is a
        // pre-release EK file, since these are the only DAS files which
        // used the 'NAIF/DAS' ID word.
        //
        // First, we determine the architecture from PART2, then if it is
        // DAF or NIP, we give up on the type. As mentioned above, if
        // PART2 contains DAS, we know a priori the type of the file.
        //
        if (fstr::eq(&PART2, b"DAF") || fstr::eq(&PART2, b"NIP")) {
            fstr::assign(ARCH, b"DAF");
            fstr::assign(TYPE, b"?");
        } else if fstr::eq(&PART2, b"DAS") {
            fstr::assign(ARCH, b"DAS");
            fstr::assign(TYPE, b"PRE");
        } else {
            fstr::assign(ARCH, b"?");
            fstr::assign(TYPE, b"?");
        }
    } else {
        fstr::assign(ARCH, b"?");
        fstr::assign(TYPE, b"?");
    }

    CHKOUT(b"IDW2AT", ctx)?;
    Ok(())
}
