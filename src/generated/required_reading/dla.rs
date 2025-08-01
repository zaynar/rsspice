//! #  DLA Required Reading
//!
//!  Last revised on 2017 APR 03 by N. J. Bachman.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  The "Doubly Linked Array" (DLA) SPICE file format enables data in a
//!    DAS file to be treated as a doubly linked list of virtual arrays, where
//!    each virtual array may contain integer, double precision, and character
//!    components.
//!
//!  
//!
//!
//!  
//! ##  Intended audience
//!
//!  This document is intended primarily for SPICE users who wish to gain a
//!    detailed understanding of the DLA format, particularly users who wish to
//!    create DSK files and sophisticated SPICE users who wish to create their
//!    own DLA-based file formats.
//!
//!  It may also be of interest to SPICE users who wish to access the
//!    segment-oriented routines of the DSK subsystem; however, such users may
//!    find adequate guidance in the code examples provided in API
//!    documentation of those DSK routines and of the DLA routines referenced
//!    by DSK documentation.
//!
//!  Users of DSK files can find documentation of the DSK interface routines
//!    in the DSK Required Reading file, [dsk.req](crate::required_reading::dsk).
//!
//!  
//!
//!
//!  
//! ##  Related Documents
//!
//!  The following documents pertinent to DLA usage are included in the SPICE
//!    Toolkit:
//!
//!  
//!
//! * DAS Required Reading
//!
//!  * DSK Required Reading
//!
//!  * Convert User's Guide
//!
//!  * COMMNT User's Guide
//!
//!     
//! ##  Introduction
//!
//!  The SPICE Doubly Linked Array (DLA) subsystem consists of a Fortran 77
//!    direct access file format and a set of subroutines that make it
//!    convenient to create, read, annotate, and transport files having this
//!    format. The same DLA file format is used by all language versions of
//!    SPICE.
//!
//!  The DLA file format is an instance of the DAS architecture. DLA files
//!    enable user application software to view the data in a DAS file as a
//!    doubly linked list of virtual arrays, usually called "segments," where
//!    each segment may contain character, double precision, and integer
//!    components.
//!
//!  DLA segments provide a mechanism for grouping related data. For example,
//!    in the SPICE DSK format, which is a specialization of the DLA format,
//!    data are grouped into segments; each DSK segment is a DLA segment as
//!    well. DSK segments contain data representing shapes of extended objects.
//!    Such representations may use both integer and double precision data. For
//!    example, in DSK data type 2, segments contain double precision numbers
//!    that represent the vertex coordinates of triangular plates, and integers
//!    that indicate which vertices belong to each plate.
//!
//!  DLA files may be thought of as a functional generalization of DAF files:
//!    DAF arrays contain only double precision data; DLA segments contain data
//!    of multiple types. However, there are some differences that prevent
//!    either format from being a special case of the other:
//!
//!  
//!
//! * DLA is based on the DAS low-level architecture
//!
//!  * DAS access methods differ from DAF methods
//!
//!  * DLA descriptors contain only segment location and size information; DAF
//! descriptors contain both location information and information describing
//! segment contents.
//!
//!  Applications can locate DLA segments by means of forward or backward
//!    linear searches. DLA routines are provided to start a forward or
//!    backward search, and to find the next or previous segment relative to a
//!    given segment.
//!
//!  In the SPICE Toolkit, functionality that makes use of DLA segments' data
//!    is provided by higher-level code, as is the case for the DSK subsystem.
//!
//!  
//!
//!
//!  
//! #  DLA Files
//!
//!  This chapter discusses details of the DLA file format.
//!
//!  
//!
//!
//!  
//! ##  DLA File Structure
//!
//!  DLA files are a special case of DAS files; they inherit all of the
//!    characteristics of DAS files. DAS files, once they have been written,
//!    are normally "segregated," meaning that the character, double
//!    precision, and integer records of a DAS file form three contiguous
//!    components of the respective data types. See the DAS Required Reading,
//!    [das.req](crate::required_reading::das), for details.
//!
//!  The underlying DAS architecture, in its segregated form, is:
//!
//!  
//!
//! ```text
//!         File component          Record count
//!         ==============          ============
//!  
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
//!  The DLA format presents a higher-level view of the DAS architecture. For
//!    most purposes, DLA files can be thought of as having the following
//!    logical components:
//!
//!  
//!
//! ```text
//!    +----------------------------+
//!    |       file record          |
//!    +----------------------------+
//!    |       comment area         |
//!    |                            |
//!    |                            |
//!    +----------------------------+
//!    | DLA format version         |
//!    +----------------------------+
//!    | DLA forward and backward   |
//!    | pointers                   |
//!    +----------------------------+
//!    | DLA descriptors            |
//!    +----------------------------+
//!    | first virtual DLA segment  |
//!    +----------------------------+
//!    | second virtual DLA segment |
//!    |                            |
//!    |                            |
//!    +----------------------------+
//!               ...
//!    +----------------------------+
//!    | last virtual DLA segment   |
//!    |                            |
//!    +----------------------------+
//! ```
//!
//!  The diagram above does not show the physical order of the segments,
//!    since segment data are distributed across the character, double
//!    precision, and integer records of the file, and since DLA descriptors
//!    are interleaved with DLA segments' integer components.
//!
//!  Details of these components are discussed below.
//!
//!  
//!
//!
//!  
//! ###  The DLA File Record
//!
//!  In a DLA file, the file record conforms to the DAS file record format.
//!    See the DAS Required Reading, [das.req](crate::required_reading::das), for a complete description of
//!    this format. Within the file record, the ID word has the format
//!
//!  
//!
//! ```text
//!    DAS/xxxx
//! ```
//!
//!  where the pattern
//!
//!  
//!
//! ```text
//!    xxxx
//! ```
//!
//!  represents up to four printing characters identifying the file type. For
//!    example, in DSK files, the ID word is
//!
//!  
//!
//! ```text
//!    DAS/DSK
//! ```
//!
//!  In general the portion of the ID word indicating file type need not
//!    indicate explicitly that the file is a DLA file. That fact can be
//!    derived from the file type string if the type is known to the SPICE
//!    system.
//!
//!  
//!
//!
//!  
//! ###  DLA Comment Area
//!
//!  All properties of the DLA comment area are inherited from the DAS
//!    architecture. DAS routines are used to access the comment area
//!    programmatically. The SPICE COMMNT utility can access the DLA comment
//!    area interactively.
//!
//!  
//!
//!
//!  
//! ###  DLA Format Version
//!
//!  The first member of a DLA file's integer address space is a parameter
//!    indicating the format version of the file. The primary purpose of this
//!    parameter is to indicate that the file is a DLA file. If the format is
//!    updated, this version will allow SPICE software to identify the version
//!    and access the file appropriately.
//!
//!  SPICE identifies DLA files using a combination of three attributes:
//!
//!  
//!
//! *  1. The file architecture, which must be DAS
//!
//!  *  2. The file type, which must be recognized as one derived from the DLA format,
//! for example DSK
//!
//!  *  3. The DLA format version
//!
//!     
//! ###  DLA Forward and Backward Pointers
//!
//!  The segments of a DLA file are organized as a doubly linked list. The
//!    DLA file contains pointers to data structures called "segment
//!    descriptors," which are associated with segments, at the head and tail
//!    of the list. These pointers are used to initiate forward and backward
//!    searches.
//!
//!  
//!
//!
//!  
//! ###  DLA Segment Descriptors
//!
//!  A DLA "segment descriptor," or simply "descriptor," is a small array
//!    of integers that indicate the sizes and addresses of the components of a
//!    corresponding DLA segment. A DLA descriptor also contains integers that
//!    act as forward and backward pointers to other DLA descriptors in the DLA
//!    file's doubly linked segment list.
//!
//!  DLA descriptors are also data structures used as input and output
//!    arguments of DLA routines. They are also used extensively as arguments
//!    to DSK routines; the DSK subsystem is currently the only other SPICE
//!    subsystem where they're used. In the Fortran, IDL, and MATLAB SPICE
//!    Toolkits, DLA descriptors are implemented as arrays. In the CSPICE
//!    Toolkit, DLA descriptors are implemented by the structure SpiceDLADescr.
//!
//!  The DLA segment descriptor members are:
//!
//!  
//!
//! ```text
//!    +---------------+
//!    | BACKWARD PTR  | Linked list backward pointer
//!    +---------------+
//!    | FORWARD PTR   | Linked list forward pointer
//!    +---------------+
//!    | BASE INT ADDR | Base DAS integer address
//!    +---------------+
//!    | INT COMP SIZE | Size of integer segment component
//!    +---------------+
//!    | BASE DP ADDR  | Base DAS d.p. address
//!    +---------------+
//!    | DP COMP SIZE  | Size of d.p. segment component
//!    +---------------+
//!    | BASE CHR ADDR | Base DAS character address
//!    +---------------+
//!    | CHR COMP SIZE | Size of character segment component
//!    +---------------+
//!  
//! ```
//!
//!  The "base address" of a segment component of a given data type is the
//!    address, in the DAS address space of that type, preceding the first
//!    element of that component. All DAS addresses are 1-based.
//!
//!  The general form of the doubly linked list layout in DAS integer address
//!    space is:
//!
//!  
//!
//! ```text
//!  
//!                  +------------------------------+
//!                  |  Pointer to first descriptor | ----+
//!                  +------------------------------+     |
//!    +-------------|  Pointer to last descriptor  |     |
//!    |             +------------------------------+     |
//!    |                                                  |
//!    |             +------------------------------+     |
//!    |  NULL <---- |  Backward pointer            | <---+ First node
//!    |       +---> |                              |
//!    |       |     +------------------------------+
//!    |       |     |  Forward pointer             | ----+
//!    |       |     +------------------------------+     |
//!    |       |     |  Rest of DLA Descriptor 1    |     |
//!    |       |     +------------------------------+     |
//!    |       |     |  Segment 1 integer component |     |
//!    |       |     +------------------------------+     |
//!    |       +---- |  Backward pointer            | <---+ Second node
//!    |       +---> |                              |
//!    |       |     +------------------------------+
//!    |       |     |  Forward pointer             | ----+
//!    |       |     +------------------------------+     |
//!    |       |     |  Rest of DLA Descriptor 2    |     |
//!    |       |     +------------------------------+     |
//!    |       |     |  Segment 2 integer component |     |
//!    |       |     +------------------------------+     |
//!    |  pointer from third node                     <---+ Third node
//!    |                            .
//!    |                            .
//!    |                            .
//!    |  pointer to node N-1                  pointer from node N-1
//!    |       |                                          |
//!    |       |     +------------------------------+     |
//!    +--->   +---- |  Backward pointer            | <---+ Final (Nth) node
//!                  +------------------------------+
//!                  |  Forward pointer             | ----> NULL
//!                  +------------------------------+
//!                  |  Rest of DLA Descriptor N    |
//!                  +------------------------------+
//!                  |  Segment N integer component |
//!                  +------------------------------+
//!  
//! ```
//!
//!  While this diagram shows the presence of multiple segments, a DLA file
//!    need not contain more than one segment. It is valid, but usually not
//!    useful, for a DLA file to have no segments.
//!
//!  
//!
//!
//!  
//! ##  DLA Segment Layout
//!
//!  The data populating a DLA file are stored in the file's segments.
//!
//!  The segments of a DLA file consist of components in the respective DAS
//!    character, double precision, and integer address spaces. The components
//!    occupy contiguous address ranges in the respective address spaces.
//!
//!  The general form of the DLA segment layout is:
//!
//!  
//!
//! ```text
//!  
//!    Character space   Double precision space   Integer space
//!  
//!    +-------------+   +--------------------+
//!    |Segment 1    |   |Segment 1           |  +------------+
//!    |character    |   |double precision    |  |Segment 1   |
//!    |component    |   |component           |  |integer     |
//!    +-------------+   |                    |  |component   |
//!    |Segment 2    |   |                    |  |            |
//!    |character    |   +--------------------+  |            |
//!    |component    |   |Segment 2           |  +------------+
//!    |             |   |double precision    |
//!    +-------------+   |component           |  +------------+
//!                      +--------------------+  |Segment 2   |
//!                                              |integer     |
//!                                              |component   |
//!                                              +------------+
//!         ...                   ...                 ...
//!  
//! ```
//!
//!  In the diagram above, the first integer segment component is displaced
//!    downward slightly to indicate that the first component starts at an
//!    integer address greater than 1: that component is preceded by other
//!    information. DLA descriptors lie between consecutively numbered integer
//!    components, so those components are not contiguous.
//!
//!  Any of the segment components may be empty, but the DAS integer address
//!    space is never empty.
//!
//!  In a segregated DLA file, all segment components of a given type are
//!    stored sequentially in the DAS address space of that type.
//!
//!  
//!
//!
//!  
//! #  DLA Routines
//!
//!  Access to DLA files is provided by DLA routines and routines of the
//!    SPICE DAS subsystem.
//!
//!  In all languages supported by SPICE, the DLA subsystem provides routines
//!    for traversing DLA segment lists. Although the term is somewhat
//!    inaccurate, this is usually referred to as "searching" the lists.
//!
//!  Routines of the DAS subsystem enable applications to open a DLA file for
//!    read access, close a DLA file, and extract comments from a DLA file's
//!    comment area.
//!
//!  In the Fortran and C SPICE Toolkits, DAS routines can open an existing
//!    DLA file for write access and delete or write to a DLA file's comment
//!    area.
//!
//!  In the Fortran SPICE Toolkit, DLA routines are provided to open a new
//!    DLA file and to start and finish new DLA segments. DAS routines are
//!    provided to write data to DLA segments, read data from DLA segments, and
//!    update data in existing DLA segments.
//!
//!  
//!
//!
//!  
//! ###  Summary of DLA Mnemonics
//!
//!  All of the subroutines in the DLA family have names beginning with the
//!    letters "DLA," followed by a two- or three-character mnemonic. For
//!    example, the routine that starts a forward search through the segment
//!    descriptor list in a DLA file is named [DLABFS](crate::raw::dlabfs), pronounced "DLA-B-F-S."
//!    The following is a complete list of DLA mnemonics and translations, in
//!    alphabetical order.
//!
//!  
//!
//! ```text
//!    BBS  Begin backward search
//!    BFS  Begin forward search
//!    BNS  Begin new segment
//!    ENS  End new segment
//!    FNS  Find next segment
//!    FPS  Find previous segment
//!    OPN  Open new DLA file
//!    SSG  Test for same segment
//! ```
//!
//!     
//! ###  Summary of DAS Mnemonics
//!
//!  The DAS routines whose mnemonics are listed below serve as part of the
//!    DLA system's interface.
//!
//!  
//!
//! ```text
//!    AC   Add comments from buffer to file
//!    ADC  Add character data to file
//!    ADD  Add double precision data to file
//!    ADI  Add integer data to file
//!    CLS  Close file
//!    DC   Delete comments from file
//!    EC   Extract comments from file
//!    OPR  Open file for read access
//!    OPW  Open file for write access
//!    RDC  Read character data from file
//!    RDD  Read double precision data from file
//!    RDI  Read integer data from file
//!    UDC  Update character data in file
//!    UDD  Update double precision data in file
//!    UDI  Update integer data in file
//! ```
//!
//!     
//! ###  Summary of Calling Sequences
//!
//!  Calling sequences of the DLA family of routines are given below.
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
//!    DASCLS ( HANDLE )
//!    DASOPR ( FNAME, HANDLE )
//!    DLAOPN ( FNAME, FTYPE, IFNAME, NRESV, HANDLE )
//!    DASOPW ( FNAME, HANDLE )
//! ```
//!
//!  Beginning and ending new segments:
//!
//!  
//!
//! ```text
//!    DLABNS ( HANDLE )
//!    DLAENS ( HANDLE )
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
//!  Searching for segments:
//!
//!  
//!
//! ```text
//!    DLABBS ( HANDLE, DESCR, FOUND )
//!    DLAFNS ( HANDLE, DESCR, NXTDSC, FOUND )
//!    DLAFPS ( HANDLE, DESCR, PRVDSC, FOUND )
//!    DLABFS ( HANDLE, DESCR, FOUND )
//! ```
//!
//!  Accessing the comment area:
//!
//!  
//!
//! ```text
//!    DASAC ( HANDLE, N, BUFFER )
//!    DASDC ( HANDLE )
//!    DASEC ( HANDLE, BUFSIZ, N, BUFFER, DONE )
//! ```
//!
//!  Comparing DLA segment descriptors
//!
//!  
//!
//! ```text
//!    DLASSG
//! ```
//!
//!     
//! #  Appendix A: Revision History
//!
//!  
//!
//!
//!  
//! ###  2017 APR 03 by N. J. Bachman
//!
//!  Initial release.
//!
//!  
//!
//!
//!     
