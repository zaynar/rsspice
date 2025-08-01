//! #  DSK Required Reading
//!
//!  Last revised on 2021 OCT 22 by N. J. Bachman.
//!
//!  
//!
//!
//!  
//! ##  Abstract
//!
//!  The Digital Shape Kernel (DSK) subsystem is the component of SPICE
//!    concerned with detailed shape models for extended objects.
//!
//!  
//!
//!
//!  
//! ###  Purpose
//!
//!  This document is a reference guide for the SPICE DSK subsystem. Here
//!    you'll find
//!
//!  
//!
//! * A discussion of the DSK subsystem's software. This includes the API
//! (application programming interface) routines---these are the routines that
//! may be called directly by SPICE-based user application code---and the SPICE
//! utility programs that work with DSK files.
//!
//!  * Discussions of DSK concepts
//!
//!  * A description of the DSK file format
//!
//!  * Discussion of problems that may arise when using the DSK subsystem
//!
//!  The DSK subsystem does not deal with triaxial ellipsoid shape models;
//!    these models are usually provided by PCK files. See the PCK Required
//!    Reading, [pck.req](crate::required_reading::pck), for further information.
//!
//!  
//!
//!
//!  
//! ###  Intended Audience
//!
//!  This document addresses the needs of several groups of SPICE users.
//!    Users looking for a basic discussion of the capabilities of the SPICE
//!    DSK subsystem should read the introduction below. Users planning to
//!    write application code using the DSK subsystem may benefit from reading
//!    the entire document, possibly excepting the description of the details
//!    of DSK type 2, but in any case should read the "DSK Concepts" chapter.
//!    Users planning to create DSK files are encouraged to read the entire
//!    document.
//!
//!  This document assumes you already have a strong understanding of SPICE
//!    concepts and terminology.
//!
//!  
//!
//!
//!  
//! ###  References
//!
//!  The references listed below provide essential background for programmers
//!    intending to use the DSK subsystem. They also document DSK utility
//!    programs.
//!
//!  
//!
//! *  1. DAS Required Reading ([das.req](crate::required_reading::das))
//!
//!  *  2. DLA Required Reading ([dla.req](crate::required_reading::dla))
//!
//!  *  3. NAIF Integer ID Codes ([naif_ids.req](crate::required_reading::naif_ids))
//!
//!  *  4. Frames Required Reading ([frames.req](crate::required_reading::frames))
//!
//!  *  5. Convert User's Guide ([convert.ug](crate::raw::convert.ug))
//!
//!  *  6. DSKBRIEF User's Guide
//!
//!  *  7. MKDSK User's Guide
//!
//!  *  8. DSKEXP User's Guide
//!
//!  *  9. DLACAT User's Guide.
//!
//!  *  10. BINGO User's Guide.
//!
//!  *  11. SPICE Tutorials. Thes are available on the NAIF web site at
//!
//!  ```text
//!             https://naif.jpl.nasa.gov/naif/tutorials.html
//! ```
//!  The programs
//!
//!  
//!
//! ```text
//!    DLACAT
//!    BINGO
//! ```
//!
//!  are utilities that are not part of the SPICE Toolkit, but that operate
//!    on DSK files. They are available on the NAIF web site at
//!
//!  
//!
//! ```text
//!    https://naif.jpl.nasa.gov/naif/utilities.html
//! ```
//!
//!     
//! ##  Introduction
//!
//!  The Digital Shape Kernel (DSK) subsystem is the component of SPICE
//!    concerned with detailed shape models for extended objects. These objects
//!    typically are solar system bodies such as planets, dwarf planets,
//!    natural satellites, asteroids, and comet nuclei. DSK data can represent
//!    the shapes of such objects, as well as local topography, such as that in
//!    the vicinity of a rover or a tracking station.
//!
//!  DSK data also can represent shapes of artificial objects such as
//!    spacecraft components, or represent abstractions such as the subset of a
//!    target body's surface that has a property of interest.
//!
//!  The DSK subsystem comprises software, the DSK file format specification,
//!    and documentation.
//!
//!  The primary purpose of the DSK subsystem is to enable SPICE-based
//!    applications to conveniently and efficiently use detailed shape data in
//!    geometry computations performed by SPICE routines. DSK data enable these
//!    routines to produce more accurate results than those obtainable using
//!    triaxial ellipsoid shape models. See the section below titled
//!    "High-Level DSK-Enabled Geometry Routines" for details.
//!
//!  The DSK implementation ensures that shape data used by SPICE are
//!    accompanied by all of the attribute information necessary for correct,
//!    programmatic use of the data---including, but not limited to, reference
//!    frames, central bodies, coordinate systems, spatial coverage bounds, and
//!    time bounds of applicability. The DSK format enables data to be
//!    augmented by indexes, or other pre-computed parameters, that greatly
//!    enhance the speed of common geometric computations, such as those of
//!    ray-surface intercepts.
//!
//!  DSK data sets can be distributed across multiple DSK files; this is
//!    normal for large data sets. Such sets of files can be "loaded" (made
//!    available for read access by SPICE software) concurrently; SPICE
//!    software will select data from the appropriate files as needed to
//!    perform computations.
//!
//!  The DSK file format facilitates storage, transmission, and archival of
//!    shape data. It allows the data to be annotated with free-form
//!    descriptive comments, also called "metadata."
//!
//!  DSK documentation uses the term "data type" to refer to types of
//!    mathematical shape representations, associated DSK file formats, and
//!    software. The DSK subsystem is designed to accommodate multiple data
//!    types, and to enable high-level SPICE geometry software to function
//!    independently of these types.
//!
//!  Currently there is only one DSK data type, which represents the shape of
//!    an object by a set of triangular plates. This representation is called a
//!    "tessellated plate model" or "triangular plate model." The DSK
//!    documentation refers to this type as "DSK type 2."
//!
//!  Support for digital elevation models (DEMs) will be added in a later
//!    version of the SPICE Toolkit; one or more new DSK data types will be
//!    developed to support such data.
//!
//!  Because many popular file formats for shape data exist, and because it
//!    is impractical for these formats to be used directly by SPICE for
//!    geometric computations, the DSK subsystem supports conversion of a
//!    variety of text-based shape data file formats to DSK format; it also
//!    supports conversion of DSK files to a variety of text formats.
//!
//!  
//!
//!
//!  
//! #  DSK Software
//!
//!  Below we list the SPICE routines that either work directly with DSK
//!    files or that support DSK usage. We also briefly describe the SPICE
//!    utility programs that work with DSK files.
//!
//!  
//!
//!
//!  
//! ##  High-Level DSK-Enabled Geometry Routines
//!
//!  High-level SPICE geometry routines constitute the principal application
//!    programming interface to the DSK subsystem. The routines described here
//!    are supported in all language versions of the SPICE Toolkit.
//!
//!  All of these routines perform computations that involve a target body.
//!    All can model the shape of that body using data provided by DSK files.
//!    Many of the routines can use triaxial ellipsoid models as well.
//!
//!  These routines have extensive header documentation. Each header
//!    describes all input and output arguments and includes one or more
//!    example programs accompanied by example meta-kernels and corresponding
//!    program outputs.
//!
//!  
//!
//! *  [DSKXSI](crate::raw::dskxsi)
//!
//!
//!  Ray-surface intercept with source information: compute a ray-surface
//! intercept using data provided by multiple loaded DSK segments. Return
//! information about the source of the data defining the surface on which
//! the intercept was found: DSK handle, DLA and DSK descriptors, and DSK
//! data type-dependent parameters. \[Compare with [SINCPT](crate::raw::sincpt).]
//!
//!  *  [DSKXV](crate::raw::dskxv)
//!
//!
//!  Vectorized ray-surface intercept: compute ray-surface intercepts for a
//! set of rays, using data provided by multiple loaded DSK segments.
//! \[Compare with [SINCPT](crate::raw::sincpt).]
//!
//!  *  [GFOCLT](crate::raw::gfoclt)
//!
//!
//!  Occultation search: determine time intervals when an observer sees one
//! target occulted by, or in transit across, another.
//!
//!  *  [ILLUMF](crate::raw::illumf)
//!
//!
//!  Illumination angles: compute the illumination angles---phase,
//! incidence, and emission---at a specified point on a target body. Return
//! logical flags indicating whether the surface point is visible from the
//! observer's position and whether the surface point is illuminated.
//!
//!  *  [LATSRF](crate::raw::latsrf)
//!
//!
//!  Compute surface points specified by longitudes and latitudes: map an
//! array of planetocentric longitude/latitude coordinate pairs to surface
//! points on a specified target body.
//!
//!  *  [LIMBPT](crate::raw::limbpt)
//!
//!
//!  Find limb points on a target body: the limb is the set of points of
//! tangency on the target of rays emanating from the observer. The caller
//! specifies half-planes bounded by the observer-target center vector in
//! which to search for limb points.
//!
//!  *  [OCCULT](crate::raw::occult)
//!
//!
//!  Occultation state: determine the occultation condition (not occulted,
//! partially, etc.) of one target relative to another target as seen by an
//! observer at a given time.
//!
//!  *  [SINCPT](crate::raw::sincpt)
//!
//!
//!  Ray-surface intercept: for a given observer, target, and ray direction,
//! find the nearest intersection to the observer of the ray and target
//! body's surface, optionally corrected for light time and stellar
//! aberration.
//!
//!  *  [SRFNRM](crate::raw::srfnrm)
//!
//!
//!  Surface normal vectors: map an array of surface points on a specified
//! target body to the corresponding unit length outward surface normal
//! vectors.
//!
//!  *  [SUBPNT](crate::raw::subpnt)
//!
//!
//!  Sub-observer point: compute the rectangular coordinates of the
//! sub-observer point on a target body at a specified epoch, optionally
//! corrected for light time and stellar aberration.
//!
//!  *  [SUBSLR](crate::raw::subslr)
//!
//!
//!  Sub-solar point: compute the rectangular coordinates of the sub-solar
//! point on a target body at a specified epoch, optionally corrected for
//! light time and stellar aberration.
//!
//!  *  [TERMPT](crate::raw::termpt)
//!
//!
//!  Find terminator points on a target body: the terminator is the set of
//! points of tangency on the target body of planes tangent to both this
//! body and to a light source. The caller specifies half-planes, bounded
//! by the illumination source center-target center vector, in which to
//! search for terminator points.
//!
//!     
//! ##  DSK File Loading and Unloading
//!
//!  DSK files are loaded and unloaded by the same routines used for all
//!    other SPICE kernels:
//!
//!  
//!
//! *  [FURNSH](crate::raw::furnsh)
//!
//!
//!  Load a SPICE kernel. DSK files can be loaded directly by [FURNSH](crate::raw::furnsh); they
//! also can be referenced in a meta-kernel which can be loaded by [FURNSH](crate::raw::furnsh).
//! The latter method is usually preferable; see the "Intro to Kernels"
//! tutorial on the NAIF web site.
//!
//!  *  [UNLOAD](crate::raw::unload)
//!
//!
//!  Unload a SPICE kernel. [UNLOAD](crate::raw::unload) can unload both DSK files and
//! meta-kernels.
//!
//!  *  [KCLEAR](crate::raw::kclear)
//!
//!
//!  Unload all SPICE kernels and clear the kernel pool.
//!
//!     
//! ##  DSK Utility Routines
//!
//!  DSK utility routines perform functions other than geometry computations.
//!
//!  Routines for DSK file creation:
//!
//!  
//!
//! *  [DSKOPN](crate::raw::dskopn)
//!
//!
//!  Open a new DSK file for subsequent write operations.
//!
//!  *  [DSKCLS](crate::raw::dskcls)
//!
//!
//!  Close a DSK file.
//!
//!  Also see the data type-specific segment writing routines below.
//!
//!  Routine for fetching DSK segment attributes:
//!
//!  
//!
//! *  [DSKGD](crate::raw::dskgd)
//!
//!
//!  Return the DSK descriptor from a DSK segment identified by a DAS handle
//! and DLA descriptor.
//!
//!  Routines for determining objects covered by DSK files:
//!
//!  
//!
//! *  [DSKOBJ](crate::raw::dskobj)
//!
//!
//!  Find the set of body ID codes of all objects for which data are
//! provided in a specified DSK file.
//!
//!  *  [DSKSRF](crate::raw::dsksrf)
//!
//!
//!  Find the set of surface ID codes for all surfaces associated with a
//! given body in a specified DSK file.
//!
//!  Routines for fetching and adjusting DSK tolerances:
//!
//!  
//!
//! *  [DSKGTL](crate::raw::dskgtl)
//!
//!
//!  Retrieve the value of a specified DSK tolerance or margin parameter.
//!
//!  *  [DSKSTL](crate::raw::dskstl)
//!
//!
//!  Set the value of a specified DSK tolerance or margin parameter.
//!
//!     
//! ##  DSK Type 2 Routines
//!
//!  The DSK routines specific to data type 2 are:
//!
//!  
//!
//! *  [DSKB02](crate::raw::dskb02)
//!
//!
//!  Return bookkeeping data from a DSK type 2 segment.
//!
//!  *  [DSKD02](crate::raw::dskd02)
//!
//!
//!  Fetch double precision data from a DSK type 2 segment. To fetch vertex
//! data, see [DSKV02](crate::raw::dskv02).
//!
//!  *  [DSKI02](crate::raw::dski02)
//!
//!
//!  Fetch integer data from a DSK type 2 segment. To fetch plate data, see
//! [DSKP02](crate::raw::dskp02). To fetch vertex and plate counts, see [DSKZ02](crate::raw::dskz02).
//!
//!  *  [DSKMI2](crate::raw::dskmi2)
//!
//!
//!  Make a spatial index for a DSK type 2 segment.
//!
//!  *  [DSKN02](crate::raw::dskn02)
//!
//!
//!  Compute the outward unit normal vector for a specified plate in a DSK
//! type 2 segment.
//!
//!  *  [DSKP02](crate::raw::dskp02)
//!
//!
//!  Fetch triangular plates from a DSK type 2 segment.
//!
//!  *  [DSKRB2](crate::raw::dskrb2)
//!
//!
//!  Determine range bounds for a set of triangular plates to be stored in a
//! DSK type 2 segment.
//!
//!  *  [DSKV02](crate::raw::dskv02)
//!
//!
//!  Fetch vertices from a DSK type 2 segment.
//!
//!  *  [DSKW02](crate::raw::dskw02)
//!
//!
//!  Write a DSK type 2 segment to a DSK file.
//!
//!  *  [DSKZ02](crate::raw::dskz02)
//!
//!
//!  Return plate count and vertex count of a DSK type 2 segment.
//!
//!     
//! ##  DLA and DAS Routines
//!
//!  DSK files are instances of DLA files, which in turn are instances of DAS
//!    files. See the chapter "DSK Files" for further information.
//!
//!  Some lower-level functionality is provided by routines of the DLA and
//!    DAS subsystems. The routines below support linear traversal of the
//!    doubly linked list---also called "searching" the list---of DSK
//!    segments within a DSK file:
//!
//!  
//!
//! *  DASOPR
//!
//!
//!  Open a DAS file for reading.
//!
//!  *  [DASCLS](crate::raw::dascls)
//!
//!
//!  Close a DAS file.
//!
//!  *  [DLABFS](crate::raw::dlabfs)
//!
//!
//!  Begin a forward segment search in a DLA file.
//!
//!  *  [DLAFNS](crate::raw::dlafns)
//!
//!
//!  Find the segment following a specified segment in a DLA file.
//!
//!  *  [DLABBS](crate::raw::dlabbs)
//!
//!
//!  Begin a backward segment search in a DLA file.
//!
//!  *  DASFPS
//!
//!
//!  Find the segment preceding a specified segment in a DLA file.
//!
//!     
//! ##  Surface Name and ID Conversion Routines
//!
//!  The routines below map surface names to surface IDs and vice versa.
//!
//!  
//!
//! *  [SRFC2S](crate::raw::srfc2s)
//!
//!
//!  Translate a surface ID code, together with a body ID code, to the
//! corresponding surface name.
//!
//!  *  [SRFCSS](crate::raw::srfcss)
//!
//!
//!  Translate a surface ID code, together with a body string, to the
//! corresponding surface name.
//!
//!  *  [SRFS2C](crate::raw::srfs2c)
//!
//!
//!  Translate a surface string, together with a body string, to the
//! corresponding surface ID code.
//!
//!  *  [SRFSCC](crate::raw::srfscc)
//!
//!
//!  Translate a surface string, together with a body ID code, to the
//! corresponding surface ID code.
//!
//!     
//! ##  SPICE Toolkit DSK Utility Programs
//!
//!  The DSK utility programs included in the SPICE Toolkit are
//!
//!  
//!
//! *  DSKBRIEF
//!
//!
//!  Display summaries of one or more DSK files. See the DSKBRIEF User's
//! Guide, [dskbrief.ug](crate::raw::dskbrief.ug).
//!
//!  *  MKDSK
//!
//!
//!  Create a DSK file from shape data provided in a text file. See the
//! MKDSK User's Guide, [mkdsk.ug](crate::raw::mkdsk.ug).
//!
//!  *  DSKEXP
//!
//!
//!  "Export" (write) DSK data to one or more text files. See the DSKEXP
//! User's Guide, [dskexp.ug](crate::raw::dskexp.ug).
//!
//!  *  COMMNT
//!
//!
//!  Read, extract, append to, or delete the contents of a DSK file's
//! comment area. See the COMMNT User's Guide, [commnt.ug](crate::raw::commnt.ug).
//!
//!  *  TOBIN, TOXFR
//!
//!
//!  Convert a transfer format DSK file to binary format, and vice versa.
//! See the user's guide "Converting and Porting SPICE Binary Data
//! Files," [convert.ug](crate::raw::convert.ug).
//!
//!     
//! ##  Non-SPICE Toolkit DSK Utility Programs
//!
//!  The utility programs below are not part of the SPICE Toolkit, but are
//!    available, as is their documentation, from the NAIF server.
//!
//!  
//!
//! *  BINGO
//!
//!
//!  Convert a binary DSK file from IEEE little-endian format to IEEE
//! big-endian format, or vice versa.
//!
//!  *  DLACAT
//!
//!
//!  Concatenate DLA files (DSK files are instances of DLA files) into a
//! single file.
//!
//!     
//! #  DSK Concepts
//!
//!  
//!
//!
//!  
//! ##  Shapes and Surfaces
//!
//!  The terms "shape data," "surface data," and "topography" are
//!    generally used as synonyms in DSK documentation. The term "surface,"
//!    when applied to DSK data, refers solely to the geometric form of the
//!    surface---never to associated properties such as albedo or chemical
//!    composition.
//!
//!  The term "surfaces" is also used to refer to DSK data sets themselves,
//!    particularly when there are multiple data sets, differing in some
//!    aspects, that provide data for a given body. For example, Mars
//!    topography data sets based on MGS MOLA data might be referred to as the
//!    "64 pixels/degree surface" or the "128 pixels/degree surface."
//!
//!  
//!
//!
//!  
//! ##  Surface IDs
//!
//!  To facilitate run-time selection of DSK data, SPICE provides DSK
//!    segments with a second identifying attribute in addition to the
//!    "central body": the "surface ID code" (or "surface ID"). Different
//!    DSK data sets for a given body may be assigned distinct surface IDs.
//!
//!  At run time, calls from user applications to SPICE routines can restrict
//!    the DSK data used to those from specified surfaces. For example, a user
//!    application might direct the SPICE sub-observer point routine [SUBPNT](crate::raw::subpnt) to
//!    to use a high-resolution surface for a spacecraft altitude computation,
//!    versus a low-resolution surface for plotting the spacecraft's ground
//!    track.
//!
//!  Because surface IDs enable SPICE applications to select data from among
//!    those available in loaded DSK files, it is not necessary for
//!    applications to repeatedly load and unload DSK files to control which
//!    shape data are used for a given body and computation. Applications
//!    normally can load at start-up all of the DSK data for a given body, then
//!    select the data to be used on a per-computation basis.
//!
//!  Avoiding repetitive DSK loading tends to improve an application's
//!    computation speed. This is because after any change to the set of loaded
//!    DSK files, the DSK subsystem must perform some bookkeeping computations
//!    before DSK-based computations can be performed. Degradation of overall
//!    execution speed due to these computations is slight as long as they're
//!    performed infrequently.
//!
//!  SPICE surface IDs are associated with surface names; these associations
//!    are made via assignments in SPICE text kernels. Surface name-ID
//!    associations are made for specific bodies: the combination of a body
//!    name or body ID and surface name can be mapped to a surface ID code, and
//!    the combination of a body name or body ID and a surface ID code can be
//!    mapped to a surface name.
//!
//!  A given surface ID code can be re-used for different bodies without
//!    ambiguity. For a given body, it's important for users to coordinate
//!    assignment of surface names and surface IDs.
//!
//!  For a given body, the surface name to surface ID mapping may be
//!    many-to-one. If multiple surface names are associated with a surface ID
//!    for a specified body, then the surface ID-to-name conversion routines
//!    will map that surface ID to the last name associated with the ID. The
//!    surface name-to-ID mapping routines will map any of the surface names to
//!    that surface ID.
//!
//!  The SPICE routines for converting between surface IDs and surface names
//!    are listed in the section "Surface Name and ID Conversion Routines"
//!    above.
//!
//!  
//!
//!
//!  
//! ###  Defining Surface Name-ID Mappings
//!
//!  Surface name-to-ID mappings may be defined at run time by loading text
//!    kernels containing kernel variable assignments of the form
//!
//!  
//!
//! ```text
//!    NAIF_SURFACE_NAME += ( <surface name 1>, ... )
//!    NAIF_SURFACE_CODE += ( <surface code 1>, ... )
//!    NAIF_SURFACE_BODY += ( <body code 1>,    ... )
//! ```
//!
//!  Above, the Ith elements of the lists on the assignments' right hand
//!    sides together define the Ith surface name/ID mapping.
//!
//!  The same effect can be achieved using assignments formatted as follows:
//!
//!  
//!
//! ```text
//!    NAIF_SURFACE_NAME += <surface name 1>
//!    NAIF_SURFACE_CODE += <surface code 1>
//!    NAIF_SURFACE_BODY += <body code 1>
//!  
//!    NAIF_SURFACE_NAME += <surface name 2>
//!    NAIF_SURFACE_CODE += <surface code 2>
//!    NAIF_SURFACE_BODY += <body code 2>
//!  
//!       ...
//! ```
//!
//!  Note the use of the
//!
//!  
//!
//! ```text
//!    +=
//! ```
//!
//!  operator; this operator appends to rather than overwrites the kernel
//!    variable named on the left hand side of the assignment.
//!
//!  Multiple surface names, also called "surface aliases," can be mapped
//!    to a surface ID for a given body as follows:
//!
//!  
//!
//! ```text
//!    NAIF_SURFACE_NAME += <surface name 1>
//!    NAIF_SURFACE_CODE += <surface code 1>
//!    NAIF_SURFACE_BODY += <body code 1>
//!  
//!    NAIF_SURFACE_NAME += <surface name 2>
//!    NAIF_SURFACE_CODE += <surface code 1>
//!    NAIF_SURFACE_BODY += <body code 1>
//! ```
//!
//!  Using this mapping, body code 1 and either surface name will be mapped
//!    by the surface name-to-ID conversion routines to surface code 1. Body
//!    code 1 and surface code 1 will be mapped by the surface ID-to-name
//!    routines to surface name 2.
//!
//!  
//!
//!
//!  
//! ##  Segments
//!
//!  Data in every DSK file are grouped into one or more subsets called
//!    "segments." DSK segments are implemented as DLA segments, also called
//!    "arrays." Each DSK segment has double precision and integer
//!    components.
//!
//!  Each DSK segment contains data for some or all of the surface of a
//!    single body object. This object is called the "body," "central
//!    body," or "center," even though it need not be a natural solar system
//!    body.
//!
//!  Within a segment, the data have the following attributes in common:
//!
//!  
//!
//! * Body
//!
//!  * Surface
//!
//!  * Reference frame
//!
//!  * Coordinate system
//!
//!  * Coordinate system parameters, if applicable (for example, planetodetic
//! equatorial radius and flattening coefficient)
//!
//!  * Spatial coverage bounds
//!
//!  * Time bounds
//!
//!  * Data type
//!
//!  * Data class
//!
//!  These attributes are discussed in the following sections.
//!
//!  When DSK files are loaded via one or more calls to [FURNSH](crate::raw::furnsh), all segments
//!    from those files become available to the DSK subsystem for use in
//!    computations. At run time, when a request for shape data for a specified
//!    body is made to the DSK subsystem, all segments for that body can be
//!    considered as possible sources of data to satisfy the request.
//!
//!  For small DSK data sets, such as low-resolution shape models for
//!    asteroids, a single segment can suffice to store all of the data for the
//!    model. Large DSK data sets typically consist of tens or hundreds of
//!    segments distributed over multiple DSK files. Normally all segments for
//!    a given body can be loaded at one time. The limit on the total number of
//!    DSK segments for all bodies that can be loaded is given in Appendix B.
//!
//!  DSK utilities typically create or operate on segments:
//!
//!  
//!
//! * MKDSK creates a single-segment DSK file
//!
//!  * DSKBRIEF displays summary information for groups of segments having common
//! attributes, or for individual segments
//!
//!  * DSKEXP exports data from DSK segments to separate output files
//!
//!     
//! ###  DLA and DSK Descriptors
//!
//!  Lower-level SPICE DSK routines refer to DSK segments by means of data
//!    structures called DLA and DSK "descriptors."
//!
//!  The DLA descriptor of a DSK segment indicates the location of the
//!    segment's data in the DSK file containing that segment. DLA descriptors
//!    contain DAS base addresses and sizes of the double precision and integer
//!    components of the associated segments. DLA descriptors are integer
//!    arrays.
//!
//!  User applications can locate all segments in a DSK file by calling the
//!    DLA "begin forward search" routine [DLABFS](crate::raw::dlabfs), then repeatedly calling the
//!    DLA "find next segment" routine [DLAFNS](crate::raw::dlafns). See the API documentation of
//!    those routines for details and code examples.
//!
//!  The DSK descriptor of a DSK segment contains the segment's attribute
//!    information; these are the attributes listed above. User applications
//!    can determine attributes of a DSK segment by obtaining the DLA
//!    descriptor of the segment, then calling the SPICE routine [DSKGD](crate::raw::dskgd) to
//!    obtain the segment's DSK descriptor.
//!
//!  See the chapter "DSK Files" for details.
//!
//!  
//!
//!
//!  
//! ###  Reference Frames
//!
//!  Each DSK segment has an associated reference frame. That frame is fixed
//!    to the central body. The center of the frame need not be the central
//!    body. For example, a segment containing data for Mars could use a
//!    topocentric reference frame centered at a specified surface point on
//!    Mars.
//!
//!  Within a DSK segment, all shape data are expressed relative to that
//!    segment's reference frame.
//!
//!  For example, if a segment containing data for Phobos uses the IAU_PHOBOS
//!    body-fixed frame, and if the segment contains vertices of triangular
//!    plates, the coordinates of those vertices are expressed in the
//!    IAU_PHOBOS reference frame.
//!
//!  The set of DSK segments to be used in a computation for a given body
//!    need not be associated with a single reference frame, but using data
//!    from mixed frames should be done cautiously. It is up to the user to
//!    combine data in ways that make sense.
//!
//!  For example, Mars data expressed in the Mars-centered IAU_MARS frame can
//!    be used together with Mars data expressed in one or more Mars
//!    topocentric frames.
//!
//!  On the other hand, it doesn't make sense to combine earth data expressed
//!    in the ITRF93 frame with data expressed in the IAU_EARTH frame, because
//!    those frames have some relative rotation. The same is true for data
//!    expressed in the IAU_MOON and MOON_ME frames: even though these are
//!    realizations of the same reference frame, the approximation error in the
//!    IAU_MOON frame is time-dependent, so these frames have relative
//!    rotation.
//!
//!  
//!
//!
//!  
//! ###  Coordinate Systems and Spatial Coverage
//!
//!  For the purpose of segment selection, the spatial coverage of a DSK
//!    segment is considered to be three-dimensional: it is a region of space
//!    within which the segment provides data for the associated body.
//!
//!  The supported coordinate systems are:
//!
//!  
//!
//! * Latitudinal (planetocentric)
//!
//!  * Planetodetic
//!
//!  * Rectangular
//!
//!  Planetocentric coordinates are appropriate for most natural bodies.
//!
//!  Planetodetic coordinates should be used only for large bodies having
//!    surfaces well approximated by spheroids.
//!
//!  Rectangular coordinates may be appropriate for data sets expressed in
//!    topocentric reference frames, for artificial structures, and for
//!    extremely irregular natural bodies.
//!
//!  A segment's coordinate system is used to represent the segment's
//!    coverage bounds.
//!
//!  For example, a DSK segment that uses the Phobos planetocentric
//!    coordinate system and IAU_PHOBOS reference frame might contain surface
//!    data for Phobos within the spatial region
//!
//!  
//!
//! ```text
//!    Planetocentric latitude:    -90 to  +90 degrees
//!    Planetocentric longitude:  -180 to +180 degrees
//!    Radius:                       0 to   10 km
//! ```
//!
//!  Here the planetocentric coordinate system's equatorial plane is the X-Y
//!    plane of the IAU_PHOBOS frame. The prime meridian of the coordinate
//!    system lies in the frame's X-Z plane and intersects the +X axis of the
//!    frame.
//!
//!  Another example: a DSK segment that uses a Mars planetodetic coordinate
//!    system might contain surface data for Mars within the spatial region
//!
//!  
//!
//! ```text
//!    Planetodetic latitude:   -30 to +30 degrees
//!    Planetodetic longitude:  +60 to +90 degrees
//!    Altitude:                -10 to +20 km
//! ```
//!
//!  Here the planetodetic coordinate system's equatorial plane is the X-Y
//!    plane of the segment's reference frame, for example the IAU_MARS frame.
//!    The prime meridian of the coordinate system lies in the frame's X-Z
//!    plane and intersects the +X axis of the frame. The altitude is measured
//!    relative to a reference spheroid, the size and shape parameters of which
//!    are contained in the segment.
//!
//!  A third example: a DSK segment that contains data for a horizon mask for
//!    a tracking station might use rectangular coordinates and a topocentric
//!    frame having axis directions
//!
//!  
//!
//! ```text
//!    X: north
//!    Y: west
//!    Z: up
//! ```
//!
//!  The region covered by the segment might be
//!
//!  
//!
//! ```text
//!    X:   -0.5 to +0.5 km
//!    Y:   -0.5 to +0.5 km
//!    Z:   -0.2 to +0.2 km
//! ```
//!
//!  In this case the horizon mask need not model the topography surrounding
//!    the station; it can simply model obscuration due to the topography. So
//!    the coverage region need not extend to the horizon; it can be contained
//!    in a small box enclosing the station.
//!
//!  
//!
//!
//!  
//! ###  Spatial coverage: Dimensions
//!
//!  The coverage bounds of a segment enable the DSK subsystem to rapidly
//!    determine whether the segment may be applicable for a given geometric
//!    computation such as a ray-surface intercept. For this purpose, it is
//!    convenient to consider the coverage of a segment to be
//!    three-dimensional.
//!
//!  For many applications, it is more natural to consider the spatial
//!    coverage of a segment to be two-dimensional. This is true when the
//!    surface represented by the segment can be expressed as a function that
//!    maps a two-dimensional region to radius or height values. For example,
//!    surface height relative to a reference spheroid can be a function of
//!    planetodetic longitude and latitude. In a topocentric frame, the Z
//!    coordinate of a surface can be a function of the X and Y coordinates.
//!
//!  In cases where a surface is viewed as a function of two coordinates,
//!    those coordinates are called the "domain coordinates." In some DSK
//!    documentation, the terms "horizontal" or "tangential" coordinates
//!    may be used as synonyms.
//!
//!  The DSKBRIEF summary utility displays spatial coverage in three
//!    dimensions for individual segments. It treats spatial coverage as
//!    two-dimensional for the purpose of displaying combined coverage of
//!    multiple segments, and for displaying gaps within that combined
//!    coverage. For such displays, coverage and gaps will be displayed as
//!    longitude-latitude rectangles in the planetocentric or planetodetic
//!    systems, or as X-Y rectangles in the rectangular system.
//!
//!  
//!
//!
//!  
//! ###  Spatial coverage: Gaps and Padding
//!
//!  In general, the spatial bounds of a DSK segment don't imply that there
//!    is complete coverage of the domain coordinates within those bounds;
//!    spatial coverage can have gaps. In some cases this is an inevitable
//!    consequence of the shape of the body: for example, if the coordinate
//!    system is rectangular and the body is a torus lying on the X-Y plane,
//!    there will be no data for some (X,Y) coordinates.
//!
//!  Spatial coverage gaps also can occur due to the way data are grouped in
//!    segments by a DSK file's creator. Normally DSK file creators should
//!    ensure that segments don't have coverage gaps that users would not
//!    expect. Coverage gaps can cause geometric computations to fail at run
//!    time.
//!
//!  The concept of spatial coverage gaps normally applies to a segment's
//!    domain coordinates, such as longitude and latitude. It is also possible
//!    for a surface's maximum or minimum height, radius, or Z coordinate to
//!    be, respectively, strictly less than or strictly greater than the
//!    corresponding upper or lower bound. The term "gap" usually does not
//!    apply to such differences; these differences usually have no impact on
//!    computations.
//!
//!  Data for a segment need not be confined to the spatial region delimited
//!    by the spatial bounds. DSK creators can include in DSK segments
//!    "padding" data that extend slightly beyond the segments' spatial
//!    bounds. Padding data can ensure that coverage implied by a segment's
//!    domain coordinate bounds is really present.
//!
//!  
//!
//!
//!  
//! ###  Time Bounds
//!
//!  Segments in a DSK file provide data for a time interval determined by
//!    the DSK file's creator.
//!
//!  For a surface having a shape that evolves over a time span of interest,
//!    multiple versions of the surface corresponding to different time
//!    intervals can be created. When a computation is performed for a
//!    particular time, only segments providing data for time intervals that
//!    include that time will be considered.
//!
//!  In many cases the surface represented by a DSK segment is considered to
//!    be constant with respect to time, so the start and stop time bounds may
//!    be set, respectively, to values far in the past and future (for example,
//!    plus or minus one century) relative to the time range for which the
//!    segment is expected to be used.
//!
//!  
//!
//!
//!  
//! ###  Data Types
//!
//!  The DSK subsystem supports multiple forms of mathematical
//!    representations of shape data. For each such form, there is a
//!    corresponding DSK segment type and set of routines that can access
//!    segments of that type. Collectively the form of representation, the
//!    segment structure and associated software are called a "DSK data
//!    type."
//!
//!  DSK data type 2 represents body shapes using collections of triangular
//!    plates. Another DSK data type, not yet implemented, will represent
//!    surfaces as digital elevation models (DEMs).
//!
//!  The SPICE system's high-level geometry routines operate without
//!    reference to the data types of DSK segments providing shape data to
//!    these routines. These routines require lower-level, type-dependent
//!    routines to provide functionality that is common across all DSK data
//!    types, such as "find the intercept of a ray with the surface defined by
//!    a segment," or "return the unit length outward normal vector at a
//!    specified point on the surface defined by a segment."
//!
//!  Some SPICE routines perform functions specific to particular data types.
//!    For example, the routines [DSKV02](crate::raw::dskv02) and [DSKP02](crate::raw::dskp02) return, respectively,
//!    vertices and plates from a type 2 segment.
//!
//!  
//!
//!
//!  
//! ###  Data Classes
//!
//!  The "data class" of a segment indicates aspects of the topology of the
//!    shape defined by that segment. There are currently two data classes:
//!
//!  
//!
//! * Class 1: single-valued surface. The surface is a single-valued function of
//! the segment's domain coordinates.
//!
//!  * Class 2: arbitrary surface. Any surface for which there are multiple points
//! for a given longitude and latitude, or for a given X and Y, belongs to
//! class 2.
//!
//!  *  Surfaces that have features such as overhanging cliffs, arches, or caves
//! belong to class 2.
//!
//!  *  Any DSK type 2 surface having an "inward-facing" plate---one for which
//! the outward normal vector has negative dot product with any of the plate's
//! vertices---is a class 2 surface.
//!
//!  *  The nucleus of the comet Churyumov-Gerasimenko is an example of a class 2
//! shape.
//!
//!     
//! ##  Data Competition and Priority
//!
//!  Two segments containing data for the same body, surface, location, and
//!    time are said to contain "competing" data. Segment priority is a
//!    scheme for determining which segment's data to use in case of
//!    competition.
//!
//!  Unlike the other SPICE binary kernel systems, the DSK subsystem does not
//!    necessarily make use of segment priority for a given computation.
//!    Instead, a user application can specify that a computation is
//!    "unprioritized." This means that all loaded data for the given body
//!    and a specified list of surfaces are to be used together to represent
//!    the shape to be used in the computation.
//!
//!  When DSK data for different surfaces for one body are loaded
//!    concurrently, surface lists, which are inputs to SPICE API routines that
//!    use DSK data, can be used to ensure that the correct set of DSK data are
//!    used for a given computation, and that none of the data compete. It is
//!    not necessary to load or unload DSK files to give the desired data top
//!    priority.
//!
//!  See the API documentation for any high-level SPICE geometry routine, for
//!    example [SINCPT](crate::raw::sincpt), for a discussion of surface lists.
//!
//!  In the N0066 SPICE Toolkit, all high-level geometry routines that work
//!    with DSK data support only unprioritized computations.
//!
//!  Although not currently used, the DSK subsystem does have a priority
//!    scheme: as in the SPICE SPK, CK, and binary PCK subsystems, when two
//!    segments from the same file compete, the one located later in the file
//!    (at higher addresses) takes precedence. When two segments from different
//!    files compete, the one from the file loaded later takes precedence.
//!
//!  
//!
//!
//!  
//! ##  Greedy Algorithms
//!
//!  A numerical problem affecting DSK computations is the possibility of
//!    "false negatives": finding no result when one should be expected. An
//!    example of this is not finding a ray-surface intercept when one clearly
//!    should exist.
//!
//!  The possibility of false negatives, at a minimum, complicates the design
//!    of user applications that depend on DSK data.
//!
//!  Aside from errors in DSK files themselves, the main cause of false
//!    negative results is round-off error.
//!
//!  Round-off error can cause a ray that should hit the common edge between
//!    two plates to be determined, according to double precision arithmetic,
//!    to miss both plates. Similarly, round-off error can cause a ray that
//!    should hit the common longitude boundary or latitude boundary shared by
//!    two DSK segments to hit neither segment boundary.
//!
//!  The DSK subsystem uses several techniques to avoid false negative
//!    results for ray-surface intercepts. These fall into the category of
//!    "greedy algorithms": they effectively treat segments and data as
//!    though they occupy not only the spatial regions implied by their
//!    boundaries, but the surrounding regions as well. (An examination of the
//!    default DSK "greedy" parameters will reveal that the greediness of DSK
//!    algorithms is a mild case. See "Access to DSK Tolerances and Margin
//!    Values" below.)
//!
//!  For example, when a ray-surface intercept computation is attempted, an
//!    intercept is considered to exist if the ray passes sufficiently close to
//!    the target---not only if it hits.
//!
//!  
//!
//!
//!  
//! ###  Greedy Ray-Segment Boundary Intercepts
//!
//!  In the discussion below, we refer to both "segment boundaries" and
//!    "tangent surfaces." Both terms require definition:
//!
//!  
//!
//! *  Segment boundary
//!
//!
//!  This is the set of surfaces forming the boundary of the spatial region
//! defined by the coordinate bounds in a segment's DSK descriptor. There
//! are six such surfaces: for each one, one coordinate is at the segment's
//! minimum or maximum value for that coordinate, and the other coordinates
//! vary from their minimum to their maximum.
//!
//!  *  For example, a segment using planetocentric latitudinal coordinates
//! might have the longitude range 0:30 degrees, latitude range 0:45
//! degrees, and radius range 6300:6400 km. Then the segment's boundary on
//! the sphere of radius 6400 km has longitude ranging from 0 to 30
//! degrees, and latitude ranging from 0 to 45 degrees.
//!
//!  *  Tangent surface
//!
//!
//!  This a "level surface" in geometric terms: a surface defined by
//! setting one coordinate to a segment's minimum or maximum bound for that
//! coordinate. For the segment above, the sphere of radius 6400 km is one
//! such tangent surface. As in this example, the tangent surfaces may
//! include areas that are not part of the segment's boundary.
//!
//!  One step in the ray-surface intercept computation is determination of
//!    the set of applicable segments that are hit by the ray. The segments
//!    that have the appropriate attributes are tested to see whether the ray
//!    hits the tangent surfaces defined by the coordinate ranges in the
//!    segments' DSK descriptors.
//!
//!  By default, when the ray's intersection with a segment's tangent surface
//!    is computed, if that intersection is close to the spatial region
//!    indicated by the segment's boundaries, an intersection with the segment
//!    is considered to exist. In the example we're using, a ray's intersection
//!    with the sphere of radius 6400 km that has longitude slightly greater
//!    than 30 degrees and latitude slightly greater than 45 degrees could be
//!    considered an intersection with the segment's boundary, if the longitude
//!    and latitude excursions are within margins.
//!
//!  The goal is to ensure that any segment that the ray might hit is
//!    considered for a more detailed intercept computation.
//!
//!  
//!
//!
//!  
//! ###  DSK Type 2 Plate Expansion
//!
//!  By default, to eliminate the problem of false negatives from ray-surface
//!    intercept computations, where the surface is defined by the data in a
//!    selected type 2 segment, the DSK type 2 ray-surface intercept
//!    computation expands plates slightly before attempting to compute the
//!    ray's intersection with them. The expansion is done by scaling up, by a
//!    factor slightly greater than 1, each of a plate's centroid-vertex
//!    vectors. The expanded plate lies in the plane of the original plate, its
//!    edges are parallel to those of the original plate, and its centroid
//!    coincides with that of the original plate.
//!
//!  The default plate expansion fraction is 1e-10. The expansion factor is
//!
//!  
//!
//! ```text
//!    1 + expansion_fraction
//! ```
//!
//!     
//! ###  Additional Greedy Algorithms
//!
//!  Greedy algorithms are also used for
//!
//!  
//!
//! * Determining whether a ray-plate intercept, once found, is close enough to a
//! segment's boundary to be considered inside the segment
//!
//!  * Determining whether a point is close enough to a plate to be considered to
//! belong to that plate
//!
//!     
//! ###  DSK Tolerances
//!
//!  Some DSK tolerance parameters do not strictly support "greedy"
//!    behavior, but rather enhances the DSK software's usability. For example,
//!    there is a DSK parameter that is used to decide whether a longitude
//!    value is close enough to a segment's longitude range to be considered to
//!    lie within that range. Another tolerance parameter is used to decide
//!    whether angular values are valid. For example, a latitude that exceeds
//!    pi/2 radians by a positive number less than this tolerance is treated as
//!    though it were exactly pi/2.
//!
//!  
//!
//!
//!  
//! ###  Drawbacks
//!
//!  A possible negative consequence of greedy algorithms is that some
//!    results derived using them may be quite different from those derived
//!    without them.
//!
//!  For example, if a ray hits the edge of an expanded plate, and that plate
//!    has a maximum edge length of 1 km, then the ray might have missed the
//!    original plate by as much as 0.1 millimeters. After missing the original
//!    plate, the ray might hit the surface far from that plate, or it might
//!    miss the target altogether.
//!
//!  This issue, while it may appear serious, is a normal consequence of
//!    using finite precision arithmetic. A comparable difference in results
//!    might be observed were the computation without plate expansion performed
//!    on two different computer systems.
//!
//!  
//!
//!
//!  
//! ###  Access to DSK Tolerances and Margin Values
//!
//!  The greedy algorithms described above rely on tolerance parameters.
//!    These and other DSK tolerance and margin parameters are assigned default
//!    values in the include file dsktol.inc.
//!
//!  The parameters used for greedy algorithms are user-adjustable.
//!    Applications can call the routines [DSKGTL](crate::raw::dskgtl) or [DSKSTL](crate::raw::dskstl) to fetch or reset
//!    these parameters at run time.
//!
//!  It is recommended that the parameters be reset only by expert users.
//!
//!  
//!
//!
//!  
//! #  DSK Files
//!
//!  The SPICE DSK file format is designed to support rapid access to large
//!    data sets. It is designed to be portable, and to support inclusion of
//!    documentation within DSK files.
//!
//!  The DSK file format is based on two lower-level SPICE file formats: the
//!    DSK format is a special case of the SPICE DLA format, which in turn is a
//!    special case of SPICE DAS format.
//!
//!  
//!
//!
//!  
//! ##  DAS Files
//!
//!  Most DSK properties are inherited from the "Direct Access Segregated"
//!    (DAS) specification. The properties visible to SPICE users are:
//!
//!  
//!
//! * Binary file format: integers and double precision numbers are stored in
//! binary format for compactness and speed of read access.
//!
//!  * Direct access: data from any part of the file can be read in constant time,
//! aside from latency of the storage medium.
//!
//!  * Array abstraction: the data portion of a DAS file appears to user
//! application code as three arrays of contiguous data: one each of
//! characters, integers, and double precision numbers. User applications can,
//! for example, directly read the Nth double precision number from a DAS file.
//! Fortran I/O features such as record numbers, record lengths, and logical
//! units (or C emulations of these features) are hidden from the user
//! application's view.
//!
//!  * Portability: DAS files can be transferred from IEEE big-endian to IEEE
//! little-endian platforms, and vice-versa, requiring no modification to be
//! readable on the target platform.
//!
//!  *  DAS files can be transferred to ANY platform on which SPICE is supported;
//! this is done by first converting the files to an ASCII "transfer" format
//! on the source platform, moving the files, then converting the transfer
//! format files to binary files on the target platform.
//!
//!  * Platform-independent buffering: DAS software buffers data read from DAS
//! files using a built-in mechanism that's independent of any buffering
//! mechanism provided by the host platform.
//!
//!  * Comment area: DAS files contain a data structure called the "comment
//! area"; this area can contain an arbitrary quantity of free-form, ASCII
//! text. The comment area enables DAS file creators to include explanatory
//! documentation in the files.
//!
//!  *  The SPICE Toolkit contains API routines to access the comment area. Also,
//! the Toolkit contains a utility program, COMMNT, that can read, delete, or
//! append to comments in the comment area.
//!
//!  See the DAS Required Reading [das.req](crate::required_reading::das) for details concerning the DAS
//!    subsystem and DAS file format.
//!
//!  
//!
//!
//!  
//! ##  DLA Files
//!
//!  The DLA file format organizes data into a doubly linked list of virtual
//!    segments. DLA routines enable application software to traverse such
//!    segment lists. Since the DSK file format is an instance of the DLA
//!    format, this traversal capability is inherited by the DSK subsystem.
//!
//!  DLA files indicate the DAS addresses and sizes of their segments'
//!    character, double precision, and integer components using data
//!    structures called "DLA descriptors." The DLA segment descriptor
//!    members are:
//!
//!  
//!
//! ```text
//!  
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
//!  DLA descriptors are used in the DSK subsystem to identify locations of
//!    the components of DSK segments within DSK files.
//!
//!  See the DLA Required Reading, [dla.req](crate::required_reading::dla), for details concerning the DLA
//!    subsystem and file format.
//!
//!  
//!
//!
//!  
//! ##  DSK File Format
//!
//!  For the majority of practical DSK applications, the DSK file format can
//!    be adequately understood using the following simplified model:
//!
//!  
//!
//! ```text
//!    +----------------------------------+
//!    |            File Record           |
//!    +----------------------------------+
//!    |            Comment Area          |
//!    +----------------------------------+
//!    |            Segment 1             |
//!    +----------------------------------+  -+
//!    |            ...                   |   |
//!    +----------------------------------+   | Optional
//!    |            Segment N             |   |
//!    +----------------------------------+  -+
//! ```
//!
//!  That is, a DSK file contains some identification and bookkeeping
//!    information called a "file record," it has a comment area, and it has
//!    one or more DSK segments containing shape data.
//!
//!  The segments are connected to each other as a doubly linked list, and
//!    the list can be traversed in forward or backward order. (In the N0066
//!    Mice and Icy Toolkits, only forward traversal is supported.)
//!
//!  Each DSK segment has integer and double precision components. These
//!    components occupy, respectively, contiguous ranges of DAS integer and
//!    double precision addresses:
//!
//!  
//!
//! ```text
//!    +----------------------------------+
//!    |            Segment I             |  =
//!    +----------------------------------+
//!  
//!    +--------------+     +------------------+
//!    |              |     |                  |
//!    |  Segment I:  |     |    Segment I:    |
//!    |              |     |                  |
//!    |   Integer    |  +  |     Double       |
//!    |   Component  |     |     Precision    |
//!    |   (optional) |     |     Component    |
//!    |              |     |                  |
//!    +--------------+     |                  |
//!                         |                  |
//!                         +------------------+
//! ```
//!
//!  The double precision component of a segment can be further expanded as:
//!
//!  
//!
//! ```text
//!    +------------------+
//!    |                  |
//!    |    Segment I:    |
//!    |                  |
//!    |     Double       |
//!    |     Precision    |   =
//!    |     Component    |
//!    |                  |
//!    |                  |
//!    +------------------+
//!  
//!    +--------------+     +------------------+
//!    |  Segment I:  |     |    Segment I:    |
//!    |              |     |                  |
//!    |  DSK Segment |  +  |     Double       |
//!    |  Descriptor  |     |     Precision    |
//!    +--------------+     |     Data         |
//!                         |     (optional)   |
//!                         |                  |
//!                         |  +  Bookkeeping  |
//!                         |     Information  |
//!                         |     (optional)   |
//!                         |                  |
//!                         +------------------+
//! ```
//!
//!  The base addresses and sizes of a DSK segment's integer and double
//!    precision components are given by the segment's DLA descriptor. The DAS
//!    integer address range of a DSK segment is
//!
//!  
//!
//! ```text
//!    integer base address+1 : integer base address+
//!                             integer component size
//! ```
//!
//!  Similarly, the DAS double precision address range of a DSK segment is
//!
//!  
//!
//! ```text
//!    d.p. base address+1 : d.p. base address+
//!                          d.p. component size
//! ```
//!
//!  Low-level details of the general DSK file format, if not discussed in
//!    this document, can be obtained from the DAS Required Reading, [das.req](crate::required_reading::das),
//!    and from the DLA Required Reading, [dla.req](crate::required_reading::dla).
//!
//!  An abstract view of a DSK segment---a view that ignores physical file
//!    layout and numeric data types---is
//!
//!  
//!
//! ```text
//!  
//!                DSK segment =
//!  
//!    +-------------------------------------+
//!    |            DSK Descriptor           |
//!    +-------------------------------------+
//!    |      Type-specific shape data       |
//!    +-------------------------------------+
//!    | Type-specific ancillary information |
//!    +-------------------------------------+
//!  
//! ```
//!
//!  DSK segments of all data types contain DSK descriptors.
//!
//!  Further details of the DSK segment's structure are data type-dependent.
//!    Currently there is just one DSK data type: type 2. It is discussed
//!    below.
//!
//!  
//!
//!
//!  
//! #  DSK Type 2: Triangular Plate Model
//!
//!  The following discussion of shape data may be of interest to any DSK
//!    users. The discussion of type 2 ancillary information is quite detailed
//!    and likely is not of interest to most DSK users. It may be useful for
//!    DSK creators, especially those interested in optimizing performance.
//!
//!  
//!
//!
//!  
//! ##  Type 2 Shape Data
//!
//!  DSK type 2 represents shapes of objects as collections of triangular
//!    plates. This data type can model nearly any shape: the shape need not be
//!    smooth, connected, or continuously deformable to sphere.
//!
//!  Each triangular plate has three vertices: type 2 data consist of a set
//!    of vertices and a set of "plates" to which the vertices belong.
//!
//!  
//!
//!
//!  
//! ###  Vertices
//!
//!  Vertices are vectors in three-dimensional space: each vertex is
//!    represented by a double precision array consisting of the vertex's X, Y,
//!    and Z components, in that order.
//!
//!  The components of a vertex are expressed in the body-fixed reference
//!    frame of the DSK segment to which the vertex belongs. Each vertex
//!    represents an offset from the center of that reference frame.
//!
//!  The center of a type 2 segment's reference frame need not coincide with
//!    the body for which the segment provides data. For example, vertices for
//!    a DSK segment representing Mars topography might be expressed in a Mars
//!    topocentric frame; the vertices would then represent offsets from the
//!    Mars surface point at the center of that frame.
//!
//!  Within a type 2 segment, vertex components are always expressed in units
//!    of km, regardless of the units associated with the input data from which
//!    the segment was constructed.
//!
//!  Each vertex has an associated integer ID; vertex IDs range from 1 to NV,
//!    where NV is the number of vertices in the segment. This 1-based
//!    numbering scheme is used for all language versions of SPICE, so the
//!    vertex IDs in a DSK file match those used in SPICE DSK code on all
//!    platforms.
//!
//!  
//!
//!
//!  
//! ###  Plates
//!
//!  The term "plate" refers to both a planar surface with a triangular
//!    boundary in 3-dimensional space, and to a data structure.
//!
//!  As a data structure, a plate is a 3-tuple of integer vertex IDs that
//!    indicate which vertices belong to that plate.
//!
//!  Each plate has an associated "outward normal" direction: this
//!    direction is perpendicular to the plate. For surfaces that constitute
//!    boundaries of solid objects---for example, a sphere---the outward normal
//!    direction has the usual meaning: it points toward the exterior of the
//!    object. For other surfaces, for example a single plate, the outward
//!    normal direction may be an arbitrary choice.
//!
//!  The order of a plate's vertices implies the outward normal direction: if
//!    the vertices are
//!
//!  
//!
//! ```text
//!    V(1), V(2), V(3)
//! ```
//!
//!  then the outward normal direction is
//!
//!  
//!
//! ```text
//!    ( V(2) - V(1) )  x  ( V(3) - V(2) )
//! ```
//!
//!  where "x" denotes the vector cross product operator.
//!
//!  DSK creators must take vertex order into account when they define the
//!    plates of a DSK type 2 segment.
//!
//!  As the formula above shows, the outward normal direction is undefined if
//!    two or three vertices of a plate coincide; in this case the "plate" is
//!    actually a line segment or a point. Plates having these characteristics
//!    are termed "degenerate." Even if all of a plate's vertices are
//!    distinct, the normal direction vector suffers great loss of precision if
//!    the angle between two plate edges is very close to zero.
//!
//!  Degenerate and nearly degenerate plates are allowed in type 2 segments,
//!    but it is strongly recommended that DSK creators exclude them from input
//!    data sets. Such plates can cause run-time failures of user applications
//!    performing functions that require outward normal directions to exist,
//!    for example computing emission and solar incidence angles.
//!
//!  Each plate has an associated integer ID; plate IDs range from 1 to NP,
//!    where NP is the number of plates in the segment. This 1-based numbering
//!    scheme is used for all language versions of SPICE, so the plate IDs in a
//!    DSK file match those used in SPICE DSK code on all platforms.
//!
//!  
//!
//!
//!  
//! ##  Type 2 Ancillary Information
//!
//!  Much of the "added value" provided by DSK type 2 segments derives from
//!    their spatial indexes: these enable rapid association between spatial
//!    regions and plates, and between vertices and plates.
//!
//!  A DSK type 2 spatial index consists of a "voxel-plate mapping" and
//!    optionally, a "vertex-plate mapping," as well as various associated
//!    parameters.
//!
//!  All DSK type 2 segments contain a voxel-plate mapping. This mapping
//!    enables DSK type 2 software to rapidly determine which plates are near a
//!    specified ray or point.
//!
//!  
//!
//!
//!  
//! ##  Pointers and Offsets
//!
//!  In a DSK type 2 segment, integers that refer to locations of data or
//!    ancillary information are called "pointers" or "offsets." In this
//!    context "pointer" is not a Fortran data type (Fortran 77 does not have
//!    a pointer type) but an indication of the role of the integer.
//!
//!  In a DSK type 2 segment, pointers and offsets are expressed relative to
//!    the DAS base addresses of that segment, or relative to the DAS addresses
//!    of other members of the segment. This ensures that the segment is
//!    "relocatable": it has no dependence on its absolute DAS addresses and
//!    can be moved or copied without corrupting its contents.
//!
//!  
//!
//!
//!  
//! ##  DSK Type 2 Segment Parameters
//!
//!  The DSK type 2 parameters are constant values stored in both the integer
//!    and double precision components of a DSK type 2 segment. These are
//!    distinct from parameters belonging to the segment's DSK descriptor.
//!
//!  A subset of these parameters refer to the segment's voxel grids. They
//!    are listed here; they are explained later in context.
//!
//!  The integer parameters are:
//!
//!  
//!
//! * Vertex count
//!
//!  * Plate count
//!
//!  * Fine voxel count (redundant, used for convenience)
//!
//!  * Fine voxel grid extents in the X, Y, and Z directions
//!
//!  * Coarse voxel scale
//!
//!  * Size of voxel-plate pointer array
//!
//!  * Size of voxel-plate association array
//!
//!  * Size of vertex-plate association array
//!
//!  The double precision parameters are:
//!
//!  
//!
//! * Fine voxel size (km)
//!
//!  * Vertex bounds in the X, Y, and Z directions (km)
//!
//!  * Voxel grid origin (3-vector, km)
//!
//!     
//! ##  Spatial Index: Voxel-Plate Mapping
//!
//!  The voxel-plate mapping associates regions of space with plates that
//!    intersect those regions.
//!
//!  The voxel-plate mapping plays a critical role in DSK geometry
//!    computations, because it enables plates relevant to computation to be
//!    located quickly:
//!
//!  
//!
//! * For ray-surface intercept computations, the voxel-plate mapping is used to
//! quickly find the plates that are close enough to the ray to be tested for
//! intersection.
//!
//!  * For association of an individual point with a plate, the voxel-plate
//! mapping is used to quickly find plates that are close enough to the point
//! so that inclusion of the point by those plates should be tested.
//!
//!  The voxel-plate mapping is implemented as a data structure comprising
//!    four sub-structures and several associated parameters. The
//!    sub-structures are
//!
//!  
//!
//! * The fine voxel grid (aka the "fine grid")
//!
//!  * The coarse voxel grid (aka the "coarse grid")
//!
//!  * The voxel-plate pointer array
//!
//!  * The voxel-plate association array
//!
//!  The structures refer to each other as shown in the following diagram:
//!
//!  
//!
//! ```text
//!  
//!    +---------------+
//!    |               |
//!    |               |
//!    |               |
//!    |               |                                  +-------------+
//!    |               |                                  |             |
//!    |               |                                  |             |
//!    |               |                                  |             |
//!    |               |                +-------------+   |             |
//!    |               |                |             |   |             |
//!    |               |   +--------+   |             |   |             |
//!    |               |   |        |   |             |   |             |
//!    |     Fine      |   | Coarse |   | Voxel-plate |   | Voxel-plate |
//!    |     voxel     |-->|  voxel |-->|   pointer   |-->| association |
//!    |     grid      |   |  grid  |   |    array    |   |    array    |
//!    |               |   |        |   |             |   |             |
//!    |               |   +--------+   |             |   |             |
//!    |               |                |             |   |             |
//!    |               |                +-------------+   |             |
//!    |               |                                  |             |
//!    |               |                                  |             |
//!    |               |                                  |             |
//!    |               |                                  +-------------+
//!    |               |
//!    |               |
//!    |               |
//!    +---------------+
//!  
//! ```
//!
//!  The structures shown above enable DSK software to map a point in
//!    3-dimensional space to a set of nearby plates as follows:
//!
//!  
//!
//! *  1. The fine voxel containing the point is identified. This is done by a
//! constant-time arithmetic calculation.
//!
//!  *  2. The coarse voxel containing the fine voxel is identified. This is done by a
//! constant-time address calculation.
//!
//!  *  3. The voxel-plate pointer corresponding to the fine voxel is identified. This
//! is done by a constant-time address calculation.
//!
//!  *  4. The list of plates associated with the voxel is fetched. The time used by
//! this process is linear with respect to the count of the voxel's associated
//! plates.
//!
//!  These structures are described in detail in the following sections.
//!
//!  
//!
//!
//!  
//! ###  Fine Voxel Grid
//!
//!  The fine voxel grid is an array, also called a "grid," of cubical
//!    regions in 3-dimensional space. The cubical regions are called "fine
//!    voxels" or simply "voxels" when the term is unambiguous. The overall
//!    shape of the grid is that of a box (a 3-dimensional solid having six
//!    rectangular sides).
//!
//!  The sides of the grid are aligned with the coordinate axes of the DSK
//!    segment's reference frame.
//!
//!  A segment's fine voxel grid contains all vertices in the segment (and
//!    therefore all plates); a small margin is used so that no plate contacts
//!    the grid's boundary. Thus no point on or outside of the grid's boundary
//!    can touch a plate.
//!
//!  The fine voxel grid is fully characterized by
//!
//!  
//!
//! * An origin. This is a 3-vector representing an offset from the reference
//! frame's center. The origin is always placed at the minimum X, Y, and Z
//! coordinates of the grid.
//!
//!  * A voxel size. This is the common edge length of the cubes making up the
//! grid. Units are km. See the discussion of the "fine voxel scale" below.
//!
//!  * X, Y, and Z grid extents. These are the counts of the grid's fine voxels in
//! the directions of the reference frame's coordinate axes.
//!
//!  The fine voxel grid is not implemented by a physical array; there is no
//!    storage cost associated with a large count of fine voxels. (There are
//!    consequences other than storage cost if the fine voxel size is too
//!    small: see the section below titled "Size of the Voxel-Plate
//!    Association Array.")
//!
//!  The diagram below shows the position of the fine voxel grid relative to
//!    its origin, the orientation of the grid relative to the reference
//!    frame's axes, and the relationship between the grid extents and the
//!    dimensions of the fine voxel grid.
//!
//!  In this diagram
//!
//!  
//!
//! * VGREXT is a 3-dimensional integer array containing the X, Y, and Z grid
//! extents
//!
//!  * VOXSIZ is the fine voxel edge length, in km
//!
//!  * O represents the grid's origin
//!
//!  ```text
//!  
//!  
//!  
//!                         .------------.       ^ fine voxel count =
//!                        /            /|       | VGREXT(3)
//!                       /            / |       |
//!                      /            /  |       | length =
//!                     /            /   |       | VGREXT(3) * VOXSIZ (km)
//!     ^ +Z           /            /    .    .  v
//!     |    .        .------------.    /    /
//!     |   /         |            |   /    /
//!     |  /+Y        |            |  /    /  fine voxel count = VRGEXT(2)
//!     | /           |            | /    /
//!     |/            |            |/    /  length =
//!     *------->     O------------*    *   VGREXT(2) * VOXSIZ (km)
//!        +X
//!                   <------------>
//!  
//!    fine voxel count = VGREXT(1)
//!    length           = VGREXT(1) * VOXSIZ (km)
//!  
//!  
//!  
//! ```
//!     
//! ###  Fine Voxel Scale
//!
//!  The fine voxel scale is a parameter that relates the fine voxel size
//!    (referred to above as VOXSIZ) to the average extent of the plates in a
//!    DSK type 2 segment.
//!
//!  A plate's "extent" in the direction of coordinate axis i is the
//!    maximum value of coordinate i, taken over the plate's three vertices,
//!    minus the minimum value of coordinate i, also taken over the plate's
//!    three vertices. The average extent of a segment's plate set is the
//!    average of all the the plates' extents in the X, Y, and Z directions.
//!
//!  The fine voxel scale maps the plate set's extents to a voxel size by:
//!
//!  
//!
//! ```text
//!    VOXSIZ  = file_voxel_scale * average_plate_extent
//! ```
//!
//!     
//! ###  Coarse Voxel Grid
//!
//!  The coarse voxel grid is a data structure that represents the same
//!    spatial region as the fine voxel grid. Unlike the fine grid, the coarse
//!    grid is implemented by an array in the DSK segment. The term "coarse
//!    voxel grid" may refer to either the spatial region or the data
//!    structure, depending on context.
//!
//!  The voxels of the coarse grid are cubes of identical size. The edge
//!    length of the coarse voxels is an integer multiple of the fine voxels'
//!    edge length. Let CGRSCL represent this multiple; then each coarse voxel
//!    contains
//!
//!  
//!
//! ```text
//!          3
//!    CGRSCL
//! ```
//!
//!  fine voxels.
//!
//!  We use the term "parent" to refer to the unique coarse voxel that
//!    contains a specified fine voxel.
//!
//!  Each extent of the fine voxel grid is
//!
//!  
//!
//! ```text
//!    CGRSCL
//! ```
//!
//!  times the corresponding extent of the coarse grid.
//!
//!  The diagram below shows the relationship between the coarse voxel grid's
//!    extents and edge lengths. The integer array VGREXT, as above, contains
//!    the extents of the fine voxel grid. The parameter VOXSIZ contains the
//!    fine voxels' edge length.
//!
//!  
//!
//! ```text
//!  
//!  
//!  
//!          .------------.       ^  coarse voxel count =
//!         /            /|       |  VGREXT(3) / CGRSCL
//!        /            / |       |
//!       /            /  |       |  length =
//!      /            /   |       |  VGREXT(3) * VOXSIZ (km)
//!     /            /    .    .  v
//!    .------------.    /    /
//!    |            |   /    /  coarse voxel count = VRGEXT(2)/CGRSCL
//!    |            |  /    /
//!    |            | /    / length =
//!    |            |/    /  VGREXT(2) * VOXSIZ (km)
//!    *------------*    *
//!  
//!    <------------>
//!  
//!    coarse voxel count = VGREXT(1) / CGRSCL
//!    length             = VGREXT(1) * VOXSIZ (km)
//!  
//!  
//!  
//!  
//! ```
//!
//!  In the description below, the term "integer" refers to an element of
//!    the DAS integer address space.
//!
//!  The DSK integer array representing the coarse voxel grid has one element
//!    for each coarse voxel. If any plates intersect the spatial region
//!    corresponding to a coarse voxel, the corresponding coarse voxel grid
//!    array element contains a pointer into the voxel-plate pointer array. If
//!    no plates intersect that spatial region, the voxel contains the value
//!    zero, which represents a null pointer. (Caution: for reasons of backward
//!    compatibility, the values zero and -1 are used in different parts of DSK
//!    type 2 segments indicate null pointers.)
//!
//!  Note that during the construction of a spatial index, plate-voxel
//!    "intersection" may be determined using a margin so that plates very
//!    near a voxel are considered to intersect it. This is the case for DSK
//!    type 2 segments created by [DSKW02](crate::raw::dskw02) and by MKDSK.
//!
//!  A non-null pointer in a given coarse voxel is a 1-based index of a
//!    pointer set within the voxel-plate pointer array. The pointer set
//!    indicates the locations of plate lists associated with the fine voxels
//!    having the coarse voxel as a parent.
//!
//!  The maximum coarse voxel count within a segment, MAXCGR, is set to
//!
//!  
//!
//! ```text
//!    100000
//! ```
//!
//!  This value cannot be changed in any future version of SPICE.
//!
//!  The value is small enough to make it practical for DSK type 2 software
//!    to buffer the entire coarse voxel grid in memory.
//!
//!  In all DSK type 2 segments, MAXCGR integers are allocated for the coarse
//!    voxel grid, even if the grid is smaller.
//!
//!  
//!
//!
//!  
//! ###  Purpose of the Coarse Voxel Grid
//!
//!  Below, the term "empty" means "not intersected by any plates."
//!
//!  The coarse voxel grid allows type 2 segments to avoid storing pointers
//!    for all fine voxels, since only fine voxels belonging to non-empty
//!    coarse voxels require pointers. Since in many practical cases, the
//!    majority of coarse voxels are empty, this often greatly reduces the
//!    required number of pointers.
//!
//!  The coarse voxel grid also tends to reduce the number of physical file
//!    reads necessary to determine the plate set relevant to a given
//!    computation, since DSK type 2 software often can use it to quickly
//!    determine that a given fine voxel is empty, without looking up a
//!    voxel-plate pointer and then a plate list for that voxel. Any fine voxel
//!    that belongs to an empty coarse voxel is empty as well, and typically
//!    the majority of empty fine voxels do belong to empty coarse voxels.
//!
//!  
//!
//!
//!  
//! ###  Voxel-Plate Pointer Array
//!
//!  The voxel-plate pointer array contains a contiguous set of pointers for
//!    each non-empty coarse voxel. There is one pointer for each fine voxel
//!    having the associated coarse voxel as a parent. Each such "pointer" is
//!    a 1-based index in the voxel-plate association array of the start of the
//!    plate list corresponding to a fine voxel. Null pointers in this
//!    structure are indicated by the value -1.
//!
//!  The pointer set for a given coarse voxel contains one pointer for each
//!    fine voxel having that coarse voxel as a parent, so there are
//!
//!  
//!
//! ```text
//!          3
//!    CGRSCL
//! ```
//!
//!  pointers in each set.
//!
//!  Let NNECVX indicate the number of non-empty coarse voxels. Then the
//!    voxel-plate pointer array has the form:
//!
//!  
//!
//! ```text
//!  
//!    +---------------------+
//!    |  pointer set 1      |
//!    +---------------------+
//!              ...
//!    +---------------------+
//!    |  pointer set NNECVX | (number of non-empty coarse voxels)
//!    +---------------------+
//!  
//! ```
//!
//!  The mapping from the coarse voxel grid to pointer sets in the
//!    voxel-plate pointer array is determined by the segment's data, the voxel
//!    grid parameters, and the order in which the data are processed. As
//!    indicated in the following diagram, no particular relationship should be
//!    assumed to exist between a non-empty coarse voxel's coordinates in the
//!    coarse grid and the position of its pointer set in the voxel-plate
//!    pointer array:
//!
//!  
//!
//! ```text
//!  
//!    Coarse voxel grid            Voxel-plate pointer array
//!  
//!                                  +--------------------------------+
//!                               .->| pointer set for coarse voxel v |
//!                              /   +--------------------------------+
//!                             /
//!    +----------------+      /
//!    |   NULL         |     /
//!    +----------------+    /
//!    | coarse voxel u |--./                       ...
//!    +----------------+  /\
//!    | coarse voxel v |-*  \
//!    +----------------+     \
//!         ...                \     +--------------------------------+
//!    +----------------+       *--> | pointer set for coarse voxel u |
//!    |   NULL         |            +--------------------------------+
//!    +----------------+                           ...
//!    | coarse voxel w |--.
//!    +----------------+   \        +--------------------------------+
//!    |   NULL         |    *-----> | pointer set for coarse voxel w |
//!    +----------------+            +--------------------------------+
//!  
//! ```
//!
//!  Above, the letters
//!
//!  
//!
//! ```text
//!    u, v, w
//! ```
//!
//!  indicate arbitrary voxel indices. The positions of the null values were
//!    selected for this example. They're not representative of an actual DSK
//!    segment.
//!
//!  Within the voxel-plate pointer array, each pointer set has the form:
//!
//!  
//!
//! ```text
//!  
//!    +-----------+
//!    |  pointer  | voxel 1
//!    +-----------+
//!         ...
//!    +-----------+
//!    |  pointer  | voxel CGRSCL**3
//!    +-----------+
//!  
//! ```
//!
//!  Each pointer corresponds to a fine voxel in the coarse voxel associated
//!    with the pointer set. Treating the fine voxels in this coarse voxel as a
//!    1-dimensional array, the first fine voxel maps to the first pointer, and
//!    so on. The ordering of the fine voxels is Fortran-style, so a fine voxel
//!    with 1-based indices (I, J, K) relative to its parent coarse voxel has
//!    the one-dimensional index
//!
//!  
//!
//! ```text
//!  
//!                                        2
//!    I   +  (J-1)*CGRSCL  +  (K-1)*CGRSCL
//!  
//! ```
//!
//!  The fine voxel with coordinates (1, 1, 1) relative to the parent coarse
//!    voxel is located in parent voxel's corner having minimum X, Y, and Z
//!    values in the Cartesian coordinate system associated with the segment's
//!    reference frame.
//!
//!  
//!
//!
//!  
//! ###  Voxel-Plate Association Array
//!
//!  The voxel-plate association array contains, for each non-empty fine
//!    voxel, a list of plate IDs identifying the plates that intersect that
//!    voxel, and a plate count.
//!
//!  Let NNEFVX be the total number of non-empty fine voxels in the fine
//!    grid. Let
//!
//!  
//!
//! ```text
//!    v_1, v_2, ..., v_NNEFVX
//! ```
//!
//!  indicate NNEFVX indices of non-empty fine voxels in arbitrary order. The
//!    voxel-plate association array has the form:
//!
//!  
//!
//! ```text
//!  
//!    +-------------------------+
//!    | List for voxel v_1      |
//!    +-------------------------+
//!    | List for voxel v_2      |
//!    +-------------------------+
//!                ...
//!    +-------------------------+
//!    | List for voxel v_NNEFVX | (number of non-empty fine voxels)
//!    +-------------------------+
//!  
//! ```
//!
//!  Let N be the number of plates in the plate list for voxel v_i. Let
//!
//!  
//!
//! ```text
//!    p_1, p_2, ..., p_N
//! ```
//!
//!  be the plate IDs of these plates. The plate list for the fine voxel at
//!    index v_i has the form
//!
//!  
//!
//! ```text
//!  
//!    +--------------------+
//!    | List count = N     |
//!    +--------------------+
//!    | Plate ID p_1       |
//!    +--------------------+
//!             ...
//!    +--------------------+
//!    | Plate ID p_N       |
//!    +--------------------+
//!  
//! ```
//!
//!  The mapping from a pointer in the voxel-plate pointer array to a plate
//!    list in the voxel-plate association array is determined by the segment's
//!    data, the voxel grid parameters, and the order in which the data are
//!    processed. As indicated in the following diagram, no particular
//!    relationship should be assumed to exist between the position of a
//!    pointer in the voxel-plate pointer array and the position of the
//!    corresponding plate list in the voxel-plate association array:
//!
//!  
//!
//! ```text
//!  
//!  
//!    Voxel-plate                     Voxel-plate association array
//!    pointer array
//!                                  +-------------------------------+
//!                               .->| plate list for voxel u_2      |
//!                              /   +-------------------------------+
//!                             /
//!                            /
//!      pointer set u        /
//!    +----------------+    /
//!    | voxel u_1      |--./                       ...
//!    +----------------+  /\
//!    | voxel u_2      |-*  \
//!    +----------------+     \
//!          ...               \     +-------------------------------+
//!    +----------------+       *--->| plate list for voxel u_1      |
//!    |   NULL         |            +-------------------------------+
//!    +----------------+                           ...
//!    | voxel u_n      |--.
//!    +----------------+   \        +-------------------------------+
//!    |   NULL         |    *------>| plate list for voxel u_n      |
//!    +----------------+            +-------------------------------+
//!  
//!            3
//!      CGRSCL  elements
//!  
//!  
//! ```
//!
//!  Above, the letter "u" indicates an arbitrary pointer set in the
//!    voxel-plate pointer array, which contains NNECVX such sets. The
//!    positions of the null values were selected for this example. They're not
//!    representative of an actual DSK segment.
//!
//!  
//!
//!
//!  
//! ###  Size of the Voxel-Plate Association Array
//!
//!  This section likely is of interest only to DSK creators.
//!
//!  Other than the optional vertex-plate association array, the voxel-plate
//!    association array is the largest ancillary data structure in a DSK type
//!    2 segment. The size, in units of integers, of this array is affected by
//!    the fine voxel scale, which is a user-selectable parameter. For a given
//!    plate and vertex set, the size of fine voxels varies in proportion to
//!    the fine voxel scale.
//!
//!  Let NVOXPL be the size, in units of integers, of the voxel-plate
//!    association array; as above, let NNEFVX be the total count of non-empty
//!    fine voxels; let NP be the segment's plate count. A lower bound on
//!    NVOXPL is
//!
//!  
//!
//! ```text
//!  
//!    NNEFVX + NP
//!  
//! ```
//!
//!  This number reflects the plate counts for the plate lists of each
//!    non-empty fine voxel, plus the presence of each plate ID on at least one
//!    list.
//!
//!  Normally, a large number of plates cross voxel boundaries and so have
//!    their IDs on multiple lists. Hence NVOXPL is normally larger than the
//!    lower bound shown above.
//!
//!  Reducing the fine voxel size improves the discrimination of the fine
//!    grid, which can improve the efficiency of algorithms that must operate
//!    on plates associated with a specified spatial region. For example, in
//!    the ray-surface intercept computation, the count of plates associated
//!    with voxels intersected by the ray will usually decrease as the voxel
//!    size is reduced.
//!
//!  However, as the fine voxel size is reduced, more plates cross voxel
//!    boundaries---such plates are on the plate list of each voxel they
//!    touch---and NVOXPL increases. The memory required to hold the spatial
//!    index increases as well; it may become too large to allow a program
//!    calling [DSKW02](crate::raw::dskw02) (the utility MKDSK is one such program) to run
//!    successfully. If a DSK segment with a very large value of NVOXPL is
//!    successfully created, its large size may have a detrimental effect on
//!    disk access time.
//!
//!  
//!
//!
//!  
//! ##  Spatial Index: Vertex-Plate Mapping
//!
//!  Recall that a "plate" is a 3-tuple of integer vertex IDs. Given a
//!    vertex ID, the vertex-plate mapping enables DSK software to quickly find
//!    the plates that include that vertex ID as one of their own. This enables
//!    geometric algorithms to quickly find the plates that lie close to a
//!    given plate.
//!
//!  Currently (as of the time of release of the N0066 SPICE Toolkit) there
//!    are no SPICE routines that rely on the vertex-plate mapping. Creation of
//!    this mapping is therefore optional. Both the DSK type 2 writer routine
//!    [DSKW02](crate::raw::dskw02) and the utility program MKDSK enable users to indicate whether to
//!    create a vertex-plate mapping in an output DSK segment.
//!
//!  
//!
//!
//!  
//! ###  Structure of the Vertex-Plate Mapping
//!
//!  The vertex-plate mapping consists of two arrays in DAS integer address
//!    space: the vertex-plate pointer array and the vertex-plate association
//!    array. Elements of the former array refer to positions in the latter:
//!
//!  
//!
//! ```text
//!  
//!  
//!                        +--------------+
//!                        |              |
//!                        |              |
//!    +---------------+   |              |
//!    |               |   |              |
//!    |               |   |              |
//!    |               |   |              |
//!    | Vertex-plate  |-->| Vertex-plate |
//!    | pointer array |   | association  |
//!    |               |   |   array      |
//!    |               |   |              |
//!    +---------------+   |              |
//!                        |              |
//!                        |              |
//!                        +--------------+
//!  
//!  
//! ```
//!
//!  Let NV be the number of vertices in the segment. Then the vertex-plate
//!    pointer array contains NV elements, and the ith element indicates the
//!    plate list associated with vertex i.
//!
//!  The vertex-plate association array contains a plate list for each
//!    vertex:
//!
//!  
//!
//! ```text
//!  
//!    +----------------------+
//!    | List for vertex v_1  |
//!    +----------------------+
//!    | List for vertex v_2  |
//!    +----------------------+
//!              ...
//!    +----------------------+
//!    | List for vertex v_NV |
//!    +----------------------+
//!  
//! ```
//!
//!  Let N be the number of plates in the plate list for vertex i. Let
//!
//!  
//!
//! ```text
//!    p_1, p_2, ..., p_N
//! ```
//!
//!  be the plate IDs of these plates. The plate list for vertex i has the
//!    form
//!
//!  
//!
//! ```text
//!  
//!  
//!    +--------------------+
//!    | List count = N     |
//!    +--------------------+
//!    | Plate ID p_1       |
//!    +--------------------+
//!             ...
//!    +--------------------+
//!    | Plate ID p_N       |
//!    +--------------------+
//!  
//! ```
//!
//!  The mapping from a vertex to a plate list in the vertex-plate
//!    association array is determined by the segment's data, the voxel grid
//!    parameters, and the order in which the data are processed. No particular
//!    relationship should be assumed to exist between a vertex ID and the
//!    position of the corresponding plate list in the vertex-plate association
//!    array.
//!
//!  
//!
//!
//!  
//! ###  Size of the Vertex-Plate Mapping Array
//!
//!  This section likely is of interest only to DSK creators.
//!
//!  Let NV be a DSK type 2 segment's vertex count, and let NP be the
//!    segment's plate count. Then the size, in units of integers, of the
//!    vertex-plate pointer array is
//!
//!  
//!
//! ```text
//!    NV
//! ```
//!
//!  and the size of the vertex-plate association array is
//!
//!  
//!
//! ```text
//!    NV  +  3*NP
//! ```
//!
//!  The latter value is due to the facts that
//!
//!  
//!
//! * Each plate ID is on exactly three lists
//!
//!  * There is a list for each of the NV vertices
//!
//!  * Each list contains a plate count
//!
//!  The potentially large size of the vertex-plate association array makes
//!    the optional vertex-plate mapping a good candidate for omission when a
//!    DSK type 2 segment is created.
//!
//!  
//!
//!
//!  
//! ##  Layout of DSK Type 2 Segments
//!
//!  Below we describe the organization of integer and double precision data
//!    and ancillary information in DSK type 2 segments.
//!
//!  
//!
//!
//!  
//! ###  DSK Type 2 Integer Segment Component
//!
//!  The layout in DAS integer address space of the integer items in a DSK
//!    type 2 segment is:
//!
//!  
//!
//! ```text
//!  
//!       +-----------------+
//!       | NV              |   number of vertices
//!       +-----------------+
//!       | NP              |   number of plates
//!       +-----------------+
//!       | NVXTOT          |   total number of voxels
//!       +-----------------+
//!       | VGREXT          |   voxel grid extents, 3 integers
//!       +-----------------+
//!       | CGRSCL          |   coarse voxel grid scale
//!       +-----------------+
//!       | VOXNPT          |   size of voxel-plate pointer list
//!       +-----------------+
//!       | VOXNPL          |   size of voxel-plate association list
//!       +-----------------+
//!       | VTXNPL          |   size of vertex-plate association list
//!       +-----------------+
//!       | PLATES          |   NP 3-tuples of vertex IDs
//!       +-----------------+
//!       | VOXPTR          |   voxel-plate pointer array, variable size
//!       +-----------------+
//!       | VOXPLT          |   voxel-plate association list, variable size
//!       +-----------------+
//!       | VTXPTR          |   vertex-plate pointer array, 0 or
//!       |                 |   NV integers
//!       +-----------------+
//!       | VTXPLT          |   vertex-plate association list,
//!       |                 |   0 or NV + 3*NP integers
//!       +-----------------+
//!       | CGRPTR          |   coarse grid pointers, MAXCGR integers
//!       +-----------------+
//!  
//! ```
//!
//!  The sizes of all variable-size items are stored at known locations, so
//!    the starting position of any item can be calculated. Parameters
//!    specifying offsets of the items from the segment's base integer address
//!    are declared in dsk02.inc. The segment's base integer address is
//!    available from the segment's DLA descriptor.
//!
//!  SPICE provides the low-level utility routine [DSKI02](crate::raw::dski02) to fetch any of the
//!    items shown above. Plates and the plate count may be fetched more
//!    conveniently using the routines [DSKP02](crate::raw::dskp02) and [DSKZ02](crate::raw::dskz02).
//!
//!  
//!
//!
//!  
//! ###  DSK Type 2 Double Precision Segment Component
//!
//!  The layout in DAS double precision address space of the double precision
//!    items in a DSK type 2 segment is:
//!
//!  
//!
//! ```text
//!  
//!       +-------------------+
//!       | DSK descriptor    |  DSKDSZ d.p. values
//!       +-------------------+
//!       | Vertex bounds     |  6 d.p. values (min/max for each component)
//!       +-------------------+
//!       | Voxel grid origin |  3 d.p. values
//!       +-------------------+
//!       | Fine voxel size   |  1 d.p. value
//!       +-------------------+
//!       | Vertices          |  3*NV d.p. values
//!       +-------------------+
//!  
//! ```
//!
//!  The parameter DSKDSZ is declared in dskdsc.inc.
//!
//!  SPICE provides the low-level utility routine [DSKD02](crate::raw::dskd02) to fetch any of the
//!    items shown above. Vertices and the vertex count may be fetched more
//!    conveniently using the routines [DSKV02](crate::raw::dskv02) and [DSKZ02](crate::raw::dskz02).
//!
//!  The DSK segment descriptor layout is:
//!
//!  
//!
//! ```text
//!  
//!       +---------------------+
//!       | Surface ID code     |
//!       +---------------------+
//!       | Center ID code      |
//!       +---------------------+
//!       | Data class code     |
//!       +---------------------+
//!       | Data type           |
//!       +---------------------+
//!       | Ref frame code      |
//!       +---------------------+
//!       | Coord sys code      |
//!       +---------------------+
//!       | Coord sys parameters|  10 d.p. values
//!       +---------------------+
//!       | Min coord 1         |
//!       +---------------------+
//!       | Max coord 1         |
//!       +---------------------+
//!       | Min coord 2         |
//!       +---------------------+
//!       | Max coord 2         |
//!       +---------------------+
//!       | Min coord 3         |
//!       +---------------------+
//!       | Max coord 3         |
//!       +---------------------+
//!       | Start time          |
//!       +---------------------+
//!       | Stop time           |
//!       +---------------------+
//!  
//! ```
//!
//!  The DSK descriptor of a DSK segment may be fetched using the routine [DSKGD](crate::raw::dskgd).
//!
//!  
//!
//!
//!  
//! ###  Coordinate System Parameters
//!
//!  The coordinate system parameter section of a DSK descriptor always
//!    contains 10 elements.
//!
//!  The contents of this section are dependent on the coordinate system. For
//!    planetodetic coordinates, the contents are:
//!
//!  
//!
//! ```text
//!  
//!       +------------------------+
//!       | Equatorial radius (km) |
//!       +------------------------+
//!       | Flattening coefficient |
//!       +------------------------+
//!       | <undefined>            | 8 d.p. values
//!       +------------------------+
//!  
//! ```
//!
//!  These parameters define the axes of a reference ellipsoid. The length of
//!    the polar axis is
//!
//!  
//!
//! ```text
//!  
//!      polar_axis = (1 - flattening_coefficient) * equatorial_axis
//!  
//! ```
//!
//!  For planetocentric latitudinal and rectangular coordinates, all elements
//!    are undefined.
//!
//!  DSK subsystem computations involving a DSK segment always use the
//!    coordinate parameters stored in that segment. These parameters may
//!    differ from those specified in a text PCK used by the same application
//!    program, or from those specified in a different segment for the same
//!    body.
//!
//!  It is not necessarily an error for different sets of coordinate
//!    parameters to be used in a computation, but DSK users should be aware of
//!    which parameters are used for which purpose.
//!
//!  
//!
//!
//!  
//! #  Common Problems
//!
//!  
//!
//!
//!  
//! ##  Slow DSK Computations
//!
//!  Depending on the computation and the DSK data used, DSK-based geometry
//!    computations can range from imperceptibly slower to orders of magnitude
//!    slower than those using triaxial ellipsoid shape models.
//!
//!  SPICE users can speed up DSK computations by several means:
//!
//!  
//!
//! *  1. Store DSK files on a fast medium, such as a solid-state drive.
//!
//!  *  Most DSK applications perform a large number of physical file reads, so
//! speeding up these operations has a large effect on overall speed.
//!
//!  *  For applications using large data sets, the speed of the storage medium can
//! be the dominant factor affecting overall program execution speed.
//!
//!  *  2. Use the lowest-resolution shape model that's suitable for the computation.
//!
//!  *  For example, for a small target body, generation of graphics overlays for
//! limbs, terminators, and subspacecraft points may require a type 2 shape
//! model with only a few thousand plates.
//!
//!  *  Large data sets generally result in slower data access performance. This is
//! due to both slower access to the storage medium, and to the fact that, for
//! some DSK data types, DSK software must perform more operations to find data
//! of interest in a large data set than in a small one. This latter point
//! applies to DSK data type 2.
//!
//!  *  3. Choose the proper computation method for the problem.
//!
//!  *  For example, for a ray-surface intercept computation where there are
//! multiple rays for a given observer, target, and observation time, if it's
//! valid to use the same aberration corrections for all of the rays, then the
//! lower-level routine [DSKXV](crate::raw::dskxv) will perform the computation far faster than
//! [SINCPT](crate::raw::sincpt).
//!
//!  *  Another example: for large target bodies having shapes that are well
//! approximated by ellipsoids, limb and terminator points might be
//! sufficiently accurate if computed using the "GUIDED" rather than the
//! "TANGENT" method. See the API documentation of [LIMBPT](crate::raw::limbpt) and [TERMPT](crate::raw::termpt) for
//! further information.
//!
//!  *  Another example: for an illumination angle computation, a low-resolution
//! surface may, in some cases, yield smoother and more meaningful results than
//! a high-resolution surface.
//!
//!     
//! ##  Non-Portable and Unstable Results
//!
//!  Round-off errors can cause valid yet unexpected results due to the fact
//!    that round-off errors can differ from one computer platform (this term
//!    refers not only to hardware but to math libraries and even compilation
//!    options) to the next. For example:
//!
//!  
//!
//! * A ray-surface intercept computation, given identical inputs, may result in
//! an intercept on one platform and a miss on another.
//!
//!  * The ID of the plate on which a given ray hits a target body may change from
//! one platform to the next.
//!
//!  *  This situation is not hard to contrive in test software: a program designed
//! to aim rays at a type 2 segment's plate edges and vertices can demonstrate
//! it.
//!
//!  * The altitude of an observer above the surface, given identical inputs to
//! the computation, might change drastically from one platform to the next.
//!
//!  "Unstable" results are those that vary greatly in response to small
//!    changes in input values. A small difference in input times, with all
//!    other inputs equal, can make the difference between a ray-surface
//!    intersection and non-intersection, or between spacecraft altitude
//!    measured relative to a plateau vs terrain at the base of a cliff.
//!
//!  These problems are best avoided at the time application software is
//!    designed: software developers must account for the effects of round-off
//!    error.
//!
//!  
//!
//!
//!  
//! ##  Non-Convex and Multi-Valued Surfaces
//!
//!  Non-convex surfaces can thwart logic that is valid for triaxial
//!    ellipsoid models. For example, at a given point on a non-convex surface,
//!    for a specified observer, an emission angle of less than 90 degrees
//!    doesn't necessarily imply that the point is visible from the observer.
//!    Similarly, a solar incidence angle of less than 90 degrees doesn't
//!    necessarily imply the point is illuminated by the sun.
//!
//!  Even a slight deviation from convexity can change numerical results
//!    considerably from those obtained using a triaxial ellipsoid model. For
//!    example, depending on whether there is a mountain in the foreground or
//!    whether the intercept lies in a valley (oriented in the general
//!    direction of the ray's projection on the surface), the range from an
//!    observer to a ray-surface intercept point can be much shorter or longer
//!    than the distance to the ray's intercept on the target body's reference
//!    ellipsoid.
//!
//!  Non-convex surfaces can, in some cases, render some geometric quantities
//!    undefined or unusable. For example, the nearest surface point to a given
//!    point, not on the surface, can have multiple solutions, all in
//!    substantially different directions from the given point. Another
//!    example: the origin of a body-fixed reference frame for an object may be
//!    outside of the object--- a surface modeling a planetary ring would have
//!    this property.
//!
//!  Multi-valued surfaces are those for which, for a given latitude and
//!    longitude, or for a given (X,Y) value, there are multiple radius or
//!    height values. These surfaces can occur due to presence of topographic
//!    features such as cliffs, caves, and arches. They can also occur due to
//!    the large-scale shape of an object, as is the case for the nucleus of
//!    the comet Churyumov-Gerasimenko.
//!
//!  Multi-valued surfaces invite new categories of errors not possible with
//!    single-valued, non-convex surfaces. For example, for a given observer
//!    position, the sub-observer point can vary depending on the observer's
//!    altitude. Software meant for use with single-valued surfaces, for
//!    example the routine [LATSRF](crate::raw::latsrf), may yield incorrect results for such cases.
//!
//!  Multi-valued surfaces can yield discontinuities in derived quantities
//!    that are well-behaved when an ellipsoid is used to model the target's
//!    shape. For example, when an observing spacecraft overflies a cliff, the
//!    observer's altitude can change discontinuously. If the sub-spacecraft
//!    point is corrected for light time, the light time algorithm may converge
//!    slowly or not at all.
//!
//!  Again, these problems are best solved by designing application software
//!    to avoid assumptions appropriate only for ellipsoids.
//!
//!  
//!
//!
//!  
//! ##  DSK File Creation Errors
//!
//!  The variety of possible DSK file creation errors is limited only by the
//!    fact that inputs to the process contain a finite number of bytes. We'll
//!    mention only some of the common ones.
//!
//!  
//!
//!
//!  
//! ###  MKDSK Setup File Errors
//!
//!  While MKDSK can check for obviously invalid values, there are some
//!    values that it either cannot or does not check:
//!
//!  
//!
//! * Central body ID code
//!
//!  * Surface ID code
//!
//!  * Central body reference frame---is it the one to which the data are actually
//! referenced?
//!
//!  * Segment coordinate bounds---are they compatible with the data?
//!
//!  * Angular and distance units
//!
//!  MKDSK does place a copy of the setup file in the comment area of the
//!    output DSK file, so users can check it.
//!
//!  
//!
//!
//!  
//! ###  Data Errors
//!
//!  Some data properties are allowed by MKDSK and [DSKW02](crate::raw::dskw02), but result in DSK
//!    segments that may be unsuitable for some computations. These include:
//!
//!  
//!
//! * Degenerate plates. Some plate generation algorithms can create plates that
//! have zero-length edges (in fact, MKDSK can be induced to do this). Such
//! plates can be written to a DSK segment, but they will cause failure of
//! algorithms that need to compute the outward normal vectors of plates.
//!
//!  * Missing data. Missing data can cause failure of some algorithms such as the
//! sub-observer point computation.
//!
//!     
//! ###  Segment Coverage Errors
//!
//!  It is possible for DSK creators to create segments that don't have the
//!    coverage claimed in the segments' DSK descriptors and intended by the
//!    creator to be present.
//!
//!  This is an easy error to make when a large data set is distributed
//!    across multiple DSK files. The DSK creator may assume that the original
//!    plate set, partitioned among various files, will yield the same coverage
//!    as if all plates were stored in a single segment. Not so---each segment
//!    can only provide the coverage its DSK descriptor claims it has, so if a
//!    plate needed by a segment to provide coverage near, but within, that
//!    segment's boundary is allocated to a different segment, the first
//!    segment's coverage will have a gap.
//!
//!  An artificial, but simple, example of this is a tessellation of a
//!    sphere, using triangular plates. Suppose that the surface is partitioned
//!    into a 6 x 12 grid of segments, each covering a 30 degree by 30 degree
//!    region of planetocentric longitude and latitude. Suppose each segment
//!    contains 225 pairs of plates such that each pair covers a
//!    longitude-latitude rectangle having angular extent 2 degrees by 2
//!    degrees, so each segment is "covered" by 450 plates.
//!
//!  Consider the segment covering the coordinate rectangle
//!
//!  
//!
//! ```text
//!    Planetocentric longitude (deg):    0 to +30
//!    Planetocentric latitude  (deg):  +30 to +60
//! ```
//!
//!  For each plate having two vertices on the segment's southern boundary,
//!    the edge of that plate connecting those vertices has latitude greater
//!    than 30 degrees everywhere but at the vertices themselves. At the
//!    midpoint of that edge, the latitude is actually about 30.00378 degrees.
//!
//!  A ray aimed from an exterior vertex to the center of the sphere will
//!    miss the surface if the longitude of the vertex is 15 degrees and
//!    latitude of the vertex is above 30 degrees but less than the latitude of
//!    the edge's midpoint.
//!
//!  The solution is to create each segment using "padding"---additional
//!    plates extending slightly beyond the segment's southern boundary, so no
//!    ray emanating from the origin and hitting the sphere within the
//!    segment's longitude-latitude coverage can miss all of the segment's
//!    plates.
//!
//!  The same problem exists for all southern segment latitude boundaries
//!    having positive latitude, and for all northern segment latitude
//!    boundaries having negative latitude. All of these boundaries require
//!    padding in order to achieve the intended coverage.
//!
//!  
//!
//!
//!  
//! ###  Poor Data Distribution Across Segments
//!
//!  It's possible to create DSK segments that are technically valid but that
//!    give rise to very slow run-time performance.
//!
//!  A seemingly attractive choice that can lead to this problem is
//!    partitioning a large data set into a small number of files, each of
//!    which contains a large number of plates.
//!
//!  Type 2 segments can contain 10000000 or more plates (see Appendix B),
//!    but as a segment's plate count increases, the speed of DSK ray-surface
//!    intercept computations decreases.
//!
//!  Experience indicates that DSK ray-surface intercepts exhibit an
//!    "economy of small scale" phenomenon, whereby spreading data across
//!    multiple, small segments tends to improve performance. This is true only
//!    up to a point: as the number of segments grows, the amount of time spent
//!    reading new data when switching from one segment to another grows. At
//!    some point this overhead becomes a significant drag on performance.
//!
//!  
//!
//!
//!  
//! #  Appendix A --- Revision History
//!
//!  
//!
//!
//!  
//! ###  2017 APR 05 by N. J. Bachman.
//!
//!  Initial release.
//!
//!  
//!
//!
//!  
//! #  Appendix B --- DSK Subsystem Limits
//!
//!  The limits shown here apply to the N0066 SPICE Toolkit.
//!
//!  See the files
//!
//!  
//!
//! ```text
//!    dskdsc.inc
//!    dsk02.inc
//! ```
//!
//!  for declarations of public parameters defining DSK limits.
//!
//!  
//!
//!
//!  
//! ##  General Limits
//!
//!  
//!
//! * Maximum number of loaded DSK files: 5000
//!
//!  *  The practical limit is lower, since the total number of kernels of all
//! types that can be loaded is 5000.
//!
//!  * Maximum number of loaded DSK segments: 10000
//!
//!  *  Note that the DSK subsystem, unlike the SPK, CK, and binary PCK subsystems,
//! does not search kernels for segments in "search without buffering" mode.
//! Thus instead of suffering greatly degraded performance, a user's
//! application will receive a SPICE error signal if an attempt is made to load
//! too many segments.
//!
//!  * Maximum number of surfaces in a surface list: 100
//!
//!  *  This applies to surface lists in calls to the SPICE APIs that use them.
//!
//!  * Maximum number of surface name-ID pairs that can be defined at run time:
//! 2003
//!
//!     
//! ##  DSK Type 2 Segment Limits
//!
//!  For all platforms:
//!
//!  
//!
//! *  Maximum number of coarse voxels:
//!
//!
//!  100000
//!
//!  *  Maximum number of fine voxels:
//!
//!
//!  100000000
//!
//!  For the platforms
//!
//!  
//!
//! ```text
//!    PC-64BIT-MS_C
//!    PC-CYGWIN-64BIT-GCC_C
//!    PC-CYGWIN-64BIT-GFORTRAN
//!    PC-CYGWIN-GFORTRAN
//!    PC-CYGWIN_C
//!    PC-MS_C
//!    PC-WINDOWS-64BIT-IFORT
//!    PC-WINDOWS-IFORT
//!    SUN-SOLARIS
//!    SUN-SOLARIS-64BIT-GCC_C
//!    SUN-SOLARIS-64BIT-NATIVE_C
//!    SUN-SOLARIS-GCC_C
//!    SUN-SOLARIS-NATIVE_C
//! ```
//!
//!  the following limits apply:
//!
//!  
//!
//! *  Maximum number of plates:
//!
//!
//!  10000000
//!
//!  *  Maximum number of vertices:
//!
//!
//!  5000002
//!
//!  For all others, the limits are:
//!
//!  
//!
//! *  Maximum number of plates:
//!
//!
//!  32000000
//!
//!  *  Maximum number of vertices:
//!
//!
//!  16000002
//!
//!      
