//! #  DAS Required Reading
//!
//!  Last revised on 2021 DEC 31 by B. V. Semenov.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  The "Direct Access Segregated" (DAS) SPICE file architecture stores
//!    arrays of integer, double precision, and character data. SPICE DSK, EK
//!    and DBK files, and type 1 star catalogs use the DAS architecture and
//!    associated software.
//!
//!  
//!
//!
//!  
//! ##  Notes for Fortran Users
//!
//!  This document currently discusses only the Fortran version of the SPICE
//!    DAS subsystem.
//!
//!  For a given platform, DAS files used by the Fortran, C, IDL, and MATLAB
//!    SPICE Toolkits are identical.
//!
//!  
//!
//!
//!  
//! ##  Intended audience
//!
//!  This document is intended for SPICE users who need to use lower-level,
//!    segment-oriented Digital Shape Kernel (DSK) interface routines. It is
//!    also for sophisticated SPICE users who wish to create their own
//!    DAS-based file formats. Users of SPICE kernels based on the DAS format
//!    can find documentation for those kernels in the Required Reading for the
//!    appropriate kernel type.
//!
//!  
//!
//!
//!  
//! ##  Related Documents
//!
//!  The following documents pertinent to DAS usage are included in the SPICE
//!    Toolkit:
//!
//!  
//!
//! * DLA Required Reading ([dla.req](crate::required_reading::dla))
//!
//!  * Convert User's Guide ([convert.ug](crate::raw::convert.ug))
//!
//!  * COMMNT User's Guide ([commnt.ug](crate::raw::commnt.ug))
//!
//!     
//! ##  Introduction
//!
//!  The SPICELIB "DAS" subsystem consists of a Fortran 77 direct access
//!    file architecture and a set of subroutines that make it convenient to
//!    create, read, annotate, and transport files having this architecture.
//!
//!  The purpose of the DAS subsystem is to provide SPICELIB users with a
//!    simple way to create and efficiently use their own portable, high-level,
//!    binary, direct access file formats. The DAS architecture is meant to
//!    serve as a foundation for these high-level formats, providing services
//!    and capabilities required by all of them.
//!
//!  The principal advantage of building a new file format based on DAS is
//!    that most of the new access routines---the new code needed to support
//!    reading, writing, and porting files having the new format---can be built
//!    out of DAS routines. Thus a great deal of the design, coding,
//!    documentation and testing that accompanies the development of new access
//!    routines is automatically eliminated. In addition, because files
//!    conforming to the new format are also DAS files, they inherit the
//!    features common to all DAS files:
//!
//!  
//!
//! * Direct file access
//!
//!  * Simple interface---data is referenced by "logical address"
//!
//!  * Support for character, double precision, and integer data types
//!
//!  * Built-in portability
//!
//!  * Built-in file annotation capability
//!
//!  * System-independent I/O buffering
//!
//!  The acronym "DAS" stands for "direct access, segregated." The DAS
//!    subsystem enables application programs to view data in a DAS file as
//!    though the data were segregated into three separate, possibly empty,
//!    one-dimensional arrays: one of integers, one of double precision
//!    numbers, and one of characters. Each element of these virtual arrays is
//!    pointed to by an integer known as a "logical address." The logical
//!    addresses for each data type are independent: for each data type, the
//!    range of logical addresses starts at 1 and ends with the count of scalar
//!    data elements of that type. Thus an application program can, by calling
//!    DAS interface routines, read the "100th through 200th integers" or
//!    "first through 3000th characters" from a DAS file.
//!
//!  This simple view of the organization of data within a DAS file is
//!    independent of the order in which data are written to the file when it
//!    is created: double precision numbers, integers, and characters can be
//!    written to the file in an interleaved fashion, but the data of each type
//!    will still appear to be contiguous to a program reading the file.
//!
//!  The DAS suite of routines includes routines that convert DAS files
//!    between binary and transfer formats. DAS files in transfer format can be
//!    easily ported between computer systems having different binary numeric
//!    and character data representations. The SPICE Toolkit's TOBIN, TOXFR,
//!    and SPACIT utilities allow users to carry out these conversions via
//!    command-line or menu-based programs. Therefore, any high-level file
//!    format based on DAS can be ported using existing SPICE utilities.
//!
//!  The task of adding annotation, sometimes called "comments" or
//!    "metadata," to data files is also solved by using the DAS format. The
//!    SPICE Toolkit's COMMNT utility can insert annotation into, or extract
//!    annotation from, any DAS file, so this capability is automatically
//!    available for any files having a format based on DAS.
//!
//!  Because speed of file access can be important for programs that use
//!    large files, the DAS subsystem buffers in memory data that it reads from
//!    or writes to a DAS file. This buffering operation is completely
//!    independent of the I/O mechanism of the computer system on which a
//!    DAS-based application program runs. User-written access routines for any
//!    file format based on DAS will automatically support I/O buffering simply
//!    by calling DAS routines to carry out their I/O operations.
//!
//!  
//!
//!
//!  
//! #  DAS Concepts
//!
//!  
//!
//!
//!  
//! ##  Logical Addresses
//!
//!  The DAS subsystem makes a DAS file look to an application program as
//!    though the data in the file were segregated into three virtual arrays,
//!    one each of character, integer, and double precision type. Any or all of
//!    these arrays may be empty. In DAS terminology, each element of any of
//!    these "logical" arrays is called a "word." The phrases "integer
//!    word," "character word," and "double precision word" are used when
//!    specific data types are of interest. Character words are single
//!    characters.
//!
//!  The DAS logical address of a given word is the index of that word within
//!    its virtual array. When necessary to avoid ambiguity, we'll use the
//!    phrases "integer logical address," "double precision logical
//!    address," or "character logical address" to denote logical addresses
//!    of words of each data type. Logical addresses themselves, wherever
//!    they're used in DAS routines, are represented by integers.
//!
//!  The valid range of indices for each non-empty virtual array starts at 1
//!    and ends at the word count for that array. There are no "holes" in the
//!    arrays; every logical address in the valid range for an array points to
//!    data. This property of the virtual arrays is enforced by the mechanism
//!    by which DAS files are populated with data: data may be placed in a
//!    virtual array either by appending or by updating a range of valid
//!    logical addresses. Random write access to array addresses outside of the
//!    valid range is not supported.
//!
//!  The word counts for each logical array can be obtained via the routine
//!    [DASLLA](crate::raw::daslla) ("DAS, last logical addresses").
//!
//!  The ordering of the data within each virtual array is controlled by the
//!    order in which data are appended to that array. Each append operation
//!    adds the contents of an array in memory to the DAS virtual array of the
//!    same data type. An append operation appends the elements of the array in
//!    memory, preserving order, to a contiguous range of logical addresses,
//!    starting with the successor of the highest logical address already
//!    occupied in the virtual array. In the case of arrays of strings, the
//!    order of characters within an array increases from left to right, and
//!    all characters within an array element precede those belonging to any
//!    higher-indexed element.
//!
//!  
//!
//!
//!  
//! ##  Read and Write Access
//!
//!  The DAS subsystem provides a measure of system-independent protection
//!    for files that are intended to be accessed in a "read only" fashion. A
//!    program can open a DAS file for "read access" or "write access";
//!    when a DAS file is opened for read access, it cannot be modified by the
//!    DAS routines. A DAS file opened for write access can be read as well as
//!    written.
//!
//!  A program selects the access type of a DAS file by opening it with a
//!    call to the appropriate DAS routine: DASOPR opens files for read access;
//!    DASOPW opens existing DAS files for write access; and DASONW opens a new
//!    DAS file for write access. The routine DASOPS opens a new scratch DAS
//!    file for write access.
//!
//!  "Opening" a DAS file via one of the DAS file opening routines makes
//!    the file available for I/O, but the file is not necessarily physically
//!    opened from the perspective of the host file system. SPICE manages DAS
//!    files opened by any of these routines so the files behave as though
//!    they're physically open, as far as user application software is
//!    concerned.
//!
//!  For convenience, we still use the term "open" to refer to a DAS file
//!    that has been made available for read or write access by a call to a DAS
//!    file open routine.
//!
//!  A program is allowed to open a DAS file for read access multiple times
//!    during a single run, but once a file is opened for write access, any
//!    subsequent attempts to open it before it has been closed again cause an
//!    error to be signaled.
//!
//!  The file access restrictions imposed by the DAS subsystem do not pertain
//!    to access of a single DAS file by multiple programs. Limitations on this
//!    type of file access are system-dependent. However, NAIF recommends
//!    avoiding arrangements involving access by multiple programs to a DAS
//!    file that has been opened for writing by any one of them.
//!
//!  
//!
//!
//!  
//! ###  Access to Non-Native DAS Files
//!
//!  As of the N0066 version of the SPICE Toolkit, the DAS subsystem supports
//!    reading DAS files having certain non-native binary file formats: IEEE
//!    big-endian DAS files can be read on IEEE little-endian host systems, and
//!    vice versa. This capability involves run-time translation of numeric
//!    data.
//!
//!  Reading non-native DAS files may be substantially slower than reading
//!    native files; the performance cost of run-time translation depends in
//!    part on the fraction of read operations that reference buffered data.
//!
//!  Writing non-native DAS files is not supported. Non-native DAS files must
//!    be converted to native format in order to update or add to their data,
//!    or to modify their comment areas. See the Convert User's Guide
//!    ([convert.ug](crate::raw::convert.ug)) or the BINGO User's Guide for details.
//!
//!  
//!
//!
//!  
//! ##  File Handles
//!
//!  DAS routines that open DAS files return an integer argument called a
//!    "file handle." Most DAS routines use file handles rather than file
//!    names or Fortran logical units to specify DAS files. File handles can be
//!    compared more reliably and quickly than file names. Unlike logical
//!    units, the characteristics of DAS file handles are system-independent.
//!
//!  There is a one-to-one correspondence between file handles and DAS files
//!    that are known to a program. In particular, when a DAS file is opened
//!    for read access multiple times during a single program run, the same
//!    handle is returned each time.
//!
//!  The DAS subsystem does not recycle handles: once a file handle is
//!    assigned to a file, that integer will not be assigned to a different
//!    file during the same program run, even if the file to which the handle
//!    was assigned has been closed. At any time, a list of handles of open DAS
//!    files can be obtained by calling the DAS utility DASHOF.
//!
//!  
//!
//!
//!  
//! ##  The DAS Comment Area
//!
//!  The DAS file architecture supports a data structure called the "comment
//!    area." This feature allows DAS files to store free-form text comments,
//!    also known as metadata. Comments may be labels or other descriptive text
//!    that fully identifies the file and indicates its intended purpose.
//!
//!  The contents of the comment area must be printable text. The comment
//!    area is line-oriented; text inserted into the comment area can be
//!    retrieved with the original line breaks preserved.
//!
//!  The DAS file architecture itself imposes no limits on lengths of comment
//!    lines. However, one of the SPICE Toolkit DAS comment area writer
//!    routines, [DASACU](crate::raw::dasacu), does have a 255 character limit. In general, as the
//!    length of comment lines increases, the portability of the formatting of
//!    the comment text decreases. Limiting comment line lengths to 80
//!    characters, and avoiding use of tab characters, enhances the probability
//!    that comments will appear as intended when displayed.
//!
//!  
//!
//!
//!  
//! ##  Internal File Names
//!
//!  The DAS file format allows a 60-character string called an "internal
//!    file name" to be stored in a DAS file. Normally, this string is placed
//!    in the file when the file is created. The name may be retrieved at any
//!    time by calling [DASRFR](crate::raw::dasrfr).
//!
//!  The internal file name provides a mechanism for attaching an identifier
//!    to a DAS file in a manner independent of the file system on which the
//!    DAS file was created.
//!
//!  The internal file name capability existed before the DAS comment area
//!    feature was developed; the comment area provides much greater
//!    flexibility in annotating DAS files. NAIF suggests the comment area,
//!    rather than the internal file name, be used for system-independent file
//!    identification.
//!
//!  
//!
//!
//!  
//! ##  Binary and Transfer DAS files
//!
//!  The DAS subsystem provides routines that convert a binary DAS file to an
//!    equivalent transfer file, as well as routines that perform the inverse
//!    conversion. These routines can be used when it is necessary to move DAS
//!    files between platforms having incompatible binary architectures, and
//!    when run-time translation is not adequate because the files must be
//!    writable or efficiently readable on the target platform. In theory, it
//!    could also be the case that there are pairs of platforms for which
//!    run-time translation cannot be performed; this situation has existed in
//!    the past, but does not currently exist at the time of release of the
//!    N0066 SPICE Toolkit.
//!
//!  No SPICELIB routines other than the DAS conversion routines themselves
//!    depend on the DAS transfer format, and this format is not considered
//!    part of the DAS subsystem specification. Non-SPICELIB applications
//!    should avoid reliance on the specific format of DAS transfer files; this
//!    format may be updated by NAIF, though any previous transfer formats will
//!    continue to be supported.
//!
//!  The transfer format has the property of preserving all of the character,
//!    double precision, and integer information in the virtual arrays of DAS
//!    files; in particular, the full precision of all numeric quantities is
//!    preserved when a binary DAS file is converted to transfer format and
//!    then back to binary format on the same computer.
//!
//!  
//!
//!
//!  
//! ##  Buffering
//!
//!  When the DAS subsystem reads a physical record from a DAS file, or when
//!    it writes a physical record, a copy of the data in that record is saved
//!    in memory. We call this "buffering" the record. The subsystem buffers
//!    a limited number of the most recently accessed records of each data
//!    type.
//!
//!  Because the DAS subsystem buffers records, successive reads from or
//!    writes to the same record are generally more efficient than they would
//!    be if file I/O were performed in response to each read or write request.
//!    Because the DAS subsystem uses its own declared storage for buffering,
//!    this capability is guaranteed to be supported, regardless of the I/O
//!    buffering capabilities of the computer system on which the DAS subsystem
//!    is used.
//!
//!  The buffering performed by the DAS subsystem is automatic, and the
//!    existence of this feature can normally be safely ignored. The cases
//!    where it must be taken into account are:
//!
//!  
//!
//! * When the creation of a DAS file terminates abnormally
//!
//!  * When DAS file access speed must be optimized
//!
//!  In the first case, the contents of the DAS file may not contain all of
//!    the data "written" to the file by the DAS writing routines (DASADx and
//!    DASUDx), because some of that data may have been buffered but not
//!    written at the time the file creation was terminated.
//!
//!  In the second case, adjusting the sizes of the buffer arrays in [DASRWR](crate::raw::dasrwr)
//!    may improve performance. The sizes of the arrays are parameterized, so
//!    it is easy to change them. However, modifying SPICELIB code can create a
//!    maintenance burden for the owner of the modified code, because in order
//!    to accept a SPICELIB update (which could contain bug fixes), it will be
//!    necessary to modify the new versions of any previously modified SPICELIB
//!    routines.
//!
//!  
//!
//!
//!  
//! ##  Using Multiple DAS Files Concurrently
//!
//!  SPICE limits the total number of concurrently open DAS and DAF files to
//!    5000.
//!
//!  Accessing multiple DAS files concurrently may affect a program's speed
//!    of execution, since all of the files compete for the same space in the
//!    DAS record buffers.
//!
//!  
//!
//!
//!  
//! ##  DAS vs DAF
//!
//!  The DAS subsystem is, roughly speaking, a generalization of the DAF
//!    subsystem, since DAS supports storage of integer and character data as
//!    well as double precision data, whereas the DAF subsystem supports
//!    storage of double precision data only. The DAF format can easily be
//!    "simulated" using a DAS file.
//!
//!  On the other hand, the DAF subsystem provides higher-level services that
//!    are not built into DAS: in the DAF subsystem, the data are partitioned
//!    into a linked list of virtual arrays, each one having its own name and
//!    descriptor, and the subsystem provides routines to manipulate the arrays
//!    and their descriptive information.
//!
//!  As of the N0066 version of the SPICE Toolkit, a virtual array capability
//!    for DAS files is provided by the SPICE DLA subsystem. The DLA file
//!    format is a special case of the DAS format. DLA files enable user
//!    application software to view the data in a DAS file as a doubly linked
//!    list of virtual arrays, also called "segments," where each segment may
//!    have character, double precision, and integer components. See the DLA
//!    Required Reading, [dla.req](crate::required_reading::dla), for details.
//!
//!  
//!
//!
//!  
//! #  Using the DAS Subsystem
//!
//!  This chapter explains how to use the DAS subsystem in your own
//!    application program. The preceding chapter, "DAS Concepts," contains
//!    prerequisite information for the following discussion.
//!
//!  In the code examples shown in this chapter, we'll display a brief phrase
//!    describing the function of each DAS routine that appears in the code, as
//!    shown below:
//!
//!  
//!
//! ```text
//!    CALL DASCLS ( HANDLE )                             {Close DAS file}
//! ```
//!
//!     
//! ##  Creating a New DAS File
//!
//!  New DAS files are created by calling DASONW:
//!
//!  
//!
//! ```text
//!    CALL DASOPN ( FNAME, FTYPE, IFNAME, NCOMR, HANDLE )
//!  
//!                                                    {Open new DAS file}
//! ```
//!
//!  Here FNAME, FTYPE, IFNAME and NCOMR are inputs, and HANDLE is an output.
//!    FNAME is the name of the new file to be created, IFNAME is the internal
//!    file name of the new file, and NCOMR is the number of comment records to
//!    reserve. HANDLE is the integer handle assigned to the DAS file by
//!    DASOPN.
//!
//!  After this call, the DAS file is ready to be populated with data.
//!
//!  
//!
//!
//!  
//! ##  Writing DAS Files
//!
//!  There are two ways to write data to a DAS file: data can be added to the
//!    file by appending it to the virtual arrays, and data already in the
//!    virtual arrays can be updated.
//!
//!  The routines [DASADC](crate::raw::dasadc), [DASADD](crate::raw::dasadd), and [DASADI](crate::raw::dasadi) are used to add data to DAS
//!    files. These routines append data to the file's character, double
//!    precision, or integer logical arrays. As an example, we'll add some data
//!    of each type to the file FNAME from the example above. We'll start out
//!    with integer data.
//!
//!  [DASADI](crate::raw::dasadi) adds the contents of an integer array to a DAS file. In the call
//!    below, the argument HANDLE identifies the file, the number 100 is the
//!    number of elements in the data array, and DATAI is an array of integers
//!    whose contents are to be appended to the file's integer virtual array.
//!
//!  
//!
//! ```text
//!    DO I = 1, 100
//!       DATAI(I) = I
//!    END DO
//!  
//!    CALL DASADI ( HANDLE, 100, DATAI )              {Add data, integer}
//! ```
//!
//!  [DASADD](crate::raw::dasadd) is the double precision analog of [DASADI](crate::raw::dasadi). Since the logical
//!    arrays of different data types are independent, the double precision
//!    data from the array DATAD will occupy double precision logical addresses
//!    1 through 100.
//!
//!  
//!
//! ```text
//!    DO I = 1, 100
//!       DATAD(I) = DBLE(I)
//!    END DO
//!  
//!    CALL DASADD ( HANDLE, 100, DATAD )     {Add data, double precision}
//! ```
//!
//!  [DASADC](crate::raw::dasadc) is the character analog of [DASADI](crate::raw::dasadi). The second argument to DASADC
//!    indicates the total number of character logical addresses to be filled. [DASADC](crate::raw::dasadc) has two additional input arguments that DASADI and [DASADD](crate::raw::dasadd) do not
//!    have: begin and end substring bounds that indicate from which portion of
//!    the array of input strings to transfer data.
//!
//!  In this case, we'll presume that the array DATAC is declared
//!
//!  
//!
//! ```text
//!    CHARACTER*(80)            DATAC ( 2 )
//! ```
//!
//!  ```text
//!    DATAC(1) = 'Here''s the first line of character data.'
//!    DATAC(2) = 'And here''s the second line.'
//! ```
//!  The call below will write the contents of elements 1:2 of DATAC to DAS
//!    logical character addresses 1:160, because all of the characters of each
//!    element of DATAC are written to the file.
//!
//!  
//!
//! ```text
//!    CALL DASADC ( HANDLE, 160, 1, 80, DATAC )     {Add data, character}
//! ```
//!
//!  If we don't wish to write out all 80 characters of each element of
//!    DATAC, we can use different substring bounds in the call to [DASADC](crate::raw::dasadc). As
//!    an example, we'll add two more lines to the file, this time using only
//!    characters 1 through 50 in each element of DATAC:
//!
//!  
//!
//! ```text
//!    DATAC(1) = 'This is the third line.'
//!    DATAC(2) = 'This is the fourth line.'
//!  
//!    CALL DASADC ( HANDLE, 100, 1, 50, DATAC )     {Add data, character}
//! ```
//!
//!  This call added only 100 characters to the file.
//!
//!  We can continue to add data of any type to the file at this point. As an
//!    example, we'll add 100 more integers to the file.
//!
//!  
//!
//! ```text
//!    DO I = 1, 100
//!       DATAI(I) = I + 100
//!    END DO
//!  
//!    CALL DASADI ( HANDLE, 100, DATAI )              {Add data, integer}
//! ```
//!
//!  In addition to appending data to the virtual arrays in a DAS file, you
//!    can update array elements whose values have already been set. The DAS
//!    update routines are [DASUDC](crate::raw::dasudc), [DASUDD](crate::raw::dasudd), and [DASUDI](crate::raw::dasudi), which accept input
//!    arrays of character strings, double precision numbers, and integers,
//!    respectively.
//!
//!  It is not always convenient to set the values of the elements of a DAS
//!    file's virtual arrays in strictly increasing order; using the update
//!    capability, a program can fill in array elements with placeholder
//!    values, then update those values when their actual values are known.
//!
//!  To continue the example, we could update the first 100 integers in the
//!    file by negating them:
//!
//!  
//!
//! ```text
//!    DO I = 1, 100
//!       DATA(I) = -I
//!    END DO
//!  
//!    CALL DASUDI ( HANDLE, 1, 100, DATA )           {Update data,
//!                                                    integer}
//! ```
//!
//!  The routines [DASUDD](crate::raw::dasudd) and [DASUDC](crate::raw::dasudc) update double precision and character
//!    data in an analogous way.
//!
//!  To complete the new DAS file we've created, we close it using the
//!    routine [DASCLS](crate::raw::dascls):
//!
//!  
//!
//! ```text
//!    CALL DASCLS ( HANDLE )                         {Close DAS file}
//! ```
//!
//!  DAS files should never be closed directly by a Fortran CLOSE statement.
//!    In particular, DAS files that have been written to may not contain all
//!    of the data written unless [DASCLS](crate::raw::dascls) is used to close them.
//!
//!  At this point, the virtual arrays in our example DAS file have the
//!    following contents:
//!
//!  
//!
//! ```text
//!    Integer virtual array
//!    ---------------------
//!  
//!                                                   +-----+
//!         Integer logical address #1                | -1  |
//!                                                   +-----+
//!                                                      .
//!                                                      .
//!                                                      .
//!                                                   +-----+
//!         Integer logical address #100              |-100 |
//!                                                   +-----+
//!         Integer logical address #101              | 101 |
//!                                                   +-----+
//!                                                      .
//!                                                      .
//!                                                      .
//!                                                   +-----+
//!         Integer logical address #200              | 200 |
//!                                                   +-----+
//!  
//! ```
//!
//!  ```text
//!    Double precision virtual array
//!    ------------------------------
//!  
//!                                                   +-----+
//!         Double precision logical address #1       |1.D0 |
//!                                                   +-----+
//!                                                      .
//!                                                      .
//!                                                      .
//!                                                   +-----+
//!         Double precision logical address #100     |1.D2 |
//!                                                   +-----+
//! ```
//!  ```text
//!    Character virtual array
//!    -----------------------
//!  
//!                                                   +---+
//!         Character logical address #1              | H |
//!                                                   +---+
//!                                                   | e |
//!                                                   +---+
//!                                                   | r |
//!                                                   +---+
//!                                                   | e |
//!                                                   +---+
//!                                                   | ' |
//!                                                   +---+
//!                                                   | s |
//!                                                   +---+
//!                                                     .
//!                                                     .
//!                                                     .
//!                                                   +---+
//!         Character logical address #80             |   |
//!                                                   +---+
//!         Character logical address #81             | A |
//!                                                   +---+
//!                                                   | n |
//!                                                   +---+
//!                                                   | d |
//!                                                   +---+
//!                                                     .
//!                                                     .
//!                                                     .
//!                                                   +---+
//!         Character logical address #160            |   |
//!                                                   +---+
//!         Character logical address #161            | T |
//!                                                   +---+
//!                                                   | h |
//!                                                   +---+
//!                                                   | i |
//!                                                   +---+
//!                                                   | s |
//!                                                   +---+
//!                                                     .
//!                                                     .
//!                                                     .
//!                                                   +---+
//!         Character logical address #210            |   |
//!                                                   +---+
//!         Character logical address #211            | T |
//!                                                   +---+
//!                                                   | h |
//!                                                   +---+
//!                                                   | i |
//!                                                   +---+
//!                                                   | s |
//!                                                   +---+
//!                                                     .
//!                                                     .
//!                                                     .
//!                                                   +---+
//!         Character logical address #260            |   |
//!                                                   +---+
//!  
//! ```
//!     
//! ##  Writing to an Existing DAS file
//!
//!  The DASADx and DASUDx routines described above can be used to write to
//!    an old DAS file (one that contains data and is not currently open), as
//!    well as to a newly created one. To write to an old DAS file, open it for
//!    writing using DASOPW:
//!
//!  
//!
//! ```text
//!    CALL DASOPW ( FNAME, HANDLE )                  {Open DAS file for
//!                                                    writing}
//! ```
//!
//!  After this call, the DASADx and DASUDx routines can be used to add data
//!    to the file and update data in the file.
//!
//!  When the modification of the DAS file is complete, the file should be
//!    closed using [DASCLS](crate::raw::dascls).
//!
//!  
//!
//!
//!  
//! ##  Reading DAS Files
//!
//!  The DAS reading routines are [DASRDC](crate::raw::dasrdc), [DASRDD](crate::raw::dasrdd), and [DASRDI](crate::raw::dasrdi). Each of these
//!    routines reads data from a range of logical addresses. These routines
//!    can read from a DAS file that is open for reading or writing.
//!
//!  We'll continue our example by showing the results of reading the file we
//!    created.
//!
//!  We start out by reading a range of integer addresses. After the
//!    following sequence of calls, the array DATAI will contain the integers
//!    101 through 200.
//!
//!  
//!
//! ```text
//!    CALL DASOPR ( FNAME,  HANDLE )          {Open DAS file for reading}
//!  
//!    CALL DASRDI ( HANDLE, 101, 200, DATAI )        {Read data, integer}
//! ```
//!
//!  The following call will fill the array DATAI with the integers -1
//!    through -100:
//!
//!  
//!
//! ```text
//!    CALL DASRDI ( HANDLE, 1, 100, DATAI )          {Read data, integer}
//! ```
//!
//!  We can retrieve the double precision data from the file using [DASRDD](crate::raw::dasrdd).
//!    The following call will fill the array DATAD with the values 1.D0
//!    through 1.D2:
//!
//!  
//!
//! ```text
//!    CALL DASRDD ( HANDLE, 1, 100, DATAD )          {Read data, double
//!                                                    precision}
//! ```
//!
//!  Now for the character data: we'll read the data written by [DASADC](crate::raw::dasadc) using [DASRDC](crate::raw::dasrdc). After the call
//!
//!  
//!
//! ```text
//!    CALL DASRDC ( HANDLE, 1, 160, 1, 80, DATAC )  {Read data, character}
//! ```
//!
//!  the array DATAC (whose elements have a length of 80 characters) will
//!    contain the lines
//!
//!  
//!
//! ```text
//!    DATAC(1) = 'Here''s the first line of character data.'
//!    DATAC(2) = 'And here''s the second line.'
//! ```
//!
//!  Both elements of DATAC will be padded with trailing blanks.
//!
//!  A second call
//!
//!  
//!
//! ```text
//!    CALL DASRDC ( HANDLE, 161, 260, 1, 50, DATAC )
//! ```
//!
//!  will make the substring assignments
//!
//!  
//!
//! ```text
//!    DATAC(1)(1:50) =  'This is the third line.'
//!    DATAC(2)(1:50) =  'This is the fourth line.'
//! ```
//!
//!  The character substrings DATAC(1)(51:80) and DATAC(2)(51:80) are left
//!    unchanged by this call.
//!
//!  
//!
//!
//!  
//! ##  Converting Between Binary and Transfer Format
//!
//!  The DAS subsystem contains two routines that convert between DAS binary
//!    and transfer format. These are:
//!
//!  
//!
//! ```text
//!    DASBT
//!    DASTB
//! ```
//!
//!  [DASBT](crate::raw::dasbt) converts a binary DAS file to transfer format.
//!
//!  
//!
//! ```text
//!    CALL DASBT ( DAS, XFRLUN )         {Convert DAS to transfer file}
//!  
//! ```
//!
//!  .
//!
//!  [DASTB](crate::raw::dastb) is the inverse of this routine.
//!
//!  
//!
//! ```text
//!    CALL DASTB( XFRLUN, DAS )        {Convert transfer file to DAS}
//!  
//! ```
//!
//!  Since the SPICE Toolkit utility programs TOBIN and TOXFR are able to
//!    convert between binary and transfer DAS formats, SPICE Toolkit users
//!    should rarely need to make direct use of these conversion routines.
//!
//!  
//!
//!
//!  
//! ##  Annotating DAS Files
//!
//!  The comment area of a DAS file may be used to store comments, also known
//!    as metadata, in an encoded form. This capability is supported by the
//!    SPICELIB routines
//!
//!  
//!
//! *  [DASAC](crate::raw::dasac)
//!
//!
//!  Add comments to comment area.
//!
//!  *  [DASDC](crate::raw::dasdc)
//!
//!
//!  Delete comments from comment area.
//!
//!  *  [DASEC](crate::raw::dasec)
//!
//!
//!  Extract comments from comment area to a character array.
//!
//!  Normally, SPICELIB users will not need to manipulate the comment area
//!    using DAS API routines; the basic functionality of storing comments in a
//!    DAS file and extracting stored comments is provided by the SPICE Toolkit
//!    utility COMMNT. See the COMMNT user's guide, [commnt.ug](crate::raw::commnt.ug), for details.
//!
//!  
//!
//!
//!  
//! ##  Mapping Between Handles, File Names, and Units
//!
//!  Given a valid DAS handle, the corresponding file name and Fortran
//!    logical unit can be found by calling the appropriate DAS routine:
//!
//!  
//!
//! ```text
//!    CALL DASHFN ( HANDLE, FNAME )            { Handle to file name    }
//!    CALL DASHLU ( HANDLE, UNIT  )            { Handle to logical unit }
//! ```
//!
//!  For completeness, the inverse mapping routines are supplied:
//!
//!  
//!
//! ```text
//!    CALL DASFNH ( FNAME, HANDLE )            { File name to handle    }
//!    CALL DASLUH ( UNIT,  HANDLE )            { Logical unit to handle }
//! ```
//!
//!     
//! ##  Obtaining a DAS File Summary
//!
//!  Occasionally, an application program may need to know the range of
//!    logical addresses in use within a DAS file. This information can be
//!    obtained from the file's summary, which is returned by DASHFS:
//!
//!  
//!
//! ```text
//!    CALL DASHFS ( HANDLE, NRESV, FREE, LASTLA, LASTRC, LASTWD )
//!  
//!                                            { Handle to file summary }
//! ```
//!
//!  The desired information is in the output argument LASTLA, which is an
//!    array of the last logical addresses of character, double precision, and
//!    integer types, in that order. Given only this information, a program can
//!    read the contents of the file by using the DASRDx routines.
//!
//!  The other arguments of DASHFS will probably be of interest only to
//!    programmers creating new DAS subroutines. See the header of DASHFS for
//!    further information.
//!
//!  
//!
//!
//!  
//! ##  Using Scratch DAS Files
//!
//!  DAS files can be opened with a Fortran status of 'SCRATCH'. Scratch DAS
//!    files are always opened for write access. When they are closed, they are
//!    automatically deleted.
//!
//!  To obtain a scratch DAS file, use the routine DASOPS:
//!
//!  
//!
//! ```text
//!    CALL DASOPS ( HANDLE )                    { Open scratch DAS file }
//! ```
//!
//!  Here HANDLE is an output argument. DASOPS does not take a file name
//!    input argument because ANSI Standard Fortran 77 does not allow
//!    assignment of file names to scratch files. Since the file is temporary,
//!    there is no need to give it an internal file name or to reserve records
//!    in it.
//!
//!  As with any other DAS file, scratch DAS files should be closed by
//!    calling [DASCLS](crate::raw::dascls).
//!
//!  Application programs that need large amounts of temporary work space and
//!    that must be able to access that work space randomly can take advantage
//!    of the DAS subsystem: scratch DAS files are direct access files, provide
//!    buffered read and write access, and have the same simple interface as do
//!    permanent DAS files.
//!
//!  
//!
//!
//!  
//! #  DAS Architecture
//!
//!  The information in this section is provided for the sake of
//!    completeness; most SPICELIB users, including those designing their own
//!    DAS-based formats, will not need to know the details of the DAS
//!    architecture. NAIF strongly recommends that SPICELIB-based application
//!    programs access DAS files only through SPICELIB DAS access routines,
//!    which make the DAS format transparent to the calling program.
//!
//!  
//!
//!
//!  
//! ##  Overview of DAS Architecture
//!
//!  DAS files are Fortran direct access files having a record length long
//!    enough so that NWD double precision numbers, NWI integers, or NWC
//!    characters can fit in a single record. The values of these parameters
//!    are:
//!
//!  
//!
//! ```text
//!    NWC  =  1024
//!    NWD  =   128
//!    NWI  =   256
//! ```
//!
//!  Each record in a DAS file is dedicated to a specific purpose; records
//!    either contain data or control information that describes the structure
//!    and contents of the file. The types of records are:
//!
//!  
//!
//! * File records
//!
//!  * Comment records
//!
//!  * Directory records
//!
//!  * Character data records
//!
//!  * Double precision data records
//!
//!  * Integer data records
//!
//!  As the above list implies, each data record contains data of only one
//!    type.
//!
//!  Every DAS file contains one file record, which is always the first
//!    record of the DAS file. Comment records are optional. All DAS files
//!    contain at least one directory record, and useful DAS files also contain
//!    at least one data record.
//!
//!  The diagram below shows how records of different types are grouped
//!    within a general binary DAS file. The parenthesized numbers at the right
//!    indicate the number of records within the named group on the left.
//!
//!  
//!
//! ```text
//!    +------------------------+
//!    |      file record       |   ( 1 )
//!    +------------------------+
//!    |    reserved records    |   ( NRESVR )
//!    +------------------------+
//!    |                        |
//!    |                        |
//!    |    comment records     |   ( NCOMR )
//!    |                        |
//!    |                        |
//!    +------------------------+
//!    | first data directory   |   ( 1 )
//!    +------------------------+
//!    |      data records      |
//!    |                        |   ( variable )
//!    |                        |
//!    +------------------------+
//!                .
//!                .
//!                .
//!    +------------------------+
//!    | last data directory    |   ( 1 )
//!    +------------------------+
//!    |     data records       |
//!    |                        |   ( variable )
//!    |                        |
//!    +------------------------+
//! ```
//!
//!  Normally, when a new DAS file is closed, the data records are segregated
//!    into contiguous sets of records of each data type present in the file.
//!    The format of a DAS file after this re-organization is shown below:
//!
//!  
//!
//! ```text
//!    +------------------------+
//!    |      file record       |   ( 1 )
//!    +------------------------+
//!    |    reserved records    |   ( variable )
//!    +------------------------+
//!    |    comment records     |
//!    |                        |   ( variable )
//!    |                        |
//!    +------------------------+
//!    | first data directory   |   ( 1 )
//!    +------------------------+
//!    | character data records |
//!    |                        |   ( variable )
//!    |                        |
//!    +------------------------+
//!    |   d.p. data records    |
//!    |                        |   ( variable )
//!    |                        |
//!    +------------------------+
//!    |  integer data records  |
//!    |                        |   ( variable )
//!    |                        |
//!    +------------------------+
//! ```
//!
//!  The purpose of the re-organization is to simplify the process of mapping
//!    logical addresses to physical locations in the file, thereby speeding up
//!    access to data.
//!
//!  Note that the structure of the re-organized DAS file is just a special
//!    case of the general structure. All of the DAS routines that make
//!    explicit use of the structure of DAS files are equipped to work with the
//!    general structure: it is never assumed that data segregation has taken
//!    place.
//!
//!  
//!
//!
//!  
//! ##  The File Record
//!
//!  The file record contains the following information about the DAS file
//!    containing it:
//!
//!  
//!
//! * An ID word
//!
//!  * The internal file name
//!
//!  * The reserved record count
//!
//!  * The reserved character count
//!
//!  * The comment record count
//!
//!  * The comment character count
//!
//!  * The binary file format identification string
//!
//!  * The FTP error detection string
//!
//!  The ID word is a character string that identifies the file as a DAS
//!    file, as opposed to a DAF or some other type of file. The ID word
//!    contains eight characters, the first four of which are
//!
//!  
//!
//! ```text
//!    DAS/
//! ```
//!
//!  The contents of the file record are organized as follows:
//!
//!  
//!
//! ```text
//!    +-----------------------------------------------------+
//!    |IDWORD|IFNAME|NRR|NRC|NCR|NCC|BFF|<nulls>|FTP|<nulls>|
//!    +-----------------------------------------------------+
//! ```
//!
//!  where the abbreviations represent:
//!
//!  
//!
//! *  IDWORD
//!
//!
//!  is the DAS ID word.
//!
//!  *  IFNAME
//!
//!
//!  is the internal file name.
//!
//!  *  NRR
//!
//!
//!  is the number of reserved records.
//!
//!  *  NRC
//!
//!
//!  is the number of reserved characters---the number of characters in use
//! in the reserved record area.
//!
//!  *  NCR
//!
//!
//!  is the number of comment records.
//!
//!  *  NCC
//!
//!
//!  is the number of comment characters---the number of characters in use
//! in the comment area.
//!
//!  *  BFF
//!
//!
//!  is the binary file format identifier.
//!
//!  *  FTP
//!
//!
//!  is the FTP error detection string.
//!
//!  The file record can be read and updated at run time using [DASRFR](crate::raw::dasrfr) and [DASWFR](crate::raw::daswfr). [DASRFR](crate::raw::dasrfr) is normally used to read the internal file name of a DAS
//!    file. On the other hand, non-SPICELIB code should rarely, if ever, need
//!    to update the file record.
//!
//!  
//!
//!
//!  
//! ##  Reserved Records
//!
//!  The reserved record area is a data structure that serves as a hook for
//!    future development. SPICE users normally will have no reason to access
//!    reserved records, and indeed the DAS API supplies no access methods for
//!    these records.
//!
//!  Reserved records play no role in DAS operations, but their number must
//!    be known by the DAS subsystem. The number is set when a DAS file is
//!    created, and is automatically updated at run time if reserved records
//!    are added by calling DASARR.
//!
//!  
//!
//!
//!  
//! ##  Comment Records
//!
//!  Comment records are character records that constitute the DAS comment
//!    area.
//!
//!  
//!
//!
//!  
//! ##  Directory Records
//!
//!  Directory records, or "directories," are records that allow the DAS
//!    subsystem to find the physical location in a DAS file corresponding to a
//!    specified logical address.
//!
//!  Each directory record describes the data types of a number of data
//!    records that follow. Each directory also contains, for each data type,
//!    the lowest and highest logical address occurring in any of the records
//!    described by the directory.
//!
//!  The directories in a DAS file form a doubly linked list: each directory
//!    contains forward and backward pointers to the next and previous
//!    directories. Thus the list of directory records in a DAS can be viewed
//!    as shown below:
//!
//!  
//!
//! ```text
//!       NIL
//!        ^     +---------------------------------------------------+
//!        |     |                                                   |
//!        `-----|                                                   |
//!              |           First Directory Record                  |
//!        .---->|                                                   |
//!        |  .--|                                                   |
//!        |  |  +---------------------------------------------------+
//!        |  |                       .
//!        |  |                       .
//!        |  |                       .
//!        |  |  +---------------------------------------------------+
//!        |  `->|                                                   |
//!        `-----|                                                   |
//!              |           Second Directory Record                 |
//!        .---->|                                                   |
//!        |  .--|                                                   |
//!        |  |  +---------------------------------------------------+
//!        |  |
//!  
//!        .  .                       .
//!        .  .                       .
//!        .  .                       .
//!  
//!        |  |
//!        |  |  +---------------------------------------------------+
//!        |  `->|                                                   |
//!        `-----|                                                   |
//!              |            Last Directory Record                  |
//!              |                                                   |
//!           .--|                                                   |
//!           |  +---------------------------------------------------+
//!           V
//!          NIL
//! ```
//!
//!  After data segregation, the list of directories contains a single
//!    record.
//!
//!  The structure of each directory record is as follows:
//!
//!  
//!
//! ```text
//!    +-----------------------------------------------------------------+
//!    | <pointers> | <address ranges> |      <cluster descriptors>      |
//!    +-----------------------------------------------------------------+
//! ```
//!
//!  where the
//!
//!  
//!
//! ```text
//!    <pointers>
//! ```
//!
//!  section looks like
//!
//!  
//!
//! ```text
//!    +-----------------------------------------+
//!    | <backward pointer> | <forward pointer> |
//!    +-----------------------------------------+
//! ```
//!
//!  the
//!
//!  
//!
//! ```text
//!    <address ranges>
//! ```
//!
//!  section looks like
//!
//!  
//!
//! ```text
//!    +-------------------------------------------+
//!    | <char range> | <d.p. range> | <int range> |
//!    +-------------------------------------------+
//! ```
//!
//!  and each range looks like one of:
//!
//!  
//!
//! ```text
//!    +------------------------------------------------+
//!    | <lowest char address> | <highest char address> |
//!    +------------------------------------------------+
//!  
//!    +------------------------------------------------+
//!    | <lowest d.p. address> | <highest d.p. address> |
//!    +------------------------------------------------+
//!  
//!    +------------------------------------------------+
//!    | <lowest int address>  | <highest int address>  |
//!    +------------------------------------------------+
//! ```
//!
//!  When the set of data records described by a directory record contains no
//!    data of a given type, the address range corresponding to that type is
//!    0:0.
//!
//!  Following the pointers and address range information is a sequence of
//!    cluster descriptors. "Clusters" are maximal, contiguous sequences of
//!    data records of a given data type. By "maximal" we mean that each
//!    cluster is preceded and followed by a record that is not a data record
//!    of the cluster's type. Each cluster descriptor indicates the data type
//!    and record count of the cluster it describes. Successive cluster
//!    descriptors map to successive clusters of data records in the file, as
//!    shown below:
//!
//!  
//!
//! ```text
//!    +---------------------------------------------------+
//!    |    . . . | DESCR(i)  | DESCR(i+1) | . . .         | Directory
//!    +---------------|----------|------------------------+
//!                    |        . |
//!                    |        . |
//!                    |        . |
//!    +---------------|----------|------------------------+
//!    |               |          |                        | Data records
//!    |               V          |    Cluster (i)         |
//!    |                          |                        |
//!    +--------------------------|------------------------+
//!    |                          |                        |
//!    |                          V    Cluster (i+1)       |
//!    |                                                   |
//!    |                                                   |
//!    |                                                   |
//!    +---------------------------------------------------+
//!                             .
//!                             .
//!                             .
//! ```
//!
//!  Note that although the number of cluster descriptors in a directory
//!    record is limited, the number of records described by a directory is
//!    virtually arbitrary, because the number of records in each cluster may
//!    vary. In particular, after data segregation is performed, a DAS file
//!    contains only three clusters and requires a single directory in order to
//!    describe those clusters.
//!
//!  The cluster descriptors are implemented using a run-length encoding
//!    scheme. The first element of the series of descriptors occupies two
//!    integers; these represent a type code and a count. The rest of the
//!    descriptors are just signed counts; the data types of the clusters they
//!    describe are determined by the sign of the count and the data type of
//!    the previous descriptor.
//!
//!  
//!
//!
//!  
//! ##  Data Records
//!
//!  Each DAS data record contains data of only one type.
//!
//!  Within each DAS data record, word numbers start at one and increase up
//!    to NWI, NWD, or NWC: the number of words in an integer, double
//!    precision, or character data record. The organization of data records
//!    can be thought of as follows:
//!
//!  
//!
//! ```text
//!  
//!    Character records
//!    -----------------
//!  
//!         +------------------------------------+
//!         | | |           ...                | |
//!         +------------------------------------+
//!          1 2                               NWC
//!  
//!  
//!    Double precision records
//!    ------------------------
//!  
//!         +--------------------------------+
//!         |       |       |   ...  |       |
//!         +--------------------------------+
//!             1      2                NWD
//!  
//!  
//!    Integer records
//!    ---------------
//!  
//!         +--------------------------------+
//!         |   |   |       ...          |   |
//!         +--------------------------------+
//!           1   2                       NWI
//!  
//! ```
//!
//!  Each data word in a DAS file is unambiguously specified by its type, the
//!    number of the record containing it, and its word number.
//!
//!  
//!
//!
//!  
//! #  DAS Routines
//!
//!  
//!
//!
//!  
//! ##  Summary of Mnemonics
//!
//!  All of the subroutines in the DAS family have names beginning with the
//!    letters "DAS," followed by a two- or three-character mnemonic. For
//!    example, the routine that reads integer data from a DAS file is named
//!    [DASRDI](crate::raw::dasrdi), pronounced "DAS-R-D-I." The following is a complete list of
//!    mnemonics and translations, in alphabetical order.
//!
//!  
//!
//! ```text
//!    A2L  Map logical address to physical location
//!    AC   Add comments from buffer to file
//!    ACR  Add comment records to file
//!    ACU  Add comment records from logical unit to DAS file
//!    ADC  Add character data to file
//!    ADD  Add double precision data to file
//!    ADI  Add integer data to file
//!    BT   Convert binary to transfer format file
//!    CLS  Close file
//!    CUD  Create and update directories
//!    DC   Delete comments
//!    EC   Extract DAS comments into buffer
//!    ECU  Extract DAS comments to logical unit
//!    FM   File manager
//!    FNH  Map file name to handle
//!    HAM  Map file handle to access method
//!    HFN  Map handle to file name
//!    HFS  Map handle to file summary
//!    HLU  Map handle to logical unit
//!    HOF  Return handles of open DAS files
//!    IOC  Low-level character record I/O
//!    IOD  Low-level double precision record I/O
//!    IOI  Low-level integer record I/O
//!    LLA  Last logical addresses of each data type
//!    LLC  Low-level file close
//!    LUH  Map logical unit to handle
//!    ONW  Open a new DAS file.
//!    OPN  Open a new DAS file (obsolete: use DASONW)
//!    OPR  Open a DAS file for reading
//!    OPS  Open a scratch DAS file
//!    OPW  Open an existing DAS file for writing
//!    RCR  Remove comment records
//!    RDC  Read character data
//!    RDD  Read double precision data
//!    RDI  Read integer data
//!    RFR  Read file record
//!    RRC  Perform buffered read of character record
//!    RRD  Perform buffered read of double precision record
//!    RRI  Perform buffered read of integer record
//!    RWR  Read and write records
//!    SDR  Segregate data records
//!    SIH  Signal invalid handles
//!    TB   Convert transfer format file to binary
//!    UDC  Update character data
//!    UDD  Update double precision data
//!    UDI  Update integer data
//!    UFS  Update file summary
//!    URC  Update character record
//!    URD  Update double precision record
//!    URI  Update integer record
//!    WBR  Write buffered records
//!    WFR  Write file record
//!    WRC  Perform buffered write of character record
//!    WRD  Perform buffered write of double precision record
//!    WRI  Perform buffered write of integer record
//! ```
//!
//!     
//! ##  Summary of Calling Sequences
//!
//!  Calling sequences of the DAS family of routines are given below.
//!    Routines are grouped according to function.
//!
//!  The following routines are intended for use by both SPICELIB users'
//!    application programs and by SPICELIB routines.
//!
//!  Opening and closing files:
//!
//!  
//!
//! ```text
//!    DASONW ( FNAME, FTYPE,  IFNAME, NCOMR, HANDLE )
//!    DASOPN ( FNAME, IFNAME, NRESV,  HANDLE )
//!    DASOPW ( FNAME, HANDLE )
//!    DASOPR ( FNAME, HANDLE )
//!    DASOPS ( HANDLE )
//!    DASCLS ( HANDLE )
//! ```
//!
//!  Adding data to files:
//!
//!  
//!
//! ```text
//!    DASADC ( HANDLE, N, BPOS, EPOS, DATA )
//!    DASADD ( HANDLE, N,             DATA )
//!    DASADI ( HANDLE, N,             DATA )
//! ```
//!
//!  Updating data in files:
//!
//!  
//!
//! ```text
//!    DASUDC ( HANDLE, FIRST, LAST, BPOS, EPOS, DATA )
//!    DASUDD ( HANDLE, FIRST, LAST,             DATA )
//!    DASUDI ( HANDLE, FIRST, LAST,             DATA )
//! ```
//!
//!  Reading data from files:
//!
//!  
//!
//! ```text
//!    DASRDC ( HANDLE, FIRST, LAST, BPOS, EPOS, DATA )
//!    DASRDD ( HANDLE, FIRST, LAST,             DATA )
//!    DASRDI ( HANDLE, FIRST, LAST,             DATA )
//! ```
//!
//!  Mapping between file names, handles, and logical units:
//!
//!  
//!
//! ```text
//!    DASHLU ( HANDLE, UNIT   )
//!    DASHFN ( HANDLE, FNAME  )
//!    DASLUH ( UNIT,   HANDLE )
//!    DASFNH ( FNAME,  HANDLE )
//! ```
//!
//!  Conversion between binary and transfer format:
//!
//!  
//!
//! ```text
//!    DASBT ( BINARY, UNIT   )
//!    DASTB ( UNIT,   BINARY )
//! ```
//!
//!  File record (and internal file name) access:
//!
//!  
//!
//! ```text
//!    DASRFR ( HANDLE, IDWORD, IFNAME, NRESVR, NRESVC, NCOMR, NCOMC )
//!    DASWFR ( HANDLE, IDWORD, IFNAME, NRESVR, NRESVC, NCOMR, NCOMC )
//! ```
//!
//!  File summary access:
//!
//!  
//!
//! ```text
//!    DASHFS ( HANDLE, NRESVR, NRESVC, NCOMR, NCOMC,
//!             FREE,   LASTLA, LASTRC, LASTWD       )
//!    DASLLA ( HANDLE, LASTC,  LASTD,  LASTI )
//!    DASUFS ( HANDLE, NRESVR, NRESVC, NCOMR, NCOMC,
//!             FREE,   LASTLA, LASTRC, LASTWD       )
//! ```
//!
//!  Adding and extracting comment records:
//!
//!  
//!
//! ```text
//!    DASAC  ( HANDLE, N, BUFFER )
//!    DASACR ( HANDLE, N )
//!    DASACU ( COMLUN, BEGMRK, ENDMRK, INSBLN, HANDLE )
//!    DASEC  ( HANDLE, BUFSIZ, N,      BUFFER, DONE   )
//!    DASECU ( HANDLE, UNIT,   FOUND )
//! ```
//!
//!  The following routines are considered to be utilities and generally will
//!    not need to be called by application programs.
//!
//!  Buffered writing, updating, and reading of records:
//!
//!  
//!
//! ```text
//!    DASWRC ( HANDLE, RECNO, RECC )
//!    DASWRD ( HANDLE, RECNO, RECD )
//!    DASWRI ( HANDLE, RECNO, RECI )
//!    DASURC ( HANDLE, RECNO, FIRST, LAST, DATAC )
//!    DASURD ( HANDLE, RECNO, FIRST, LAST, DATAD )
//!    DASURI ( HANDLE, RECNO, FIRST, LAST, DATAI )
//!    DASRRC ( HANDLE, RECNO, FIRST, LAST, DATAC )
//!    DASRRD ( HANDLE, RECNO, FIRST, LAST, DATAD )
//!    DASRRI ( HANDLE, RECNO, FIRST, LAST, DATAI )
//! ```
//!
//!  Flushing buffer contents to files:
//!
//!  
//!
//! ```text
//!    DASWBR ( HANDLE )
//! ```
//!
//!  Low-level file I/O:
//!
//!  
//!
//! ```text
//!    DASIOC ( ACTION, UNIT, RECNO, RECC )
//!    DASIOD ( ACTION, UNIT, RECNO, RECD )
//!    DASIOI ( ACTION, UNIT, RECNO, RECI )
//! ```
//!
//!  Logical address to physical location mapping:
//!
//!  
//!
//! ```text
//!    DASA2L ( HANDLE, TYPE, ADDRSS, CLBASE, CLSIZE, RECNO, WORDNO )
//! ```
//!
//!  Directory creation and updating:
//!
//!  
//!
//! ```text
//!    DASCUD ( HANDLE, TYPE, NWORDS )
//! ```
//!
//!  Segregating data records:
//!
//!  
//!
//! ```text
//!    DASSDR ( HANDLE )
//! ```
//!
//!  File handle verification:
//!
//!  
//!
//! ```text
//!    DASHOF ( HSET )
//!    DASSIH ( HANDLE, ACCESS )
//!    DASHAM ( HANDLE, ACCESS )
//! ```
//!
//!  Low-level file close utility:
//!
//!  
//!
//! ```text
//!    DASLLC ( HANDLE )
//! ```
//!
//!  The following routines are umbrella subroutines. These should never be
//!    called, but their headers may be of interest to general SPICELIB users,
//!    since they contain "global" documentation pertaining to the routines'
//!    entry points.
//!
//!  
//!
//! ```text
//!    DASFM
//!    DASRWR
//! ```
//!
//!     
//! #  Examples
//!
//!  
//!
//!
//!  
//! ##  Storing a Symbol Table in a DAS File
//!
//!  This example shows how the contents of a SPICELIB double precision
//!    symbol table can be stored in and retrieved from a DAS file. The source
//!    code for two subroutines WRSYMD and RDSYMD that perform the write and
//!    read functions is listed below. With a small number of changes, the code
//!    shown here could be modified to deal with integer or character symbol
//!    tables.
//!
//!  An important fact about user-designed, DAS-based formats is illustrated
//!    by this example: the high level format of a file must be understood by
//!    the code that reads the file, as well as the code that writes it.
//!    Therefore, most non-trivial user-designed formats will need to include a
//!    data structure that describes the rest of the file, thereby allowing the
//!    user's file reader to know how to read the file. In the following
//!    example, this data structure is very simple: it is a series of three
//!    integers whose meanings are known to the reader.
//!
//!  A SPICELIB double precision symbol consists of three arrays: an array of
//!    symbol names, an array of integers that serves to map names to their
//!    associated values, and an array of double precision values. WRSYMD adds
//!    the contents of a symbol table to a DAS file opened by the calling
//!    application. For each symbol, WRSYMD will add to the DAS file the
//!    symbol's name, the count of the values associated with that symbol, and
//!    the values associated with the symbol. In addition, the number of
//!    symbols in the symbol table, the total number of values associated with
//!    the symbols, and the length of the strings containing the symbols' names
//!    will be added to the file. The logical format of the resulting file can
//!    be thought of as follows:
//!
//!  
//!
//! ```text
//!  
//!  
//!    +----------------+   +----------------+   +----------------+
//!    |  Symbol Names  |   |  Symbol Count  |   |  D.P. Values   |
//!    |                |   +----------------+   |  Associated    |
//!    |                |   |  Value Count   |   |  With Symbols  |
//!    |                |   +----------------+   |                |
//!    |                |   |  Name Length   |   |  (possibly     |
//!    |                |   +----------------+   |   multiple     |
//!    |                |   |                |   |   values per   |
//!    |                |   | Value Counts   |   |   symbol)      |
//!    |                |   | For Each Symbol|   |                |
//!    |                |   |                |   |                |
//!    |                |   |                |   |                |
//!    |                |   |                |   |                |
//!    |                |   +----------------+   |                |
//!    |                |                        |                |
//!    +----------------+                        |                |
//!                                              |                |
//!                                              |                |
//!                                              +----------------+
//!       DAS Character        DAS integer          DAS double precision
//!       virtual array        virtual array        virtual array
//!  
//! ```
//!
//!  The source code of WRSYMD is shown below:
//!
//!  
//!
//! ```text
//!  
//!  
//!          SUBROUTINE WRSYMD ( HANDLE, SYMNAM, SYMPTR, SYMVAL )
//!          IMPLICIT NONE
//!    C
//!    C     Write a double precision symbol table to a DAS file.
//!    C
//!          INTEGER               LBCELL
//!          PARAMETER           ( LBCELL = - 5)
//!  
//!          INTEGER               HANDLE
//!          CHARACTER*(*)         SYMNAM ( LBCELL : * )
//!          INTEGER               SYMPTR ( LBCELL : * )
//!          DOUBLE PRECISION      SYMVAL ( LBCELL : * )
//!  
//!    C$ Brief_I/O
//!    C
//!    C     VARIABLE  I/O  DESCRIPTION
//!    C     --------  ---  ----------------------------------------------
//!    C     HANDLE     I   Handle of DAS file opened for writing.
//!    C     SYMSYM,
//!    C     SYMPTR,
//!    C     SYMVAL     I   Components of the symbol table.
//!    C
//!  
//!    C
//!    C     SPICELIB functions
//!    C
//!          INTEGER               CARDC
//!          INTEGER               CARDD
//!          INTEGER               SYDIMD
//!  
//!          LOGICAL               RETURN
//!  
//!    C
//!    C     Local parameters
//!    C
//!          INTEGER               MAXVAL
//!          PARAMETER           ( MAXVAL = 5000 )
//!  
//!    C
//!    C     Local variables
//!    C
//!          DOUBLE PRECISION      VALUES ( MAXVAL )
//!  
//!          INTEGER               I
//!          INTEGER               N
//!          INTEGER               NAMLEN
//!          INTEGER               NSYM
//!          INTEGER               NVAL
//!  
//!          LOGICAL               FOUND
//!  
//!    C
//!    C     Standard SPICELIB error handling.
//!    C
//!          IF ( RETURN() ) THEN
//!             RETURN
//!          END IF
//!  
//!          CALL CHKIN ( 'WRSYMD' )
//!  
//!    C
//!    C     Write the symbol and symbol value counts to the file. Also
//!    C     write out the length of the symbol name strings, so a reader
//!    C     program can know how many characters to read to obtain each
//!    C     name.
//!    C
//!          NSYM    =  CARDC (SYMNAM)
//!          NVAL    =  CARDD (SYMVAL)
//!          NAMLEN  =  LEN ( SYMNAM(1) )
//!  
//!          CALL DASADI ( HANDLE, 1, NSYM   )
//!          CALL DASADI ( HANDLE, 1, NVAL   )
//!          CALL DASADI ( HANDLE, 1, NAMLEN )
//!    C
//!    C     Now write out the symbol table entries. For each symbol,
//!    C     we'll append the symbol's name to the DAS's virtual character
//!    C     array, the value count to the virtual integer array, and the
//!    C     values to the virtual double precision array.
//!    C
//!          DO I = 1, NSYM
//!    C
//!    C        Look up the values associated with each symbol. We don't
//!    C        need to check the FOUND flag because we already know the
//!    C        symbols exist. Check the count of values; we must be able
//!    C        to fit them into our local VALUES array.
//!    C
//!             N = SYDIMD ( SYMNAM(I), SYMNAM, SYMPTR, SYMVAL )
//!  
//!             IF ( N .GT. MAXVAL ) THEN
//!  
//!                CALL SETMSG ( 'Symbol # has too many values: '
//!         .      //            'value count = #; array size = '
//!         .      //            '#.'                            )
//!                CALL ERRCH  ( '#', SYMNAM(I)                  )
//!                CALL ERRINT ( '#', N                          )
//!                CALL ERRINT ( '#', MAXVAL                     )
//!                CALL SIGERR ( 'ARRAYOVERFLOW'                 )
//!                CALL CHKOUT ( 'WRSYMD'                        )
//!                RETURN
//!  
//!             END IF
//!  
//!             CALL SYGETD ( SYMNAM(I),
//!         .                 SYMNAM,    SYMPTR,    SYMVAL,
//!         .                 N,         VALUES,    FOUND   )
//!    C
//!    C        Add the symbol's name to the DAS file.
//!    C
//!             CALL DASADC ( HANDLE, NAMLEN, 1, NAMLEN, SYMNAM(I) )
//!    C
//!    C        Now the value count.
//!    C
//!             CALL DASADI ( HANDLE,  1,  N  )
//!    C
//!    C        Finally, the values themselves.
//!    C
//!             CALL DASADD ( HANDLE,  N,  VALUES )
//!  
//!          END DO
//!  
//!          CALL CHKOUT ( 'WRSYMD' )
//!          RETURN
//!          END
//!  
//! ```
//!
//!  The symbol table placed in a DAS file by WRSYMD may be extracted by
//!    RDSYMD, shown below:
//!
//!  
//!
//! ```text
//!          SUBROUTINE RDSYMD ( HANDLE, SYMNAM, SYMPTR, SYMVAL )
//!          IMPLICIT NONE
//!    C
//!    C     Read a double precision symbol table from a DAS file.
//!    C
//!          INTEGER               LBCELL
//!          PARAMETER           ( LBCELL = - 5)
//!  
//!          INTEGER               HANDLE
//!          CHARACTER*(*)         SYMNAM ( LBCELL : * )
//!          INTEGER               SYMPTR ( LBCELL : * )
//!          DOUBLE PRECISION      SYMVAL ( LBCELL : * )
//!  
//!    C$ Brief_I/O
//!    C
//!    C     VARIABLE  I/O  DESCRIPTION
//!    C     --------  ---  ----------------------------------------------
//!    C     HANDLE     I   Handle of DAS file opened for writing.
//!    C     SYMSYM,
//!    C     SYMPTR,
//!    C     SYMVAL     O   Components of the symbol table. These cells
//!    C                    are presumed to be initialized on input.
//!    C
//!  
//!    C
//!    C     SPICELIB functions
//!    C
//!          LOGICAL               RETURN
//!  
//!    C
//!    C     Local parameters
//!    C
//!          INTEGER               CHAR
//!          PARAMETER           ( CHAR   = 1 )
//!  
//!          INTEGER               DP
//!          PARAMETER           ( DP     = 2 )
//!  
//!          INTEGER               INT
//!          PARAMETER           ( INT    = 3 )
//!  
//!          INTEGER               MAXVAL
//!          PARAMETER           ( MAXVAL = 5000 )
//!  
//!          INTEGER               MAXLEN
//!          PARAMETER           ( MAXLEN =   80 )
//!  
//!    C
//!    C     Local variables
//!    C
//!          CHARACTER*(MAXLEN)    VARNAM
//!  
//!          DOUBLE PRECISION      VALUES ( MAXVAL )
//!  
//!          INTEGER               FIRST ( 3 )
//!          INTEGER               I
//!          INTEGER               L
//!          INTEGER               LAST  ( 3 )
//!          INTEGER               N
//!          INTEGER               NAMLEN
//!          INTEGER               NSYM
//!          INTEGER               NVAL
//!  
//!    C
//!    C     Standard SPICELIB error handling.
//!    C
//!          IF ( RETURN() ) THEN
//!             RETURN
//!          END IF
//!  
//!          CALL CHKIN ( 'RDSYMD' )
//!  
//!    C
//!    C     Read the symbol and symbol value counts from the file. Also
//!    C     get the length of the symbol name strings.
//!    C
//!          CALL DASRDI ( HANDLE, 1, 1, NSYM   )
//!          CALL DASRDI ( HANDLE, 2, 2, NVAL   )
//!          CALL DASRDI ( HANDLE, 3, 3, NAMLEN )
//!    C
//!    C     Now read the symbols, their value counts, and their values.
//!    C     Add each symbol to the output symbol table.
//!    C
//!    C     Obtain the length of the names in the symbol table. Check
//!    C     that the elements of the input name array are long enough.
//!    C
//!          L  =  LEN ( SYMNAM(1) )
//!  
//!          IF ( L .LT. NAMLEN ) THEN
//!  
//!             CALL SETMSG ( 'Name array has width #; required width '
//!         .   //            'is #'                                   )
//!             CALL ERRINT ( '#',  L                                  )
//!             CALL ERRINT (  NAMLEN                                  )
//!             CALL SIGERR ( 'NAMEARRAYTOONARROW'                     )
//!             CALL CHKOUT ( 'RDSYMD'                                 )
//!             RETURN
//!  
//!          END IF
//!  
//!    C
//!    C     Initialize our logical address ranges.
//!    C
//!          CALL CLEARI ( 3, FIRST )
//!  
//!          LAST(CHAR) = 0
//!          LAST(DP)   = 0
//!          LAST(INT)  = 3
//!  
//!          DO I = 1, NSYM
//!    C
//!    C        Obtain the symbol's name.
//!    C
//!             FIRST(CHAR)  =  LAST(CHAR)  +  1
//!             LAST (CHAR)  =  LAST(CHAR)  +  NAMLEN
//!  
//!             CALL DASRDC (  HANDLE,
//!         .                  FIRST(CHAR),  LAST(CHAR),
//!         .                  1,            NAMLEN,      VARNAM  )
//!    C
//!    C        Now obtain the symbol's value count.
//!    C
//!             FIRST(INT)   =  LAST(INT)   +  1
//!             LAST (INT)   =  LAST(INT)   +  1
//!  
//!             CALL DASRDI ( HANDLE, FIRST(INT), LAST(INT), N )
//!  
//!             IF ( N .GT. MAXVAL ) THEN
//!  
//!                CALL SETMSG ( 'Symbol # has too many values: '
//!         .      //            'value count = #; array size = '
//!         .      //            '#.'                            )
//!                CALL ERRCH  ( '#', VARNAM                     )
//!                CALL ERRINT ( '#', N                          )
//!                CALL ERRINT ( '#', MAXVAL                     )
//!                CALL SIGERR ( 'ARRAYOVERFLOW'                 )
//!                CALL CHKOUT ( 'RDSYMD'                        )
//!                RETURN
//!  
//!             END IF
//!    C
//!    C        Now get the symbol's values.
//!    C
//!             FIRST(DP)    =  LAST(DP)    +  1
//!             LAST (DP)    =  LAST(DP)    +  N
//!  
//!             CALL DASRDD ( HANDLE, FIRST(DP), LAST(DP), VALUES )
//!    C
//!    C        Add the symbol to the output symbol table.
//!    C
//!             CALL SYPUTD (  VARNAM(1:NAMLEN),  VALUES,  N,
//!         .                  SYMNAM,            SYMPTR,  SYMVAL  )
//!  
//!          END DO
//!  
//!          CALL CHKOUT ( 'RDSYMD' )
//!          RETURN
//!          END
//! ```
//!
//!     
//! #  Appendix A --- Revision History
//!
//!  
//!
//!
//!  
//! ###  2021 DEC 31 by B. V. Semenov.
//!
//!  Removed bingo"dot"ug string which was causing broken links in HTML
//!    documentation in CSPICE-based packages that did not include BINGO.
//!
//!  
//!
//!
//!  
//! ###  2021 NOV 01 by N. J. Bachman.
//!
//!  Corrected the diagram showing the layout of DAS file records. Reordered
//!    lists of file record elements to match the actual order of the elements
//!    in the records.
//!
//!  
//!
//!
//!  
//! ###  2017 MAR 23 by N. J. Bachman.
//!
//!  Updated to mention:
//!
//!  
//!
//! * run-time translation
//!
//!  * handle management
//!
//!  * new limit on number of open DAS files
//!
//!  * DSK files
//!
//!  * the DLA format
//!
//!  * DAS comment area access routines
//!
//!  Errors in example code were corrected. Document organization was
//!    updated. Revision history was added.
//!
//!  
//!
//!
//!  
//! ###  2009 APR 01 by B. V. Semenov.
//!
//!  Edits to conform to current NAIF standards for Required Reading
//!    documents.
//!
//!  
//!
//!
//!  
//! ###  2002 JAN 15 by N. J. Bachman
//!
//!  Initial release.
//!
//!  
//!
//!
//!     
