//! #  Double Precision Array Files (DAF)
//!
//!  Last revised on 2017 MAR 22 by B. V. Semenov.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  The Double Precision Array File architecture stores arrays of double
//!    precision data. The SPICE SPKs, CKs, and binary PCKs use the DAF
//!    architecture.
//!
//!  
//!
//!
//!  
//! ###  Purpose
//!
//!  This document is a reference guide for the SPICE DAF subsystem. Here
//!    you'll find:
//!
//!  
//!
//! * A list of the subsystem's API (application programming interface)
//! routines---these are the routines that may be called directly by
//! SPICE-based user application code.
//!
//!  * Description of the DAF format.
//!
//!  * Discussions of concepts essential to understanding the correct use of the
//! DAF subsystem.
//!
//!     
//! ###  Intended Audience
//!
//!  This document is intended for users that require detailed knowledge of
//!    the DAF format, such as the SPK, binary PCK, and CK formats already
//!    mentioned. It is also intended for users of the SPICE library who wish
//!    to create their own DAF-based file formats.
//!
//!  Nominally, users of DAF will not need to understand the material
//!    presented in this document. For example, users of SPICE SPK and CKs who
//!    wish to read state vectors and pointing angles from those files will
//!    normally do so using only subroutines and programs designed specifically
//!    for those formats. These subroutines are documented in the NAIF [spk.req](crate::required_reading::spk)
//!    and [ck.req](crate::required_reading::ck) Required Reading files.
//!
//!  
//!
//!
//!  
//! ###  References
//!
//!  The following NAIF documents contain material closely related to the
//!    subject of this document.
//!
//!  
//!
//! *  1. SPK Required Reading ([spk.req](crate::required_reading::spk))
//!
//!  *  This document describes how the DAF structure is used in making the SPICE
//! Satellite Planet Kernel (SPK) containing trajectory and ephemeris data.
//!
//!  *  2. CK Required Reading ([ck.req](crate::required_reading::ck))
//!
//!  *  This document describes how the DAF structure is used in making the SPICE C
//! Kernel (CK) containing spacecraft or instrument orientation (pointing).
//!
//!  *  3. PCK Required Reading ([pck.req](crate::required_reading::pck))
//!
//!  *  This document describes how the DAF structure is used to store data in
//! binary SPICE PCK.
//!
//!  *  4. SPC Required Reading ([spc.req](crate::required_reading::spc))
//!
//!  *  This document describes how labels, descriptive text and other comments may
//! be added to or removed from a DAF.
//!
//!  *  5. SPY User's Guide
//!
//!  *  This document describes the use of the "spy" utility to check DAF-based SPK
//! integrity, dump SPK data, dump SPK records, dump the contents of the SPK
//! comment area. Standard Toolkit distributions do not include "spy," download
//! the executable and User's Guide at the URL
//! <http://naif.jpl.nasa.gov/naif/utilities.html>.
//!
//!     
//! #  Introduction
//!
//!  DAF---which stands for 'Double precision Array File'---is a file
//!    architecture that provides the advantages of arrays and direct access
//!    files without incurring the disadvantages of either one.
//!
//!  This architecture is supported by a set of subroutines, part of the NAIF
//!    Toolkit software library.
//!
//!  Only the SPICELIB Toolkits include the full set of DAF APIs; Fortran
//!    serves as the ground state language. The CSPICE, Icy (IDL), and Mice
//!    (Matlab) Toolkit have a subset of the API set. This document will note
//!    those routines available only in Fortran SPICELIB and those available in
//!    the other language distributions.
//!
//!  A DAF can contain any number of double precision arrays. Each of these
//!    arrays can contain an arbitrary number of elements. Because DAF files
//!    are intended to be portable, the DAF design requires that the array
//!    elements must be 'pure' double precision numbers. That is, they may not
//!    contain equivalenced or encoded integer or character values.
//!
//!  The DAF architecture assumes one byte size characters, four byte size
//!    integers, eight byte size double precision. DAFs created using SPICE
//!    libraries compiled with other size values may work if the relative size
//!    ratio between the data types remains 1-4-8.
//!
//!  The DAF subroutines in SPICE support the following operations:
//!
//!  
//!
//! *  1. Create, open, or close a DAF.
//!
//!  *  2. Add a new array to a file.
//!
//!  *  3. Locate an array within a file, either by index or by using descriptive
//! information about the array.
//!
//!  *  4. Access---that is, retrieve or update---any contiguous set of elements in an
//! array.
//!
//!  *  5. Convert a binary (direct access) DAF to an equivalent SPICE transfer file,
//! suitable for transfer to an environment using a form of binary
//! representation different from that of the source CPU.
//!
//!  *  6. Convert a DAF in SPICE transfer format into an equivalent binary file.
//!
//!  The last two functions make DAFs portable to any environment that
//!    supports ANSI Standard Fortran-77.
//!
//!  
//!
//!
//!  
//! ##  The DAF Format Concept
//!
//!  The DAF architecture design intends each array contained in a particular
//!    DAF possess a 'descriptive summary' of itself. The information making up
//!    the summary and the organization of the summary should be the same for
//!    each array in the DAF. The descriptive summary is composed of double
//!    precision and integer components. The number of double precision
//!    components, ND, and the number of integer components, NI, making up the
//!    array summaries, determine a particular format within the DAF
//!    architecture.
//!
//!  Values for ND and NI are fixed at the time a DAF is created. Any two
//!    DAFs that have the same values for ND and NI can be thought of as having
//!    the same 'format'. (This does not guarantee that the arrays in the files
//!    contain the same kinds of information, only that they could be stored in
//!    the same file.) The values selected for ND and NI must satisfy the
//!    following inequalities:
//!
//!  
//!
//! ```text
//!         (NI + 1)                     (Note that this is
//!    ND + -------- <= 125               integer division.
//!             2                         That is, (NI + 1)/2
//!                                       is rounded down to
//!                                       the nearest integer.)
//!  
//!    0 <= ND <= 124
//!  
//!    2 <= NI <= 250
//! ```
//!
//!  Each array stored in a DAF is 'described', in part, by ND double
//!    precision numbers and NI integer numbers, which are stored separately
//!    from the array. Most of the details of this 'description'---how many
//!    numbers are needed, and what they contain---are left to the designer of
//!    a specific DAF format.
//!
//!  The double precision numbers could include limits (the smallest and
//!    largest values in an array), a range of epochs throughout which the
//!    elements may be used, or statistics (the mean, median, and standard
//!    deviation of the elements).
//!
//!  The integer numbers could include contextual information (case number,
//!    identification codes for related objects or arrays) and conditional
//!    information (flags to indicate whether the array is unsorted, sorted by
//!    increasing or decreasing magnitude, or marked for deletion). Some
//!    integer numbers are used to keep track of the location of the array
//!    within the file.
//!
//!  Each array in a DAF is further described by NC characters of
//!    alphanumeric information. Examples of this alphanumeric information are
//!    producer names, archive codes, historical information, or anything else
//!    that is not easily encoded as double precision or integer numbers.
//!
//!  NC is a function of ND and NI. The relationship between NC and
//!    user-specified ND and NI was chosen to allowing a reasonable amount of
//!    space for storing the alphanumeric information for an array. NC is
//!    defined below:
//!
//!  
//!
//! ```text
//!                       (NI + 1)
//!    NC  =   8 * ( ND + -------- )     (Note that this is
//!                           2           integer division.)
//! ```
//!
//!  Using an SPK as an example with NI = 6, ND = 2, then NC = 40.
//!
//!  The double precision and integer numbers that describe each array are
//!    'packed', or equivalenced, into an auxiliary double precision array
//!    before they are stored in the file. This auxiliary array is called the
//!    'summary' of the associated array. The individual (unpacked) numbers are
//!    called the 'components' of the summary.
//!
//!  (The first ND elements of the summary contain the double precision
//!    components of the summary. Each of the remaining elements contains a
//!    pair of integer components. If NI is odd, the final element of the
//!    summary contains a single integer component.)
//!
//!  The NC alphanumeric characters that further describe each array are
//!    stored in a single character string, called the 'name' of the array.
//!
//!  
//!
//!
//!  
//! ##  Array Addresses
//!
//!  The location of each array in a DAF is defined by a pair of numbers,
//!    called the 'initial address' and 'final address' of the array.
//!
//!  The term 'address' refers to a particular way of looking at an array
//!    file. Every DAF is actually a standard Fortran-77 direct access file,
//!    with a particular record length. (Every DAF has the same record length.)
//!    Each record is capable of storing up to 128 double precision numbers.
//!
//!  It is convenient, however, to think of a DAF as a numbered collection of
//!    slots named 'words'. Each word is large enough to hold one double
//!    precision number. Words 1 through 128 are located in the first record of
//!    the file; words 129 through 256 are located in the second record; and so
//!    on. The number of each word is called the 'address' of the word within
//!    the file.
//!
//!  Any pair of addresses defines a contiguous set of words, which may fall
//!    within a single physical record or span a number of records. The
//!    elements of each array in a DAF are stored in just such a set. The
//!    address of the first array element is the 'initial address' of the
//!    array. The address of the final array element is the 'final address' of
//!    the array.
//!
//!  The initial and final addresses of an array are always the values of the
//!    final two integer components of the summary for the array.
//!
//!  
//!
//!
//!  
//! ##  Array Files and Linked Lists
//!
//!  It is a simplification, but a useful one, to say that the arrays in an
//!    array file form a doubly-linked list. Each new array added to a file is
//!    placed at the tail of this list.
//!
//!  Because the list is doubly-linked, the head and tail of the list can be
//!    located immediately. The arrays can be located by moving a pointer
//!    through the list, in either direction, one array at a time.
//!
//!  At any time, the summary and name of the array at which the pointer is
//!    currently pointing can be retrieved and examined to determine whether
//!    the array is of interest. If it is, the initial and final addresses (the
//!    final two integer components of the summary) may be used to
//!    access---retrieve or update---the entire array, or any contiguous set of
//!    elements therein.
//!
//!  For example, if BEGIN and END are the initial and final addresses of an
//!    array, the first ten elements of the array can be retrieved by asking
//!    for the elements stored in addresses BEGIN through BEGIN+9. If the array
//!    contains an odd number of elements, the middle element can be retrieved
//!    by asking for the element stored in address (BEGIN+END)/2.
//!
//!  
//!
//!
//!  
//! ##  Read and Write Access
//!
//!  Array files may be opened for two kinds of access: read and write. A
//!    file opened for read access cannot be changed, either by adding a new
//!    array or by updating an existing one. Unless one of these operations
//!    must be performed, files should be opened for read access. The
//!    protection provided to files opened for read access is independent of
//!    any particular operating system.
//!
//!  A program may open only one DAF at a time for write access. When a
//!    program attempts to open a file for write access, an error is signaled
//!    if another file is already open for write access, or if the file is
//!    already open for read access. An error is also signaled if a program
//!    attempts to open a file for read access if the file is already open for
//!    write access. (Errors are signaled through the standard SPICE error
//!    handling mechanism.)
//!
//!  
//!
//!
//!  
//! ##  File Handles
//!
//!  When a file is opened for either kind of access, it is assigned an
//!    integer 'handle'. A mapping between handles and Fortran logical units is
//!    maintained internally by the DAF subroutines.
//!
//!  As a means of accessing files, handles have two advantages over logical
//!    unit numbers.
//!
//!  
//!
//! *  1. They reduce the possibility that two or more program units of the same
//! program will interfere with each other when both need to access the same
//! DAF. When a program opens a DAF for the first time, the file is connected
//! to a logical unit, and the unit is mapped to a handle. If the program
//! attempts to open the same file again, the handle is returned immediately,
//! and a counter is incremented. The file is not disconnected from the logical
//! unit until it has been closed as many times as it has been opened. (This is
//! analogous to the creation of multiple links to a single file under the UNIX
//! operating system.) Any one program unit is prevented from releasing a file
//! that is still being used by other program units.
//!
//!  *  2. They allow the SPICE subroutines to prevent files opened for read access
//! from being modified. (Positive handles are assigned to files opened for
//! read access, negative handles to files opened for write access. Any attempt
//! to modify a file with a positive handle signals an error.) Note that this
//! scheme is independent of any file protection provided by the host operating
//! system.
//!
//!     
//! #  DAF Organization
//!
//!  
//!
//!
//!  
//! ##  Structure
//!
//!  Every DAF is a Fortran-77 direct access file, created by the following
//!    statement (or an equivalent C statement producing the same results):
//!
//!  
//!
//! ```text
//!    OPEN ( UNIT   = unit,
//!           FILE   = file name,
//!           ACCESS = 'DIRECT',
//!           RECL   = record length,
//!           STATUS = 'NEW'           )
//! ```
//!
//!  The DAF will have binary format (big-endian, little endian, etc.) based
//!    on the architecture of the machine used to create the DAF.
//!
//!  The record length is processor dependent. The smallest possible value
//!    should be selected by the user such that each record in the file is
//!    large enough to contain 128 double precision numbers (1024 bytes) or
//!    1000 characters, whichever is larger.
//!
//!  As of this version of the Toolkit, an expectation exists that a
//!    character has a size of one byte.
//!
//!  A DAF contains five types of physical records:
//!
//!  
//!
//! *  1. A single 'file record'. This contains global information about the file.
//!
//!  *  2. An arbitrary number of 'reserved records' for use as the comment area. NAIF
//! recommends against use of the DAF reserved records routines. Rather, use
//! the DAF comment area routines to store data information, file notes, or
//! meta data.
//!
//!  *  3. Some number of 'summary records'. These contain array summaries and
//! pointers to other summary records. The number of summary records in a
//! particular array file is a function of the number of arrays stored in the
//! file.
//!
//!  *  4. Some number of 'name records'. These contain array names. A DAF contains
//! one name record for each summary record.
//!
//!  *  5. An arbitrary number of 'element records'. These contain elements of the
//! arrays stored in the DAF.
//!
//!     
//! ###  The File Record
//!
//!  The file record is always the first physical record in a DAF. The record
//!    size is 1024 bytes (for platforms with one byte char size, and four
//!    bytes integer size). The items listed in the File Record:
//!
//!  
//!
//! *  1. LOCIDW (8 characters, 8 bytes): An identification word ('DAF/xxxx').
//!
//!  *  The 'xxxx' substring is a string of four characters or less indicating the
//! type of data stored in the DAF file. This is used by the SPICELIB
//! subroutines to verify that a particular file is in fact a DAF and not
//! merely a direct access file with the same record length. When a DAF is
//! opened, an error signals if this keyword is not present. \[Address 0]
//!
//!  *  2. ND ( 1 integer, 4 bytes): The number of double precision components in each
//! array summary. \[Address 8]
//!
//!  *  3. NI ( 1 integer, 4 bytes): The number of integer components in each array
//! summary. \[Address 12]
//!
//!  *  4. LOCIFN (60 characters, 60 bytes): The internal name or description of the
//! array file. \[Address 16]
//!
//!  *  5. FWARD ( 1 integer, 4 bytes): The record number of the initial summary
//! record in the file. \[Address 76]
//!
//!  *  6. BWARD ( 1 integer, 4 bytes): The record number of the final summary record
//! in the file. \[Address 80]
//!
//!  *  7. FREE ( 1 integer, 4 bytes): The first free address in the file. This is the
//! address at which the first element of the next array to be added to the
//! file will be stored. \[Address 84]
//!
//!  *  8. LOCFMT ( 8 characters, 8 bytes): The character string that indicates the
//! numeric binary format of the DAF. The string has value either "LTL-IEEE" or
//! "BIG-IEEE." \[Address 88]
//!
//!  *  9. PRENUL ( 603 characters, 603 bytes): A block of nulls to pad between the
//! last character of LOCFMT and the first character of FTPSTR to keep FTPSTR
//! at character 700 (address 699) in a 1024 byte record. \[Address 96]
//!
//!  *  10. FTPSTR ( 28 characters, 28 bytes): The FTP validation string.
//!
//!  *  This string is assembled using components returned from the SPICELIB
//! private routine ZZFTPSTR. \[Address 699]
//!
//!  *  11. PSTNUL ( 297 characters, 297 bytes): A block of nulls to pad from the last
//! character of FTPSTR to the end of the file record. Note: this value
//! enforces the length of the file record as 1024 bytes. \[Address 727]
//!
//!  Example; display the octal and ASCII output for the first 1024 bytes of
//!    an SPK, little-endian:
//!
//!  
//!
//! ```text
//!    $ od -cbv -N1024 de421.bsp
//!  
//!    -c       display ASCII character or backslash escapes
//!    -b       display octal values
//!    -v       display duplicate lines
//!    -N1024   display 1024 bytes
//!  
//!    0000  D   A   F   /   S   P   K     002  \0  \0  \0 006  \0  \0  \0
//!          104 101 106 057 123 120 113 040 002 000 000 000 006 000 000 000
//!    0020   N   I   O   2   S   P   K
//!          116 111 117 062 123 120 113 040 040 040 040 040 040 040 040 040
//!    0040
//!          040 040 040 040 040 040 040 040 040 040 040 040 040 040 040 040
//!    0060
//!          040 040 040 040 040 040 040 040 040 040 040 040 040 040 040 040
//!    0100                                                 004  \0  \0  \0
//!          040 040 040 040 040 040 040 040 040 040 040 040 004 000 000 000
//!    0120  004  \0  \0  \0 325 005      \0   L   T   L   -   I   E   E   E
//!          004 000 000 000 325 005 040 000 114 124 114 055 111 105 105 105
//!    0140  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0160  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0200  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0220  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0240  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0260  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0300  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0320  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0340  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0360  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0400  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0420  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0440  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0460  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0500  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0520  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0540  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0560  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0600  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0620  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0640  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0660  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0700  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0720  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0740  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    0760  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1000  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1020  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1040  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1060  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1100  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1120  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1140  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1160  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1200  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1220  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1240  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1260  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0   F   T   P   S   T
//!          000 000 000 000 000 000 000 000 000 000 000 106 124 120 123 124
//!    1300   R   :  \r   :  \n   :  \r  \n   :  \r  \0   : 201   : 020 316
//!          122 072 015 072 012 072 015 012 072 015 000 072 201 072 020 316
//!    1320   :   E   N   D   F   T   P  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          072 105 116 104 106 124 120 000 000 000 000 000 000 000 000 000
//!    1340  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1360  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1400  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1420  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1440  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1460  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1500  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1520  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1540  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1560  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1600  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1620  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1640  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1660  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1700  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1720  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1740  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    1760  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0  \0
//!          000 000 000 000 000 000 000 000 000 000 000 000 000 000 000 000
//!    2000
//! ```
//!
//!  The "spy" utility can output an easier-to-read-by-humans form of the
//!    file record:
//!
//!  
//!
//! ```text
//!    Spy > dump file record spk de421.bsp;
//!  
//!    File Record Contents of SPK File de421.bsp
//!    ==================================================================
//!     File Architecture      :  DAF
//!     Binary File Format     :  LTL-IEEE
//!     FTP Validation String  :  Present and Intact
//!     Internal File Name     :  NIO2SPK
//!     ID Word                :  DAF/SPK
//!     ND                     :  2
//!     NI                     :  6
//!     Forward Record Pointer :  4
//!     Backward Record Pointer:  4
//!     First Free Address     :  2098645
//!    ==================================================================
//! ```
//!
//!     
//! ###  Comment Area
//!
//!  Any DAF can contain a number of reserved records for use as a comment
//!    area. The number of records defines the size of the comment area and is
//!    assigned when creating the DAF. The comment area of an existing DAF may
//!    be modified either by adding additional text or deleting the area's
//!    contents.
//!
//!  The DAF subsystem includes routines to programmatically manipulate
//!    reserved records. NAIF recommends against use of these routines, rather
//!    use those routines designed to manipulate the DAF comment area.
//!
//!  NAIF also provides utilities to manipulate a DAF comment area. Refer to
//!    the commnt User's Guide ([commnt.ug](crate::raw::commnt.ug)) and spacit User's Guide ([spacit.ug](crate::raw::spacit.ug))
//!    for relevant information.
//!
//!  
//!
//!
//!  
//! ###  Summary Records
//!
//!  A summary record contains a maximum of 128 double precision words. The
//!    first three words of each summary record are reserved for the following
//!    control information:
//!
//!  
//!
//! *  1. The record number of the next summary record in the file. (Zero if this is
//! the final summary record.)
//!
//!  *  2. The record number of the previous summary record in the file. (Zero if this
//! is the initial summary record.)
//!
//!  *  3. The number of summaries stored in this record.
//!
//!  The record pointers form the basis of the array list. Each summary
//!    record is linked to two other summary records, allowing the summaries to
//!    be retrieved in forward or backward order. (The links between adjacent
//!    summaries in a summary record are implicit.) The names can be retrieved
//!    from the corresponding name records. And the locations (initial and
//!    final addresses) of the arrays themselves are stored in the summaries.
//!
//!  Although the control items are integer values, they are stored as double
//!    precision numbers. This allows summary records and element records,
//!    which contain only double precision numbers, to be buffered using the
//!    same mechanism.
//!
//!  The control items are followed immediately by the summaries themselves.
//!    The number of summaries (NS) that can fit in a single summary record
//!    depends on the size of a single summary (SS), a function of NI and ND:
//!
//!  
//!
//! ```text
//!                     (NI + 1)
//!    SS       =  ND + --------         (Note that this is
//!                         2             integer division.)
//!  
//!    SS * NS <=  125
//!  
//!    NS      <=  125/SS                (Note that NS must be an
//!                                       integer greater than or
//!                                       equal to one.)
//! ```
//!
//!  Using an SPK as an example with NI = 6, ND = 2, then SS = 5.
//!
//!  A summary record can be depicted as written below, where the numbers
//!    correspond to the number of the double precision word in the record.
//!
//!  
//!
//! ```text
//!    -------------------------------------------------------------
//!    | 1 | 2 | 3 | 4 | 5 | 6 |       ...       | 126 | 127 | 128 |
//!    -------------------------------------------------------------
//!      ^   ^   ^
//!    NEXT  |   |
//!         PREV |
//!             NSUM
//! ```
//!
//!  If SS is the size (in double precision words) of each summary array,
//!    then the first summary is stored in words 4 through SS+3; the second
//!    summary is stored in words SS+4 through 2(SS)+3; and so on. For example,
//!    if SS is equal to 3, 41 (125/3) summaries can fit in the summary record,
//!    leaving two words empty. In this case the record can be pictorially
//!    represented as written below, where the label below the record indicates
//!    the summary number.
//!
//!  
//!
//! ```text
//!    -------------------------------------------------------------
//!    | 1 | 2 | 3 | 4 | 5 | 6 | ... | 124 | 125 | 126 | 127 | 128 |
//!    -------------------------------------------------------------
//!      ^   ^   ^ { Summary 1 } ... {    Summary 41   }  ^     ^
//!    NEXT  |   |                                        |     |
//!         PREV |                                       These words
//!             NSUM                                     are unused.
//! ```
//!
//!  Unlike arrays, summaries are never split across physical record
//!    boundaries, so the end of each summary record may remain unused.
//!    Whenever the number of summaries stored in the current summary record
//!    reaches the maximum number that will fit, a new (empty) summary record
//!    is added to the end of the file.
//!
//!  It is now clear that the bounds on the values for ND and NI are
//!    determined by their relation to the number of double precision words
//!    used for summary information in the summary record. ND and NI must
//!    satisfy the following inequalities:
//!
//!  
//!
//! ```text
//!         (NI + 1)                     (Note that this is
//!    ND + -------- <= 125               integer division.)
//!             2
//!  
//!    0 <= ND <= 124
//!  
//!    2 <= NI <= 250
//! ```
//!
//!     
//! ###  Name Records
//!
//!  Each name record contains nothing but array names. A new name record is
//!    added to the file each time a new summary record is added. The new name
//!    record is located in the record immediately following the new summary
//!    record. Because a DAF is written in this manner, the number of summary
//!    records is equal to the number of name records.
//!
//!  Each time a new summary is added to a summary record, a new name is
//!    added to the corresponding name record. Therefore, the number of
//!    summaries in a summary record is equal to the number of names in the
//!    corresponding name record.
//!
//!  If the numbers in the summary record represent double precision words,
//!    and the numbers in the name record represent characters, the two records
//!    can be depicted as written below for a DAF whose format is specified by
//!    ND = 2 and NI = 2.
//!
//!  
//!
//! ```text
//!    -------------------------------------------------------------
//!    | 1 | 2 | 3 | 4 | 5 | 6 | ... | 124 | 125 | 126 | 127 | 128 |
//!    -------------------------------------------------------------
//!      ^   ^   ^ { Summary 1 } ... {    Summary 41   }  ^     ^
//!    NEXT  |   |                                        |     |
//!         PREV |                                     Words 127 and
//!             NSUM                                   128 are unused.
//!  
//!    -------------------------------------------------------------
//!    | 1 |...| 24 |.......| 961 | ... | 984 | 985 |.......| 1000 |
//!    -------------------------------------------------------------
//!    {   Name 1   }.......{    Name 41      }  ^              ^
//!                                              |              |
//!                                            Characters 985 through
//!                                            1000 are unused.
//! ```
//!
//!  The first name is stored in characters 1 through NC of the record; the
//!    second name is stored in characters NC+1 through 2(NC); and so on.
//!
//!  
//!
//!
//!  
//! ###  Element Records
//!
//!  Most of the records in any DAF are element records. Element records hold
//!    the elements of the arrays stored in the file. (The other records are
//!    used for accounting purposes only.)
//!
//!  Each element record contains up to 128 double precision numbers. An
//!    element record is always full (contains 128 numbers) unless it
//!    immediately precedes a summary record, in which case it may be partially
//!    filled.
//!
//!  The elements stored in a particular element record may belong to more
//!    than one array. However, elements belonging to the same array are stored
//!    contiguously within the record.
//!
//!  For example, suppose three arrays exist: A, B, and C. Array A has 10
//!    elements, array B has 100 elements, and array C has 15 elements. If all
//!    of the elements are stored in the same element record, it could be
//!    pictorially represented as written below:
//!
//!  
//!
//! ```text
//!    A[1]
//!    A[2]
//!    A[3]
//!     .
//!     .
//!     .
//!    A[10]
//!    B[1]
//!    B[2]
//!    B[3]
//!     .
//!     .
//!     .
//!    B[100]
//!    C[1]
//!    C[2]
//!    C[3]
//!     .
//!     .
//!     .
//!    C[15]
//! ```
//!
//!  A particular element record always lies between two summary/name record
//!    pairs, or between a summary/name record pair and the end of the file.
//!
//!  
//!
//!
//!  
//! ##  Extension to the DAF Format
//!
//!  NAIF modified the DAF specification for the N0052 release of the SPICE
//!    Toolkit (January, 2002). The modifications added the integrity string,
//!    the architecture strings, and the capability for DAF subsystem routines
//!    to perform run-time byte order translation.
//!
//!  
//!
//!
//!  
//! ###  DAF Run-Time Binary File Format Translation
//!
//!  On supported platforms, the DAF readers are able to read DAFs (SPK, CK
//!    and binary PCK) written using a non-native, binary representation. This
//!    access is read-only; any operations requiring writing to the file
//!    (adding information to the comment area, or appending additional
//!    ephemeris data, for example) require prior conversion of the file to the
//!    native binary file format. See the Convert User's Guide ([convert.ug](crate::raw::convert.ug)) or
//!    the BINGO User's Guide for details (standard Toolkit distributions do
//!    not include "bingo," download the executable and User's Guide at the URL
//!    <http://naif.jpl.nasa.gov/naif/utilities.html>.)
//!
//!  
//!
//!
//!  
//! #  DAF Subsystem Subroutines
//!
//!  SPICE contains a set of routines usable to create, populate, and
//!    manipulate DAFs. A complete list of the routines can be found at the end
//!    of this document (Appendix A).
//!
//!  Each subroutine is prefaced by a complete SPICE module header that
//!    describes inputs, outputs, restrictions, and exceptions, discusses the
//!    context in which the subroutine should be used, and shows typical
//!    examples of its use. Any discussion of the subroutines in this article
//!    is intended as an introduction: the final documentation for any
//!    subroutine is its module header.
//!
//!  
//!
//!
//!  
//! ##  Opening and Closing Array Files
//!
//!  An existing DAF can be opened by supplying the name of the file to
//!    [DAFOPR](crate::raw::dafopr) (for read access) or [DAFOPW](crate::raw::dafopw) (for write access). Each routine
//!    returns a file handle, which must be used for all subsequent access to
//!    the file.
//!
//!  SPICELIB (Fortran)
//!
//!  
//!
//! ```text
//!    CALL DAFOPR ( FNAME, HANDLE )          { Open for read }
//!    CALL DAFOPW ( FNAME, HANDLE )          { Open for write }
//! ```
//!
//!  CSPICE (C)
//!
//!  
//!
//! ```text
//!    dafopr_c ( fname, &handle )
//!    dafopw_c ( fname, &handle )
//! ```
//!
//!  Icy (IDL)
//!
//!  
//!
//! ```text
//!    cspice_dafopr, fname, handle
//!    cspice_dafopw, fname, handle
//! ```
//!
//!  Mice (MATLAB)
//!
//!  
//!
//! ```text
//!    handle = cspice_dafopr( fname )
//!    handle = cspice_dafopw( fname )
//! ```
//!
//!  Once opened, a DAF MUST be closed by supplying its handle to [DAFCLS](crate::raw::dafcls).
//!
//!  SPICELIB (Fortran)
//!
//!  
//!
//! ```text
//!    CALL DAFCLS ( HANDLE )                 { Close }
//! ```
//!
//!  CSPICE (C)
//!
//!  
//!
//! ```text
//!    dafcls_c ( handle )
//! ```
//!
//!  Icy (IDL)
//!
//!  
//!
//! ```text
//!    cspice_dafcls, handle
//! ```
//!
//!  Mice (MATLAB)
//!
//!  
//!
//! ```text
//!    cspice_dafcls( handle )
//! ```
//!
//!     
//! ##  Creating Array Files
//!
//!  A new DAF can be created by supplying the name of the file, the type of
//!    data in the file, values for ND and NI, an internal file name, and the
//!    number of records to be reserved. NI must be greater than or equal to 2.
//!    (For more information about the bounds on NI and ND see the subsection
//!    'Summary records'.)
//!
//!  
//!
//! ```text
//!    CALL DAFONW ( FNAME,                   { Open new }
//!                  FTYPE,
//!                  ND,
//!                  NI,
//!                  IFNAME,
//!                  RESV,
//!                  HANDLE  )
//!  
//!    This routine currently exists only in Fortran SPICELIB.
//! ```
//!
//!  The internal name of a DAF is a string of up to 60 bytes, which may be
//!    used to characterize the contents of the file. Its primary value is
//!    that, being internal to the file, it remains unchanged when the file is
//!    transferred between environments.
//!
//!  Any number of records may be reserved at the front of a DAF. By
//!    definition, the contents of these records are invisible to DAF
//!    subroutines, and may contain any information that the user wishes to
//!    store in them.
//!
//!  Once created, a new DAF remains open for write access until explicitly
//!    closed.
//!
//!  
//!
//!
//!  
//! ##  Adding Arrays
//!
//!  A new array can be added to an existing DAF by calling four routines:
//!    [DAFPS](crate::raw::dafps), [DAFBNA](crate::raw::dafbna), [DAFADA](crate::raw::dafada), and [DAFENA](crate::raw::dafena).
//!
//!  First, the summary is packed by [DAFPS](crate::raw::dafps), which requires two arrays
//!    containing the double precision and integer components of the summary.
//!    It also requires the values of ND and NI for the file.
//!
//!  
//!
//! ```text
//!    CALL DAFPS ( ND, NI, DC, IC, SUM )     { Pack summary }
//!  
//!    This routine currently exists only in Fortran SPICELIB.
//!    \literal
//!  
//!    The final two integer components of the summary are
//!    always used to store the initial and
//!    final addresses of the array imposing that NI be
//!    greater than or equal to two. These components are
//!    filled in after the array has been
//!    stored: any values for these components
//!    supplied by the user are ignored.
//!  
//!  
//!    Next, the new array must be initialized by calling
//!    DAFBNA. DAFBNA requires the handle of the file
//!    (which must be open for write access),
//!    the array name, and the array summary.
//!  
//!    \literal
//!    CALL DAFBNA ( HANDLE, SUM, NAME )      { Begin new array }
//!  
//!    This routine currently exists only in Fortran SPICELIB.
//! ```
//!
//!  The elements of the array are added by [DAFADA](crate::raw::dafada). The elements may be
//!    supplied using one call,
//!
//!  
//!
//! ```text
//!    CALL DAFADA ( DATA, N )                { Add data to array }
//!  
//!    This routine currently exists only in Fortran SPICELIB.
//! ```
//!
//!  or in any number of installments,
//!
//!  
//!
//! ```text
//!    DO WHILE ( MORE )
//!        ...
//!       CALL DAFADA ( DATA, N )             { Add data to array }
//!    END DO
//! ```
//!
//!  Once the entire array has been supplied, [DAFENA](crate::raw::dafena) makes the addition
//!    permanent.
//!
//!  
//!
//! ```text
//!    CALL DAFENA                            { End new array }
//!  
//!    This routine currently exists only in Fortran SPICELIB.
//! ```
//!
//!  If the process is aborted before [DAFENA](crate::raw::dafena) is called, the summary and name
//!    are not stored, and the new array does not become a permanent member of
//!    the file. Space allocated for elements of the array cannot be removed
//!    from the file; however, it will be overwritten by the elements of the
//!    next array added to the file.
//!
//!  One way to abort the addition of an array to a file is to call [DAFBNA](crate::raw::dafbna) to
//!    start a new array in the same file, without first ending the current
//!    array.
//!
//!  
//!
//!
//!  
//! ##  Adding Arrays to Multiple Array Files
//!
//!  It is possible to add data to arrays in multiple files in an interleaved
//!    fashion: addition of data to an array in one file can be interrupted in
//!    order to add data to an array in another file. To accomplish this, it is
//!    necessary to tell [DAFADA](crate::raw::dafada) and [DAFENA](crate::raw::dafena) which file to act upon. This file is
//!    called the 'current file'.
//!
//!  When [DAFBNA](crate::raw::dafbna) is used to begin an array, the file specified by the handle
//!    passed to [DAFBNA](crate::raw::dafbna) becomes the current file. Calls to [DAFADA](crate::raw::dafada) or [DAFENA](crate::raw::dafena)
//!    will add data to or end the last array begun in this file. If [DAFBNA](crate::raw::dafbna) is
//!    called again, this time with a different handle, the file specified by
//!    that handle becomes current. Files that are not current are not affected
//!    in any way by beginning, adding data to, or ending arrays in the current
//!    file.
//!
//!  In any given file, an array that is in progress---that is, an array
//!    begun by [DAFBNA](crate::raw::dafbna) but not yet ended by [DAFENA](crate::raw::dafena)---is called the 'current
//!    array' for that file. No file can have more than one current array.
//!
//!  In order to continue or end an array in a file that is no longer
//!    current, the file in question is selected as the current file by a call
//!    to [DAFCAD](crate::raw::dafcad):
//!
//!  
//!
//! ```text
//!    CALL DAFCAD ( HANDLE )                { DAF, continue adding data }
//!  
//!    This routine currently exists only in Fortran SPICELIB.
//! ```
//!
//!  After this call, the file identified by HANDLE will be the current file,
//!    and calls to [DAFADA](crate::raw::dafada) will add data to the current array in this file. The
//!    usual sequence of calls has the form:
//!
//!  
//!
//! ```text
//!    CALL DAFCAD ( HANDLE  )               { DAF, continue adding data }
//!    CALL DAFADA ( DATA, N )               { DAF, add data to array }
//! ```
//!
//!  Since [DAFENA](crate::raw::dafena) can be used to end arrays only in the current file, [DAFCAD](crate::raw::dafcad) is also used to select a file as current so that an array can be ended
//!    in that file:
//!
//!  
//!
//! ```text
//!    CALL DAFCAD ( HANDLE  )               { DAF, continue adding data }
//!    CALL DAFENA                           { DAF, end new array }
//! ```
//!
//!  Only files that already have an array in progress may be selected as
//!    current by [DAFCAD](crate::raw::dafcad). An error will be signaled if DAFCAD is used to select
//!    a DAF that does not have an array in progress.
//!
//!  The following example illustrates the use of [DAFCAD](crate::raw::dafcad):
//!
//!  We write data obtained from the routine GET_DATA (which is not a
//!    SPICELIB routine) into two separate DAFs. The first N/2 elements of the
//!    array DATA will be written to the first file; the rest of the array will
//!    be written to the second file.
//!
//!  Open the DAFs for write access, using either [DAFOPW](crate::raw::dafopw) (if the files
//!    already exist) or [DAFONW](crate::raw::dafonw) (if they do not).
//!
//!  
//!
//! ```text
//!    CALL DAFOPW ( FNAME1, HANDL1 )
//!    CALL DAFOPW ( FNAME2, HANDL2 )
//! ```
//!
//!  Begin the new DAFs by calling [DAFBNA](crate::raw::dafbna).
//!
//!  
//!
//! ```text
//!    CALL DAFBNA ( HANDL1, SUM1, NAME1 )
//!    CALL DAFBNA ( HANDL2, SUM2, NAME2 )
//! ```
//!
//!  Add data to the arrays, using [DAFCAD](crate::raw::dafcad) to select the current file and [DAFADA](crate::raw::dafada) to add data to the current array in the current file.
//!
//!  
//!
//! ```text
//!    CALL GET_DATA ( DATA, N, FOUND )
//!  
//!    DO WHILE ( FOUND )
//!  
//!       CALL DAFCAD   ( HANDL1                          )
//!       CALL DAFADA   ( DATA,               N/2         )
//!  
//!       CALL DAFCAD   ( HANDL2                          )
//!       CALL DAFADA   ( DATA( N/2 + 1 ),    N - N/2     )
//!  
//!       CALL GET_DATA ( DATA, N, FOUND )
//!  
//!    END DO
//! ```
//!
//!  End each array by calling [DAFENA](crate::raw::dafena), selecting the file in which to end the
//!    array by calling [DAFCAD](crate::raw::dafcad):
//!
//!  
//!
//! ```text
//!    CALL DAFCAD ( HANDL1 )
//!    CALL DAFENA
//!  
//!    CALL DAFCAD ( HANDL2 )
//!    CALL DAFENA
//! ```
//!
//!  The notions of 'current DAF' and 'current array' apply to both adding
//!    data to arrays and to searching DAFs. However, the files and arrays
//!    regarded as current for the purpose of searching are unrelated to those
//!    regarded as current for the purpose of adding data.
//!
//!  
//!
//!
//!  
//! ##  Reordering Arrays
//!
//!  Once arrays are written to a DAF, it is conceivable that their order may
//!    need to be changed. Suppose that the arrays in a DAF are to be ordered
//!    according to the arithmetic mean of the data they contain. Also suppose
//!    that the arithmetic mean of data in an array is stored in the second
//!    double precision component of the summary. After reading each summary
//!    and creating a vector IORDER with dimension N that specifies the new
//!    order of the arrays, the subroutine [DAFRA](crate::raw::dafra) can be used to reorder the
//!    arrays.
//!
//!  
//!
//! ```text
//!    CALL DAFRA ( HANDLE, IORDER, N )       { Reorder arrays }
//! ```
//!
//!     
//! ##  Searching
//!
//!  The process of locating an array of interest within an array file is
//!    known as 'searching'. The organization of the arrays as a doubly-linked
//!    list makes it possible to conduct searches in forward or backward order.
//!
//!  The complete set of DAF routines that act on the current file (for
//!    searching) is:
//!
//!  
//!
//! ```text
//!    DAFFNA                                 { DAF, find next array }
//!    DAFFPA                                 { DAF, find previous array }
//!    DAFGS                                  { DAF, get summary }
//!    DAFGN                                  { DAF, get name }
//!    DAFGH                                  { DAF, get handle }
//!    DAFRS                                  { DAF, replace summary }
//!    DAFRN                                  { DAF, replace name }
//!    DAFWS                                  { DAF, write summary }
//! ```
//!
//!  Subroutines [DAFBFS](crate::raw::dafbfs) and [DAFFNA](crate::raw::daffna) are used to search a DAF in forward order. [DAFBFS](crate::raw::dafbfs) places a pointer at the head of the doubly-linked list formed by
//!    the arrays in the file. Each call to [DAFFNA](crate::raw::daffna) moves the pointer to the
//!    next array in the list. (The first call to [DAFFNA](crate::raw::daffna) moves the pointer to
//!    the first array.) [DAFFNA](crate::raw::daffna) returns a logical flag which is true whenever
//!    another array has been found, and is false when the tail of the list has
//!    been reached. All forward searches are variations on the following
//!    template:
//!
//!  SPICELIB (Fortran)
//!
//!  
//!
//! ```text
//!    CALL DAFBFS ( HANDLE )                 { Begin forward search }
//!    CALL DAFFNA ( FOUND  )                 { Find next array }
//!  
//!    DO WHILE ( FOUND )
//!        ...
//!       CALL DAFFNA ( FOUND )               { Find next array }
//!    END DO
//! ```
//!
//!  CSPICE (C)
//!
//!  
//!
//! ```text
//!    dafbfs_c ( handle );
//!    daffna_c ( &found );
//!  
//!    while ( found )
//!       {
//!        ...
//!       daffna_c ( &found );
//!       }
//! ```
//!
//!  Icy (IDL)
//!
//!  
//!
//! ```text
//!    cspice_dafbfs, handle
//!    cspice_daffna, found
//!  
//!    while found do begin
//!        ...
//!       cspice_daffna, found
//!    endwhile
//! ```
//!
//!  Mice (MATLAB)
//!
//!  
//!
//! ```text
//!    cspice_dafbfs( handle )
//!    found = cspice_daffna
//!  
//!    while found
//!        ...
//!       found = cspice_daffna
//!    end
//! ```
//!
//!  Subroutines [DAFBBS](crate::raw::dafbbs) and [DAFFPA](crate::raw::daffpa) are likewise used to search a DAF in
//!    backward order. [DAFBBS](crate::raw::dafbbs) moves the pointer to the tail (instead of the
//!    head) of the list; [DAFFPA](crate::raw::daffpa) moves the pointer to the previous (instead of
//!    the next) array in the list. The template shown above is modified to
//!    conduct backward searches by replacing calls to [DAFBFS](crate::raw::dafbfs) and [DAFFNA](crate::raw::daffna) with
//!    calls to [DAFBBS](crate::raw::dafbbs) and [DAFFPA](crate::raw::daffpa), respectively:
//!
//!  SPICELIB (Fortran)
//!
//!  
//!
//! ```text
//!    CALL DAFBBS ( HANDLE )                 { Begin backward search }
//!    CALL DAFFPA ( FOUND  )                 { Find previous array }
//!  
//!    DO WHILE ( FOUND )
//!        ...
//!       CALL DAFFPA ( FOUND )               { Find previous array }
//!    END DO
//! ```
//!
//!  CSPICE (C)
//!
//!  
//!
//! ```text
//!    dafbbs_c ( handle );
//!    daffpa_c ( &found );
//!  
//!    while ( found )
//!       {
//!        ...
//!       daffpa_c ( &found );
//!       }
//! ```
//!
//!  Icy (IDL)
//!
//!  
//!
//! ```text
//!    cspice_dafbbs, handle
//!    cspice_daffpa, found
//!  
//!    while found do begin
//!        ...
//!       cspice_daffpa, found
//!    endwhile
//! ```
//!
//!  Mice (MATLAB)
//!
//!  
//!
//! ```text
//!    cspice_dafbbs( handle )
//!    found = cspice_daffpa
//!  
//!    while found
//!        ...
//!       found = cspice_daffpa
//!    end
//! ```
//!
//!  Once a search has begun, the pointer may be moved in either direction.
//!
//!  After the pointer has been moved to a new array, the summary and name of
//!    the array can be retrieved by [DAFGS](crate::raw::dafgs) and [DAFGN](crate::raw::dafgn). Once returned, a name can
//!    be examined directly. However, a summary must may need to be unpacked
//!    into its components by subroutine [DAFUS](crate::raw::dafus).
//!
//!  
//!
//! ```text
//!    CALL DAFBBS ( HANDLE )                 { Begin backward search }
//!    CALL DAFFPA ( FOUND  )                 { Find previous array }
//!  
//!    DO WHILE ( FOUND )
//!       CALL DAFGS ( SUM  )                 { Get summary }
//!       CALL DAFUS ( SUM, ND, NI, DC, IC )  { Unpack summary }
//!       CALL DAFGN ( NAME )                 { Get name }
//!        ...
//!       CALL DAFFNA ( FOUND )               { Find next array }
//!    END DO
//! ```
//!
//!  CSPICE (C)
//!
//!  
//!
//! ```text
//!    dafbfs_c ( handle );
//!    daffna_c ( &found );
//!  
//!    while ( found )
//!       {
//!       dafgs_c ( sum  );
//!       dafus_c ( sum, ND, NI, dc, ic );
//!       dafgn_c ( name );
//!        ...
//!       daffna_c ( &found );
//!       }
//! ```
//!
//!  The Icy (IDL) and Matlab (Mice) implementations of the dafgs_c interface
//!    do not require an explicit call to unpack the summary. I.e., a call to
//!    cspice_dafgs equates to the C calls:
//!
//!  
//!
//! ```text
//!    dafgs_c( sum );
//!    dafus_c( sum, nd, ni, dc, ic );
//! ```
//!
//!  without use of the 'sum' variable.
//!
//!  Icy (IDL)
//!
//!  
//!
//! ```text
//!    cspice_dafbfs, handle
//!    cspice_daffna, found
//!  
//!    while found do begin
//!       cspice_dafgs, ND, NI, dc, ic
//!       cspice_dafgn, name
//!        ...
//!       cspice_daffna, found
//!    endwhile
//! ```
//!
//!  Mice (MATLAB)
//!
//!  
//!
//! ```text
//!    cspice_dafbfs( handle )
//!    found = cspice_daffna
//!  
//!    while found
//!       [dc, ic ] = cspice_dafgs( ND, NI );
//!       name = cspice_dafgn
//!        ...
//!       found = cspice_daffna
//!    end
//! ```
//!
//!     
//! ##  Searching Multiple Array Files
//!
//!  Searching multiple DAFs simultaneously is a little like adding data to
//!    multiple files simultaneously: in each case, it becomes necessary to
//!    identify the file to act upon, when calling routines that don't accept
//!    an input handle argument.
//!
//!  As with adding data, the notions of 'current DAF' and 'current array'
//!    apply to searching. Starting a search in an array file by calling either
//!    [DAFBFS](crate::raw::dafbfs) or [DAFBBS](crate::raw::dafbbs) makes that file the 'current file'. Subsequent calls to
//!    [DAFFNA](crate::raw::daffna) or [DAFFPA](crate::raw::daffpa) advance or back up the array pointer in the current
//!    file. The last array found by [DAFFNA](crate::raw::daffna) or [DAFFPA](crate::raw::daffpa) in the 'current file' is
//!    the 'current array' for that file. As mentioned above, there is no
//!    relation between the files or arrays that are considered current for
//!    searching and those considered current for adding data.
//!
//!  If, after a search is started in one DAF, [DAFBFS](crate::raw::dafbfs) or [DAFBBS](crate::raw::dafbbs) are called to
//!    start a search in a second DAF, the second file becomes current: [DAFFNA](crate::raw::daffna),
//!    [DAFFPA](crate::raw::daffpa), [DAFGN](crate::raw::dafgn), and [DAFGS](crate::raw::dafgs) will all operate on the second file.
//!
//!  The routine [DAFCS](crate::raw::dafcs) is used to continue a search in a DAF that is no
//!    longer current. Calling [DAFCS](crate::raw::dafcs) makes the file specified by the input
//!    handle argument the current file for searching:
//!
//!  
//!
//! ```text
//!    CALL DAFCS  ( HANDLE )                 { DAF, continue search }
//! ```
//!
//!  After this call, the routines in the above list will act upon the file
//!    designated by HANDLE. For example, to continue a forward search in that
//!    file,
//!
//!  
//!
//! ```text
//!    CALL DAFCS  ( HANDLE )                 { DAF, continue search }
//!    CALL DAFFNA ( FOUND  )                 { DAF, find next array }
//! ```
//!
//!  and to continue a backward search,
//!
//!  
//!
//! ```text
//!    CALL DAFCS  ( HANDLE )                 { DAF, continue search     }
//!    CALL DAFFPA ( FOUND  )                 { DAF, find previous array }
//! ```
//!
//!  while to get the name and summary of the current array in the file,
//!
//!  
//!
//! ```text
//!    CALL DAFCS ( HANDLE )                  { DAF, continue search }
//!    CALL DAFGN ( NAME )                    { DAF, get name }
//!    CALL DAFGS ( SUM  )                    { DAF, get summary }
//! ```
//!
//!  A search must have been started by [DAFBFS](crate::raw::dafbfs) or [DAFBBS](crate::raw::dafbbs) before it can be
//!    continued. An error will signal if [DAFCS](crate::raw::dafcs) is used to continue a search in
//!    a DAF in which no search has been started.
//!
//!  
//!
//!
//!  
//! ##  Accessing Array Elements
//!
//!  After an array of interest has been located, the entire array or any
//!    contiguous set of elements can be accessed---read or updated---by
//!    supplying a pair of addresses. Elements are read by [DAFRDA](crate::raw::dafrda) and written
//!    by [DAFWDA](crate::raw::dafwda).
//!
//!  The following code fragment continues the example above by subtracting
//!    the average from each of the elements in the array. (Recall that IA and
//!    FA contain the initial and final addresses of the array.)
//!
//!  
//!
//! ```text
//!    CALL DAFRDA ( HANDLE, IA, FA, DATA )   { Read data
//!                                             from address }
//!    DO I = 1, FA - IA + 1
//!       DATA(I) = DATA(I) - MAXAVG
//!    END DO
//!  
//!    CALL DAFWDA ( HANDLE, IA, FA, DATA )   { Write data
//!                                             to address }
//! ```
//!
//!  Note that it is not necessary to retrieve the entire array at once. The
//!    following code fragment illustrates how to process an array of unknown
//!    size using a fixed amount of local storage. The local array DATA is
//!    declared to be size CHUNK. [DAFRDA](crate::raw::dafrda) reads a maximum of CHUNK elements from
//!    the double precision array and [DAFWDA](crate::raw::dafwda) writes them. This technique is
//!    useful when the arrays stored in a DAF may be arbitrarily large.
//!
//!  
//!
//! ```text
//!    FIRST = IA
//!  
//!    DO WHILE ( FIRST .LE. FA )
//!       LAST  = MIN ( FA,     FIRST + CHUNK - 1 )
//!       CALL DAFRDA ( HANDLE, FIRST, LAST, DATA )   { Read data
//!                                                     from address }
//!       NUMELE = LAST - FIRST + 1
//!  
//!       DO I = 1, NUMELE
//!          DATA(I) = DATA(I) - MAXAVG
//!       END DO
//!  
//!       CALL DAFWDA ( HANDLE, FIRST, LAST, DATA )   { Write data
//!       FIRST = FIRST + CHUNK                         to address }
//!    END DO
//! ```
//!
//!     
//! ##  Updating Summaries and Names
//!
//!  In the previous example, once the average value of the array has been
//!    subtracted from each element of the array, the value for the average
//!    stored in the summary is no longer valid (the average is now zero) and
//!    should be changed.
//!
//!  Subroutines [DAFRS](crate::raw::dafrs) and [DAFRN](crate::raw::dafrn) are analogous to subroutines [DAFGS](crate::raw::dafgs) and
//!    [DAFGN](crate::raw::dafgn). [DAFGS](crate::raw::dafgs) 'gets' the summary for the array to which the pointer
//!    currently points; [DAFRS](crate::raw::dafrs) replaces it. [DAFGN](crate::raw::dafgn) 'gets' the name of the array
//!    to which the pointer currently points; [DAFRN](crate::raw::dafrn) replaces it.
//!
//!  If the index, K, of the updated array is known, then the new average for
//!    the array (zero) is stored by the following code fragment.
//!
//!  In the code fragment below, we also update each array name by appending
//!    the string
//!
//!  
//!
//! ```text
//!    (new)
//! ```
//!
//!  to the existing name.
//!
//!  
//!
//! ```text
//!    CALL DAFBFS ( HANDLE )                 { Begin forward search }
//!  
//!    DO I = 1, K
//!       CALL DAFFNA ( FOUND  )              { Find next array }
//!    END DO
//!  
//!    CALL DAFGS ( SUM )                     { Get summary }
//!    CALL DAFUS ( SUM, ND, NI, DC, IC )     { Unpack summary }
//!  
//!    DC(2) = 0.D0
//!  
//!    CALL DAFPS ( ND, NI, DC, IC, SUM )     { Pack summary }
//!    CALL DAFRS ( SUM )                     { Replace summary }
//!  
//!    CALL DAFGN  ( NAME )                   { Get name }
//!    CALL SUFFIX ( '(new)', 1, NAME )
//!    CALL DAFRN  ( NAME )                   { Replace name }
//! ```
//!
//!     
//! ##  Buffering
//!
//!  Unless the value of CHUNK is 128 (the number of double precision words
//!    in a record) and the initial address of the array happens to correspond
//!    to the first word of a physical record (neither of which is very
//!    likely), each call to [DAFRDA](crate::raw::dafrda) or [DAFWDA](crate::raw::dafwda) will involve reading partial
//!    records---data that spans across records. In general, successive calls
//!    will refer to different parts of at least one record.
//!
//!  In fact, as records are read from DAFs they are saved in an internal
//!    buffer maintained by the DAF subroutines. If any part of a record is
//!    needed, it can frequently be returned directly from the buffer, without
//!    accessing the file again. In particular, when an entire array is
//!    accessed sequentially, as in the example above, each of the necessary
//!    records is read exactly one time. When the elements of an array are
//!    accessed more randomly, the number of file accesses may increase
//!    somewhat.
//!
//!  It is possible, at any point in a program, to determine the number of
//!    file accesses prevented by the buffering scheme. The subroutine [DAFNRR](crate::raw::dafnrr)
//!    returns the number of physical records actually read, and the number of
//!    records or partial records that have been requested, as illustrated
//!    below:
//!
//!  
//!
//! ```text
//!    CALL DAFNRR ( READS, REQS )            { Number of reads,
//!                                             requests         }
//!    RATIO  = DBLE(READS) / DBLE(REQS)
//!    PERCNT = INT ( RATIO * 100.D0 )
//!    WRITE (*,*) 'Reads/requests (%) = ', PERCNT
//! ```
//!
//!  Ideally, the ratio of reads to requests should approach zero. In the
//!    worst case, where it approaches one, the size of the buffer should
//!    probably be adjusted. (The module headers for [DAFRDR](crate::raw::dafrdr) and [DAFWDR](crate::raw::dafwdr) provide
//!    details on adjusting the buffer size. Please consult NAIF before
//!    altering the buffer size.)
//!
//!  
//!
//!
//!  
//! #  Designing a DAF
//!
//!  
//!
//!
//!  
//! ##  Creating an Array File
//!
//!  The following example illustrates the use of addresses and lists within
//!    an DAF by showing how a simple array file might be created, and how
//!    arrays might be added to that file.
//!
//!  Throughout the example, the following notations will be used:
//!
//!  
//!
//! * Within the file record there are several values: IDWORD, ND, NI, RI, RF,
//! and FFA. IDWORD is a character string that contains the file architecture,
//! DAF, and code for the type of data stored in the DAF file. This code is a
//! string consisting of four characters or less and is described in the header
//! of the routine [DAFONW](crate::raw::dafonw). ND and NI are the values of the parameters that
//! define the format of the file. RI and RF are the record numbers of the
//! initial and final summary records in the file. FFA is the first free
//! address in the file.
//!
//!  * Within a particular summary record, NEXT and PREV are the record numbers of
//! the next and previous summary records in the file, and NSUM is the number
//! of summaries stored in the record.
//!
//!  The first step in creating a file is to determine the name to be given
//!    to the type of data stored in the DAF file. For this example, the data
//!    type will be 'Xmpl'. For more information about the restrictions on the
//!    character string describing the type of data in the DAF file, see the
//!    header of the routine [DAFONW](crate::raw::dafonw). The IDWORD written to the new file is the
//!    concatenation of the string 'DAF/' with the data type string. So, for
//!    this example, the IDWORD written to the new file is 'DAF/Xmpl'.
//!
//!  The next step is to select values for ND and NI. Normally, these are
//!    relatively small, allowing several summaries to fit in each summary
//!    record and thus increasing the speed with which the file can be
//!    searched. A new file is opened by calling [DAFONW](crate::raw::dafonw), specifying the
//!    selected values for ND and NI. Recall that the final two integer
//!    components of any array summary---IC(NI-1) and IC(NI)---contain the
//!    initial and final addresses of the array, so NI must be at least 2.
//!
//!  The example will be easier to follow, however, if the number of
//!    summaries that can fit in a summary record is minimized. Therefore, in
//!    this example ND and NI will take on unusually large values:
//!
//!  
//!
//! ```text
//!    ND = 25
//!    NI = 27
//! ```
//!
//!  Each array summary requires 39 double precision words of storage:
//!
//!  
//!
//! ```text
//!         (NI + 1)
//!    ND + -------- =  25 + 14 = 39
//!            2
//! ```
//!
//!  Therefore, each summary record can hold 3 summaries:
//!
//!  
//!
//! ```text
//!    125 (words per record)
//!    -----------------------  =  3 (summaries per record)
//!     39 (words per summary)
//! ```
//!
//!  If 'Summary(i)\[j]' represents the j'th element of the i'th summary
//!    array, then the layout of a typical summary record is shown below.
//!
//!  
//!
//! ```text
//!    Word     Value
//!    ----     ----------------
//!       1     NEXT
//!       2     PREV
//!       3     NSUM
//!       4     Summary( 1)[ 1]
//!       5     Summary( 1)[ 2]
//!              ...
//!      42     Summary( 1)[39]
//!      43     Summary( 2)[ 1]
//!              ...
//!      81     Summary( 2)[39]
//!      82     Summary( 3)[ 1]
//!              ...
//!     120     Summary( 3)[39]
//!     121     Unused
//!              ...
//!     128     Unused
//! ```
//!
//!  The number of names that an array record can hold is equivalent to the
//!    number of summaries that the summary record can hold. In this example
//!    it's three.
//!
//!  Recall that NC, the maximum number of characters in an array name, is
//!    determined by the values of ND and NI. For this example, where ND = 25
//!    and NI = 27, the value of NC is computed below:
//!
//!  
//!
//! ```text
//!                       (NI + 1)
//!    NC  =   8 * ( ND + -------- )  = 8 * ( 25 + 14 ) = 8 * 39 = 312
//!                           2
//! ```
//!
//!  Each array name may use up to 312 characters of storage. An array name
//!    does not have to be exactly NC characters long. NC is simply the limit
//!    on the length of the array name.
//!
//!  If 'Name(i)\[j]' represents the j'th character of the i'th name, then the
//!    layout of a typical name record is shown below.
//!
//!  
//!
//! ```text
//!    Character    Value
//!    ---------    --------------
//!            1     Name( 1)[  1]
//!            2     Name( 1)[  2]
//!                   ...
//!          312     Name( 1)[312]
//!          313     Name( 2)[  1]
//!                   ...
//!          624     Name( 2)[312]
//!          625     Name( 3)[  1]
//!                   ...
//!          936     Name( 3)[312]
//!          937     Unused
//!                   ...
//!         1000     Unused
//! ```
//!
//!  Assume that RESV, the number of reserved records, is 10. When [DAFONW](crate::raw::dafonw) opens the new file, it stores the file record information in record 1,
//!    the reserved records in records 2 through 11, and the initial summary
//!    record in record 12. Because the file is empty, the initial summary
//!    record, RI, is also the final summary record, RF.
//!
//!  
//!
//! ```text
//!    RI = 12
//!    RF = 12
//! ```
//!
//!  [DAFONW](crate::raw::dafonw) stores the lone name record for the file immediately after the
//!    summary record, in record 13. Therefore the first free address, FFA, in
//!    the file is the first word in record 14:
//!
//!  
//!
//! ```text
//!    FFA = word + (record - 1) * 128
//!  
//!        =    1 + (14     - 1) * 128
//!  
//!        =    1 + 1664
//!  
//!        =    1665
//! ```
//!
//!  [DAFONW](crate::raw::dafonw) also writes the internal file name to the file record. For this
//!    example the internal file name will be 'TESTFILE'. For the rest of the
//!    example, the file record will be depicted as a collection of values
//!    enclosed by braces and preceded by a record number:
//!
//!  
//!
//! ```text
//!    r { IDWORD=x, ND=a, NI=b, IFNAME=c, RI=d, RF=e, FFA=f }
//! ```
//!
//!  So, the file record for this example file is initially:
//!
//!  
//!
//! ```text
//!    1 { IDWORD='DAF/Xmpl', ND=25, NI=27, IFNAME='TESTFILE',
//!                           RI=12, RF=12, FFA=1665           }
//! ```
//!
//!  Because there is only one summary record, the values of NEXT and PREV in
//!    that record are both zero. Because the file contains no arrays, the
//!    value of NSUM is also zero. The information needed to create the summary
//!    record is complete. For the rest of the example, each summary record
//!    will be depicted as a collection of values enclosed by angle brackets
//!    and preceded by a record number:
//!
//!  
//!
//! ```text
//!    r < NEXT=a, PREV=b, NSUM=c, (d,e),(f,g),(h,i) >
//! ```
//!
//!  The ordered pairs enclosed in parentheses are the initial and final
//!    addresses of the arrays whose summaries are contained in the record. The
//!    remaining components of each summary are ignored in order to make the
//!    example easier to follow. Thus, the lone summary record for this example
//!    file is initially:
//!
//!  
//!
//! ```text
//!    12 < NEXT=0, PREV=0, NSUM=0, (0,0),(0,0),(0,0) >
//! ```
//!
//!  Name records will always be depicted as
//!
//!  
//!
//! ```text
//!    r < " " >
//! ```
//!
//!  Element records will always be depicted as
//!
//!  
//!
//! ```text
//!    r < N >
//! ```
//!
//!  where N is the number of elements stored in the record.
//!
//!  Once the initial summary and name records have been written, the file is
//!    complete, if uninteresting:
//!
//!  
//!
//! ```text
//!     1 { IDWORD='DAF/Xmpl', ND=25, NI=27, IFNAME='TESTFILE',
//!                            RI=12, RF=12, FFA=1665           }
//!     2
//!     .
//!     . Records 2 through 11 are reserved records.
//!     .
//!    11
//!    12 < NEXT=0, PREV=0, NSUM=0, (0,0),(0,0),(0,0) >
//!    13 < " " >
//! ```
//!
//!  Assume that an array A1, containing 100 elements, is to be added to the
//!    file. The array will be stored contiguously, beginning at the first free
//!    address. Thus, its initial and final addresses will be 1665 (start
//!    record 14, so 128 * 13 + 1) and 1764 ( 128 * 13 + 100), respectively.
//!    The entire array fits into a single record, so one element record will
//!    be added to the file. The value of NSUM in the summary record is
//!    incremented by one. The new value of FFA is the address following the
//!    final address of the new array: 1765. This is stored in the file record.
//!
//!  
//!
//! ```text
//!     1 { IDWORD='DAF/Xmpl', ND=25, NI=27, IFNAME='TESTFILE',
//!                            RI=12, RF=12, FFA=1765           }
//!     2
//!     .
//!     . Records 2 through 11 are reserved records.
//!     .
//!    11
//!    12 < NEXT=0, PREV=0, NSUM=1, (1665,1764),(0,0),(0,0) >
//!    13 < " " >
//!    14 < 100 > 100 words for A1
//! ```
//!
//!  Assume that a second array A2, containing 200 elements, is to be added
//!    to the file. The elements will be stored between addresses 1765 and
//!    1964. The array will fill the remainder of the first element record, all
//!    of a second record, and part of a third, so two element records will be
//!    added to the file. The value of NSUM in the summary record is
//!    incremented again. And the new value of FFA (1965) is stored in the file
//!    record.
//!
//!  
//!
//! ```text
//!     1 { IDWORD='DAF/Xmpl', ND=25, NI=27, IFNAME='TESTFILE',
//!                            RI=12, RF=12, FFA=1965           }
//!     2
//!     .
//!     . Records 2 through 11 are reserved records.
//!     .
//!    11
//!    12 < NEXT=0, PREV=0, NSUM=2, (1665,1764),(1765,1964),(0,0) >
//!    13 < " " >
//!    14 < 128 > 100 words for A1, 28 words for A2
//!    15 < 128 > 128 words for A2
//!    16 <  44 >  44 words for A2
//! ```
//!
//!  To add a third array A3, containing 150 elements, the process is
//!    repeated. The elements will be stored between addresses 1965 and 2114.
//!    The array will fill the remainder of the third element record, and part
//!    of a fourth, so one new element record is added. The value of NSUM is in
//!    the summary record is incremented again. And the new value of FFA (2115)
//!    is in the file record.
//!
//!  
//!
//! ```text
//!     1 { IDWORD='DAF/Xmpl', ND=25, NI=27, IFNAME='TESTFILE',
//!                            RI=12, RF=12, FFA=2115           }
//!     2
//!     .
//!     . Records 2 through 11 are reserved records.
//!     .
//!    11
//!    12 < NEXT=0, PREV=0, NSUM=3, (1665,1764),(1765,1964),(1965,2114) >
//!    13 < " " >
//!    14 < 128 > 100 words for A1, 28 words for A2
//!    15 < 128 > 128 words for A2
//!    16 < 128 >  44 words for A2, 84 words for A3
//!    17 <  66 >  66 words for A3
//! ```
//!
//!  Note that the final summary record is full, so new summary and name
//!    records will added to the file. (Record 17 will remain only partially
//!    filled.) The values of NEXT and PREV in the summary records are adjusted
//!    so that the records point to each other:
//!
//!  
//!
//! ```text
//!     1 { IDWORD='DAF/Xmpl', ND=25, NI=27, IFNAME='TESTFILE',
//!                            RI=12, RF=12, FFA=2115           }
//!     2
//!     .
//!     . Records 2 through 11 are reserved records.
//!     .
//!    11
//!    12 < NEXT=18, PREV=0, NSUM=3, (1665,1764),(1765,1964),(1965,2114) >
//!    13 < " " >
//!    14 < 128 > 100 words for A1, 28 words for A2
//!    15 < 128 > 128 words for A2
//!    16 < 128 >  44 words for A2, 84 words for A3
//!    17 <  66 >  66 words for A3
//!    18 < NEXT=0, PREV=12, NSUM=0, (0,0),(0,0),(0,0) >
//!    19 < " " >
//! ```
//!
//!  The file record is updated so that the value of RF points to the new
//!    summary record, and the value of FFA in the file record will point to
//!    the first word in the first record following the new name record
//!    (address 2433):
//!
//!  
//!
//! ```text
//!     1 { IDWORD='DAF/Xmpl', ND=25, NI=27, IFNAME='TESTFILE',
//!                            RI=12, RF=18, FFA=2433           }
//!     2
//!     .
//!     . Records 2 through 11 are reserved records.
//!     .
//!    11
//!    12 < NEXT=18, PREV=0, NSUM=3, (1665,1764),(1765,1964),(1965,2114) >
//!    13 < " " >
//!    14 < 128 > 100 words for A1, 28 words for A2
//!    15 < 128 > 128 words for A2
//!    16 < 128 >  44 words for A2, 84 words for A3
//!    17 <  66 >  66 words for A3
//!    18 < NEXT=0, PREV=12, NSUM=0, (0,0),(0,0),(0,0) >
//!    19 < " " >
//! ```
//!
//!  Adding more arrays is identical to the previous example: the necessary
//!    element records are added; the summary and name records are updated; and
//!    the value of FFA is updated. However, every third array also adds new
//!    summary and name records, and the values of RF and FFA are updated as
//!    well.
//!
//!  
//!
//!
//!  
//! #  Appendix A --- Summary of DAF Routines
//!
//!  Summary of Mnemonics
//!
//!  The following is a complete list of DAF API mnemonics and translations,
//!    in alphabetical order for each SPICE supported language.
//!
//!  
//!
//!
//!  
//! ##  SPICELIB (Fortran)
//!
//!  A few of the routines listed are entry points of another routine. If a
//!    routine is an entry point, the parent routine's name will be listed
//!    inside brackets preceding the mnemonic translation.
//!
//!  Many of the subroutines listed here are not normally used except to
//!    support other subroutines. For example, because the subroutines that
//!    read and write records ([DAFRCR](crate::raw::dafrcr), [DAFRDR](crate::raw::dafrdr), [DAFRFR](crate::raw::dafrfr), [DAFWCR](crate::raw::dafwcr), [DAFWDR](crate::raw::dafwdr), [DAFWFR](crate::raw::dafwfr))
//!    are low level routines, they are not usually called by a typical user,
//!    but instead by higher level DAF routines.
//!
//!  
//!
//! ```text
//!    DAFAC                Add comments
//!    DAFADA [DAFANA]      Add data to array
//!    DAFAH                Assign handles
//!    DAFANA               Add new array
//!    DAFARR               Add reserved records
//!    DAFARW [DAFRWD]      Address to record/word
//!    DAFBBS [DAFFA]       Begin backward search
//!    DAFBFS [DAFFA]       Begin forward search
//!    DAFBNA [DAFANA]      Begin new array
//!    DAFBT                Convert binary file to transfer file
//!    DAFCAD [DAFANA]      Continue adding data
//!    DAFCLS [DAFAH]       Close an open DAF
//!    DAFCS  [DAFFA]       Continue search
//!    DAFDC                Delete comments
//!    DAFEC                Extract comments
//!    DAFENA [DAFANA]      End new array
//!    DAFFA                Find array
//!    DAFFNA [DAFFA]       Find next array
//!    DAFFNH [DAFAH]       File name to handle
//!    DAFFPA [DAFFA]       Find previous array
//!    DAFGDA               Read data from address
//!    DAFGDR [DAFRWD]      Get double precision record
//!    DAFGH  [DAFFA]       Get handle
//!    DAFGN  [DAFFA]       Get array name
//!    DAFGS  [DAFFA]       Get summary
//!    DAFGSR [DAFRWD]      Get summary/descriptor record
//!    DAFHFN [DAFAH]       Handle to file name
//!    DAFHLU [DAFAH]       Handle to logical unit
//!    DAFHOF [DAFAH]       Handles of open files
//!    DAFHSF [DAFAH]       Handle to summary format
//!    DAFLUH [DAFAH]       Logical unit to handle
//!    DAFNRR [DAFRWD]      Number of reads, requests
//!    DAFONW [DAFAH]       Open new DAF for write
//!    DAFOPR [DAFAH]       Open DAF for read
//!    DAFOPW [DAFAH]       Open DAF for write
//!    DAFPS                Pack summary
//!    DAFRA                Re-order arrays
//!    DAFRCR               Read character record
//!    DAFRDA               Read data from address
//!    DAFRFR               Read file record
//!    DAFRN  [DAFFA]       Change array name
//!    DAFRRR               Remove reserved records
//!    DAFRS  [DAFFA]       Replace summary
//!    DAFRWA               Record/word to address
//!    DAFRWD               Read, write double precision
//!    DAFSIH [DAFAH]       Signal invalid handles
//!    DAFTB                Convert transfer file to binary file
//!    DAFUS  [DAFPS]       Unpack summary
//!    DAFWCR               Write character record
//!    DAFWDA               Write data to address
//!    DAFWDR [DAFRWD]      Write double precision record
//!    DAFWFR               Write file record
//!    DAFWS  [DAFFA]       Write summary
//! ```
//!
//!     
//! ###  Deprecated routines
//!
//!  
//!
//! ```text
//!    DAFA2B               ASCII to binary
//!    DAFB2A               Binary to ASCII
//!    DAFB2T               Binary to text
//!    DAFOPN [DAFAH]       Open new
//!    DAFRDA               Read data from address
//!    DAFRDR [DAFRWD]      Read double precision record
//!    DAFT2B               Text to binary
//! ```
//!
//!  [DAFA2B](crate::raw::dafa2b), [DAFB2A](crate::raw::dafb2a), DAFT2B, and [DAFB2T](crate::raw::dafb2t) were recommended for converting DAFs
//!    to the transfer format file. These routines are now obsolete; however,
//!    they will remain in SPICELIB for backwards compatibility. Use the
//!    routines [DAFTB](crate::raw::daftb) and [DAFBT](crate::raw::dafbt).
//!
//!  The routines [DAFGDA](crate::raw::dafgda) and [DAFGSR](crate::raw::dafgsr) supersede [DAFRDA](crate::raw::dafrda).
//!
//!  
//!
//!
//!  
//! ##  CSPICE (C)
//!
//!  
//!
//! ```text
//!    dafac_c              Add comments
//!    dafbbs_c             Begin backward search
//!    dafbfs_c             Begin forward search
//!    dafcls_c             Close
//!    dafcs_c              Continue search
//!    dafdc_c              Delete comments
//!    dafec_c              Extract comments
//!    daffna_c             Find next array
//!    daffpa_c             Find previous array
//!    dafgda_c             Read data from address
//!    dafgn_c              Get array name
//!    dafgs_c              Get summary
//!    dafgsr_c             Get summary/descriptor record
//!    dafopr_c             Open for read
//!    dafopw_c             Open for write
//!    dafps_c              Pack summary
//!    dafrda_c             Read data from address
//!    dafrfr_c             Read file record
//!    dafrs_c              Replace summary
//!    dafus_c              Unpack summary
//! ```
//!
//!     
//! ###  Deprecated routines
//!
//!  
//!
//! ```text
//!    dafrda_c             Read data from address
//! ```
//!
//!     
//! ##  Icy (IDL)
//!
//!  
//!
//! ```text
//!    cspice_dafac         Add comments
//!    cspice_dafbbs        Begin backward search
//!    cspice_dafbfs        Begin forward search
//!    cspice_dafcls        Close
//!    cspice_dafcs         Continue search
//!    cspice_dafdc         Delete comments
//!    cspice_dafec         Extract comments
//!    cspice_daffna        Find next array
//!    cspice_daffpa        Find previous array
//!    cspice_dafgda        Read data from address
//!    cspice_dafgn         Get array name
//!    cspice_dafgs         Get and unpack summary
//!    cspice_dafopr        Open for read
//!    cspice_dafopw        Open for write
//!    cspice_dafrs         Replace summary
//! ```
//!
//!     
//! ##  Mice (MATLAB)
//!
//!  
//!
//! ```text
//!    cspice_dafac         Add comments
//!    cspice_dafbbs        Begin backward search
//!    cspice_dafbfs        Begin forward search
//!    cspice_dafcls        Close
//!    cspice_dafcs         Continue search
//!    cspice_dafdc         Delete comments
//!    cspice_dafec         Extract comments
//!    cspice_daffna        Find next array
//!    cspice_daffpa        Find previous array
//!    cspice_dafgda        Read data from address
//!    cspice_dafgn         Get array name
//!    cspice_dafgs         Get and unpack summary
//!    cspice_dafopr        Open for read
//!    cspice_dafopw        Open for write
//! ```
//!
//!     
//! #  Appendix B --- Revision History
//!
//!  
//!
//!
//!  
//! ###  2017 MAR 22 by E. D. Wright, N. J. Bachman, and B. V. Semenov
//!
//!  Corrected small typo in fake Tex code that caused incorrect appearence
//!    of Appendix A in HTML format.
//!
//!  Updated code example to show calls to [DAFGN](crate::raw::dafgn) and [DAFRN](crate::raw::dafrn).
//!
//!  Corrected miscellaneous typos throughout text. Corrected CSPICE cell
//!    routine names.
//!
//!  Updated to point to SPY and BINGO User's Guides available from the NAIF
//!    server "Utilities" page.
//!
//!  
//!
//!
//!  
//! ###  2013 MAY 30 by E. D. Wright.
//!
//!  Edits to conform to current NAIF standards for Required Reading
//!    documents.
//!
//!  The document now includes description of CSPICE, Icy, and Mice DAF APIs.
//!
//!  
//!
//!
//!  
//! ###  2008 JAN 17 by B. V. Semenov.
//!
//!  Previous edits.
//!
//!  
//!
//!
//!  
//! ###  2002 JAN 13 by B. V. Semenov.
//!
//!  Added a brief discussion of the DAF run-time binary file format
//!    translation capability now present in the SPICE Toolkit.
//!
//!  
//!
//!
//!  
//! ###  1994 JUL 12 by NAIF.
//!
//!  The document differs from the previous version of September 1991 in that
//!    it includes the addition of new routines that should be used instead of
//!    existing SPICELIB routines. [DAFONW](crate::raw::dafonw) replaces [DAFOPN](crate::raw::dafopn), [DAFTB](crate::raw::daftb) replaces
//!    [DAFA2B](crate::raw::dafa2b) and DAFT2B, and DAFBT replaces [DAFB2A](crate::raw::dafb2a) and [DAFB2T](crate::raw::dafb2t). Also, the term
//!    to describe the non-binary DAF file, text, has been replaced with a more
//!    accurate term, transfer, indicating that the files are written in a
//!    format suitable for transfer from one platform to another.
//!
//!  
//!
//!
//!     
