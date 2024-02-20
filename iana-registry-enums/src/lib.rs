use std::convert::TryFrom;
use std::str::FromStr;

use serde::Deserialize;

/// Errors related to parsing IANA registry entries
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum IANARegistryError {
    /// Failed to parse IANA Hash Function Textual Names
    #[error("Failed to parse iana hash function textual name")]
    HashParseError,
    /// Failed to parse IANA Operating System Names
    #[error("Failed to parse iana operating system name")]
    OsNameParseError,
}

// ================================================================================================
/// Represents list of hash function names from IANA Hash Function Textual Names
/// registry: https://www.iana.org/assignments/hash-function-text-names/hash-function-text-names.xhtml
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum HashFunctionTextualName {
    #[serde(rename = "md2")]
    Md2,
    #[serde(rename = "md5")]
    Md5,
    #[serde(rename = "sha-1")]
    Sha1,
    #[serde(rename = "sha-224")]
    Sha224,
    #[serde(rename = "sha-256")]
    Sha256,
    #[serde(rename = "sha-384")]
    Sha384,
    #[serde(rename = "sha-512")]
    Sha512,
    #[serde(rename = "shake128")]
    Shake128,
    #[serde(rename = "shake256")]
    Shake256,
}

impl FromStr for HashFunctionTextualName {
    type Err = IANARegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "md2" => Ok(Self::Md2),
            "md5" => Ok(Self::Md5),
            "sha-1" => Ok(Self::Sha1),
            "sha-224" => Ok(Self::Sha224),
            "sha-256" => Ok(Self::Sha256),
            "sha-384" => Ok(Self::Sha384),
            "sha-512" => Ok(Self::Sha512),
            "shake128" => Ok(Self::Shake128),
            "shake256" => Ok(Self::Shake256),
            _ => Err(Self::Err::HashParseError),
        }
    }
}

impl TryFrom<&str> for HashFunctionTextualName {
    type Error = IANARegistryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "md2" => Ok(Self::Md2),
            "md5" => Ok(Self::Md5),
            "sha-1" => Ok(Self::Sha1),
            "sha-224" => Ok(Self::Sha224),
            "sha-256" => Ok(Self::Sha256),
            "sha-384" => Ok(Self::Sha384),
            "sha-512" => Ok(Self::Sha512),
            "shake128" => Ok(Self::Shake128),
            "shake256" => Ok(Self::Shake256),
            _ => Err(Self::Error::HashParseError),
        }
    }
}

impl Into<&'static str> for HashFunctionTextualName {
    fn into(self) -> &'static str {
        match self {
            HashFunctionTextualName::Md2 => "md2",
            HashFunctionTextualName::Md5 => "md5",
            HashFunctionTextualName::Sha1 => "sha-1",
            HashFunctionTextualName::Sha224 => "sha-224",
            HashFunctionTextualName::Sha256 => "sha-256",
            HashFunctionTextualName::Sha384 => "sha-384",
            HashFunctionTextualName::Sha512 => "sha-512",
            HashFunctionTextualName::Shake128 => "shake128",
            HashFunctionTextualName::Shake256 => "shake256",
        }
    }
}

// ================================================================================================
/// Represents list of operation systems as defined by IANA Operating System Names
/// registry: https://www.iana.org/assignments/operating-system-names/operating-system-names.xhtml
#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum OperatingSystemName {
    #[serde(rename = "AEGIS")]
    Aegis,
    #[serde(rename = "AIX-PS/2")]
    AixPs2,
    #[serde(rename = "AIX/370")]
    Aix370,
    #[serde(rename = "AMIGA-OS-1.2")]
    AmigaOS1_2,
    #[serde(rename = "AMIGA-OS-1.3")]
    AmigaOS1_3,
    #[serde(rename = "AMIGA-OS-2.0")]
    AmigaOS2_0,
    #[serde(rename = "AMIGA-OS-2.1")]
    AmigaOS2_1,
    #[serde(rename = "AMIGA-OS-3.0")]
    AmigaOS3_0,
    #[serde(rename = "AMIGA-OS-3.1")]
    AmigaOS3_1,
    #[serde(rename = "AMIGA-OS-3.5")]
    AmigaOS3_5,
    #[serde(rename = "AMIGA-OS-3.9")]
    AmigaOS3_9,
    #[serde(rename = "APOLLO")]
    Apollo,
    #[serde(rename = "BEOS-4.5.2")]
    BeOS4_5_2,
    #[serde(rename = "BEOS-5.0")]
    BeOS5_0,
    #[serde(rename = "BS-2000")]
    BS2000,
    #[serde(rename = "CEDAR")]
    Cedar,
    #[serde(rename = "CGW")]
    CGW,
    #[serde(rename = "CHORUS")]
    Chorus,
    #[serde(rename = "CHRYSALIS")]
    Chrysalis,
    #[serde(rename = "CMOS")]
    CMOS,
    #[serde(rename = "CMS")]
    CMS,
    #[serde(rename = "COS")]
    COS,
    #[serde(rename = "CPIX")]
    CPIX,
    #[serde(rename = "CTOS")]
    CTOS,
    #[serde(rename = "CTSS")]
    CTSS,
    #[serde(rename = "CYGWIN")]
    CYGWIN,
    #[serde(rename = "DCN")]
    DCN,
    #[serde(rename = "DDNOS")]
    DDNOS,
    #[serde(rename = "DOMAIN")]
    DOMAIN,
    #[serde(rename = "DOS")]
    DOS,
    #[serde(rename = "ECOS")]
    ECOS,
    #[serde(rename = "ECOSPRO")]
    ECOSPRO,
    #[serde(rename = "EDX")]
    EDX,
    #[serde(rename = "ELF")]
    ELF,
    #[serde(rename = "EMBOS")]
    EMBOS,
    #[serde(rename = "EMMOS")]
    EMMOS,
    #[serde(rename = "EPOS")]
    EPOS,
    #[serde(rename = "FOONEX")]
    FOONEX,
    #[serde(rename = "FORTH")]
    FORTH,
    #[serde(rename = "FREEBSD")]
    FREEBSD,
    #[serde(rename = "FUZZ")]
    FUZZ,
    #[serde(rename = "GCOS")]
    GCOS,
    #[serde(rename = "GPOS")]
    GPOS,
    #[serde(rename = "HDOS")]
    HDOS,
    #[serde(rename = "IMAGEN")]
    IMAGEN,
    #[serde(rename = "IMPRESS")]
    IMPRESS,
    #[serde(rename = "INSTANT-INTERNET")]
    InstantInternet,
    #[serde(rename = "INTERCOM")]
    INTERCOM,
    #[serde(rename = "INTERLISP")]
    INTERLISP,
    #[serde(rename = "IOS")]
    IOS,
    #[serde(rename = "IRIX")]
    IRIX,
    #[serde(rename = "ISI-68020")]
    ISI68020,
    #[serde(rename = "ITS")]
    ITS,
    #[serde(rename = "KOSOS")]
    KOSOS,
    #[serde(rename = "LINUX")]
    Linux,
    #[serde(rename = "LINUX-1.0")]
    Linux1_0,
    #[serde(rename = "LINUX-1.2")]
    Linux1_2,
    #[serde(rename = "LINUX-2.0")]
    Linux2_0,
    #[serde(rename = "LINUX-2.2")]
    Linux2_2,
    #[serde(rename = "LINUX-2.4")]
    Linux2_4,
    #[serde(rename = "LINUX-2.6")]
    Linux2_6,
    #[serde(rename = "LISP")]
    LISP,
    #[serde(rename = "LISPM")]
    LISPM,
    #[serde(rename = "LOCUS")]
    LOCUS,
    #[serde(rename = "MACOS")]
    MacOS,
    #[serde(rename = "MINOS")]
    MINOS,
    #[serde(rename = "MOS")]
    MOS,
    #[serde(rename = "MPE/IX")]
    MpeIx,
    #[serde(rename = "MPE/V")]
    MpeV,
    #[serde(rename = "MPE5")]
    Mpe5,
    #[serde(rename = "MSDOS")]
    MsDOS,
    #[serde(rename = "MULTICS")]
    Multics,
    #[serde(rename = "MUSIC")]
    Music,
    #[serde(rename = "MUSIC/SP")]
    MusicSP,
    #[serde(rename = "MVS")]
    Mvs,
    #[serde(rename = "MVS/SP")]
    MvsSP,
    #[serde(rename = "NETBSD-1.0")]
    NetBSD1_0,
    #[serde(rename = "NETBSD-1.1")]
    NetBSD1_1,
    #[serde(rename = "NETBSD-1.2")]
    NetBSD1_2,
    #[serde(rename = "NETBSD-1.3")]
    NetBSD1_3,
    #[serde(rename = "NETWARE-3")]
    Netware3,
    #[serde(rename = "NETWARE-3.11")]
    Netware3_11,
    #[serde(rename = "NETWARE-4.0")]
    Netware4_0,
    #[serde(rename = "NETWARE-4.1")]
    Netware4_1,
    #[serde(rename = "NETWARE-5.0")]
    Netware5_0,
    #[serde(rename = "NEXUS")]
    Nexus,
    #[serde(rename = "NMS")]
    NMS,
    #[serde(rename = "NONSTOP")]
    NonStop,
    #[serde(rename = "NOS-2")]
    NOS2,
    #[serde(rename = "NTOS")]
    NTOS,
    #[serde(rename = "OPENBSD")]
    OpenBSD,
    #[serde(rename = "OPENVME")]
    OpenVME,
    #[serde(rename = "OPENVMS")]
    OpenVMS,
    #[serde(rename = "OS/2")]
    Os2,
    #[serde(rename = "OS/DDP")]
    OsDDP,
    #[serde(rename = "OS4")]
    Os4,
    #[serde(rename = "OS86")]
    Os86,
    #[serde(rename = "OSX")]
    OSX,
    #[serde(rename = "PCDOS")]
    PCDOS,
    #[serde(rename = "PERQ/OS")]
    PerqOs,
    #[serde(rename = "PLI")]
    PLI,
    #[serde(rename = "PRIMOS")]
    PRIMOS,
    #[serde(rename = "PSDOS/MIT")]
    PsdosMit,
    #[serde(rename = "PSOS")]
    PSOS,
    #[serde(rename = "RISC-OS")]
    RiscOS,
    #[serde(rename = "RISC-OS-3.10")]
    RiscOS3_10,
    #[serde(rename = "RISC-OS-3.50")]
    RiscOS3_50,
    #[serde(rename = "RISC-OS-3.60")]
    RiscOS3_60,
    #[serde(rename = "RISC-OS-3.70")]
    RiscOS3_70,
    #[serde(rename = "RISC-OS-4.00")]
    RiscOS4_00,
    #[serde(rename = "RMX/RDOS")]
    RmxRDOS,
    #[serde(rename = "ROS")]
    ROS,
    #[serde(rename = "RSX11M")]
    RSX11M,
    #[serde(rename = "RTE-A")]
    RteA,
    #[serde(rename = "SATOPS")]
    SATOPS,
    #[serde(rename = "SCO-OPEN-DESKTOP-1.0")]
    ScoOpenDesktop1_0,
    #[serde(rename = "SCO-OPEN-DESKTOP-1.1")]
    ScoOpenDesktop1_1,
    #[serde(rename = "SCO-OPEN-DESKTOP-2.0")]
    ScoOpenDesktop2_0,
    #[serde(rename = "SCO-OPEN-DESKTOP-3.0")]
    ScoOpenDesktop3_0,
    #[serde(rename = "SCO-OPEN-DESKTOP-LITE-3.0")]
    ScoOpenDesktopLite3_0,
    #[serde(rename = "SCO-OPEN-SERVER-3.0")]
    ScoOpenServer3_0,
    #[serde(rename = "SCO-UNIX-3.2.0")]
    ScoUnix3_2_0,
    #[serde(rename = "SCO-UNIX-3.2V2.0")]
    ScoUnix3_2V2_0,
    #[serde(rename = "SCO-UNIX-3.2V2.1")]
    ScoUnix3_2V2_1,
    #[serde(rename = "SCO-UNIX-3.2V4.0")]
    ScoUnix3_2V4_0,
    #[serde(rename = "SCO-UNIX-3.2V4.1")]
    ScoUnix3_2V4_1,
    #[serde(rename = "SCO-UNIX-3.2V4.2")]
    ScoUnix3_2V4_2,
    #[serde(rename = "SCO-XENIX-386-2.3.2")]
    ScoXenix386_2_3_2,
    #[serde(rename = "SCO-XENIX-386-2.3.3")]
    ScoXenix386_2_3_3,
    #[serde(rename = "SCO-XENIX-386-2.3.4")]
    ScoXenix386_2_3_4,
    #[serde(rename = "SCS")]
    SCS,
    #[serde(rename = "SIMP")]
    Simp,
    #[serde(rename = "SINIX")]
    Sinix,
    #[serde(rename = "SUN")]
    Sun,
    #[serde(rename = "SUN-OS-3.5")]
    SunOs3_5,
    #[serde(rename = "SUN-OS-4.0")]
    SunOs4_0,
    #[serde(rename = "SWIFT")]
    Swift,
    #[serde(rename = "TAC")]
    TAC,
    #[serde(rename = "TANDEM")]
    Tandem,
    #[serde(rename = "TENEX")]
    Tenex,
    #[serde(rename = "THE-MAJOR-BBS")]
    TheMajorBBS,
    #[serde(rename = "TOPS10")]
    Tops10,
    #[serde(rename = "TOPS20")]
    Tops20,
    #[serde(rename = "TOS")]
    TOS,
    #[serde(rename = "TP3010")]
    TP3010,
    #[serde(rename = "TRSDOS")]
    TRSDOS,
    #[serde(rename = "ULTRIX")]
    Ultrix,
    #[serde(rename = "UNIX")]
    Unix,
    #[serde(rename = "UNIX-BSD")]
    UnixBSD,
    #[serde(rename = "UNIX-PC")]
    UnixPC,
    #[serde(rename = "UNIX-V")]
    UnixV,
    #[serde(rename = "UNIX-V.1")]
    UnixV1,
    #[serde(rename = "UNIX-V.2")]
    UnixV2,
    #[serde(rename = "UNIX-V.3")]
    UnixV3,
    #[serde(rename = "UNIX-V1AT")]
    UnixV1AT,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UT2D")]
    UT2D,
    #[serde(rename = "V")]
    V,
    #[serde(rename = "VM")]
    VM,
    #[serde(rename = "VM/370")]
    VM370,
    #[serde(rename = "VM/CMS")]
    VMCMS,
    #[serde(rename = "VM/SP")]
    VMSP,
    #[serde(rename = "VMS")]
    VMS,
    #[serde(rename = "VMS/EUNICE")]
    VMSEunice,
    #[serde(rename = "VRTX")]
    VRTX,
    #[serde(rename = "WAITS")]
    Waits,
    #[serde(rename = "WANG")]
    Wang,
    #[serde(rename = "WIN32")]
    Win32,
    #[serde(rename = "WINDOWS-95")]
    Windows95,
    #[serde(rename = "WINDOWS-95-OSR1")]
    Windows95OSR1,
    #[serde(rename = "WINDOWS-95-OSR2")]
    Windows95OSR2,
    #[serde(rename = "WINDOWS-98")]
    Windows98,
    #[serde(rename = "WINDOWS-CE")]
    WindowsCE,
    #[serde(rename = "WINDOWS-NT")]
    WindowsNT,
    #[serde(rename = "WINDOWS-NT-2")]
    WindowsNT2,
    #[serde(rename = "WINDOWS-NT-3")]
    WindowsNT3,
    #[serde(rename = "WINDOWS-NT-3.5")]
    WindowsNT3_5,
    #[serde(rename = "WINDOWS-NT-3.51")]
    WindowsNT3_51,
    #[serde(rename = "WINDOWS-NT-4")]
    WindowsNT4,
    #[serde(rename = "WINDOWS-NT-5")]
    WindowsNT5,
    #[serde(rename = "WINDOWS-NT-5.1")]
    WindowsNT5_1,
    #[serde(rename = "WINDOWS-NT-6")]
    WindowsNT6,
    #[serde(rename = "WINDOWS-NT-6.1")]
    WindowsNT6_1,
    #[serde(rename = "WORLDGROUP")]
    WorldGroup,
    #[serde(rename = "WYSE-WYXWARE")]
    WyseWyxware,
    #[serde(rename = "X11R3")]
    X11R3,
    #[serde(rename = "XDE")]
    XDE,
    #[serde(rename = "XENIX")]
    Xenix,
}

impl FromStr for OperatingSystemName {
    type Err = IANARegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AEGIS" => Ok(Self::Aegis),
            "AIX-PS/2" => Ok(Self::AixPs2),
            "AIX/370" => Ok(Self::Aix370),
            "AMIGA-OS-1.2" => Ok(Self::AmigaOS1_2),
            "AMIGA-OS-1.3" => Ok(Self::AmigaOS1_3),
            "AMIGA-OS-2.0" => Ok(Self::AmigaOS2_0),
            "AMIGA-OS-2.1" => Ok(Self::AmigaOS2_1),
            "AMIGA-OS-3.0" => Ok(Self::AmigaOS3_0),
            "AMIGA-OS-3.1" => Ok(Self::AmigaOS3_1),
            "AMIGA-OS-3.5" => Ok(Self::AmigaOS3_5),
            "AMIGA-OS-3.9" => Ok(Self::AmigaOS3_9),
            "APOLLO" => Ok(Self::Apollo),
            "BEOS-4.5.2" => Ok(Self::BeOS4_5_2),
            "BEOS-5.0" => Ok(Self::BeOS5_0),
            "BS-2000" => Ok(Self::BS2000),
            "CEDAR" => Ok(Self::Cedar),
            "CGW" => Ok(Self::CGW),
            "CHORUS" => Ok(Self::Chorus),
            "CHRYSALIS" => Ok(Self::Chrysalis),
            "CMOS" => Ok(Self::CMOS),
            "CMS" => Ok(Self::CMS),
            "COS" => Ok(Self::COS),
            "CPIX" => Ok(Self::CPIX),
            "CTOS" => Ok(Self::CTOS),
            "CTSS" => Ok(Self::CTSS),
            "CYGWIN" => Ok(Self::CYGWIN),
            "DCN" => Ok(Self::DCN),
            "DDNOS" => Ok(Self::DDNOS),
            "DOMAIN" => Ok(Self::DOMAIN),
            "DOS" => Ok(Self::DOS),
            "ECOS" => Ok(Self::ECOS),
            "ECOSPRO" => Ok(Self::ECOSPRO),
            "EDX" => Ok(Self::EDX),
            "ELF" => Ok(Self::ELF),
            "EMBOS" => Ok(Self::EMBOS),
            "EMMOS" => Ok(Self::EMMOS),
            "EPOS" => Ok(Self::EPOS),
            "FOONEX" => Ok(Self::FOONEX),
            "FORTH" => Ok(Self::FORTH),
            "FREEBSD" => Ok(Self::FREEBSD),
            "FUZZ" => Ok(Self::FUZZ),
            "GCOS" => Ok(Self::GCOS),
            "GPOS" => Ok(Self::GPOS),
            "HDOS" => Ok(Self::HDOS),
            "IMAGEN" => Ok(Self::IMAGEN),
            "IMPRESS" => Ok(Self::IMPRESS),
            "INSTANT-INTERNET" => Ok(Self::InstantInternet),
            "INTERCOM" => Ok(Self::INTERCOM),
            "INTERLISP" => Ok(Self::INTERLISP),
            "IOS" => Ok(Self::IOS),
            "IRIX" => Ok(Self::IRIX),
            "ISI-68020" => Ok(Self::ISI68020),
            "ITS" => Ok(Self::ITS),
            "KOSOS" => Ok(Self::KOSOS),
            "LINUX" => Ok(Self::Linux),
            "LINUX-1.0" => Ok(Self::Linux1_0),
            "LINUX-1.2" => Ok(Self::Linux1_2),
            "LINUX-2.0" => Ok(Self::Linux2_0),
            "LINUX-2.2" => Ok(Self::Linux2_2),
            "LINUX-2.4" => Ok(Self::Linux2_4),
            "LINUX-2.6" => Ok(Self::Linux2_6),
            "LISP" => Ok(Self::LISP),
            "LISPM" => Ok(Self::LISPM),
            "LOCUS" => Ok(Self::LOCUS),
            "MACOS" => Ok(Self::MacOS),
            "MINOS" => Ok(Self::MINOS),
            "MOS" => Ok(Self::MOS),
            "MPE/IX" => Ok(Self::MpeIx),
            "MPE/V" => Ok(Self::MpeV),
            "MPE5" => Ok(Self::Mpe5),
            "MSDOS" => Ok(Self::MsDOS),
            "MULTICS" => Ok(Self::Multics),
            "MUSIC" => Ok(Self::Music),
            "MUSIC/SP" => Ok(Self::MusicSP),
            "MVS" => Ok(Self::Mvs),
            "MVS/SP" => Ok(Self::MvsSP),
            "NETBSD-1.0" => Ok(Self::NetBSD1_0),
            "NETBSD-1.1" => Ok(Self::NetBSD1_1),
            "NETBSD-1.2" => Ok(Self::NetBSD1_2),
            "NETBSD-1.3" => Ok(Self::NetBSD1_3),
            "NETWARE-3" => Ok(Self::Netware3),
            "NETWARE-3.11" => Ok(Self::Netware3_11),
            "NETWARE-4.0" => Ok(Self::Netware4_0),
            "NETWARE-4.1" => Ok(Self::Netware4_1),
            "NETWARE-5.0" => Ok(Self::Netware5_0),
            "NEXUS" => Ok(Self::Nexus),
            "NMS" => Ok(Self::NMS),
            "NONSTOP" => Ok(Self::NonStop),
            "NOS-2" => Ok(Self::NOS2),
            "NTOS" => Ok(Self::NTOS),
            "OPENBSD" => Ok(Self::OpenBSD),
            "OPENVME" => Ok(Self::OpenVME),
            "OPENVMS" => Ok(Self::OpenVMS),
            "OS/2" => Ok(Self::Os2),
            "OS/DDP" => Ok(Self::OsDDP),
            "OS4" => Ok(Self::Os4),
            "OS86" => Ok(Self::Os86),
            "OSX" => Ok(Self::OSX),
            "PCDOS" => Ok(Self::PCDOS),
            "PERQ/OS" => Ok(Self::PerqOs),
            "PLI" => Ok(Self::PLI),
            "PRIMOS" => Ok(Self::PRIMOS),
            "PSDOS/MIT" => Ok(Self::PsdosMit),
            "PSOS" => Ok(Self::PSOS),
            "RISC-OS" => Ok(Self::RiscOS),
            "RISC-OS-3.10" => Ok(Self::RiscOS3_10),
            "RISC-OS-3.50" => Ok(Self::RiscOS3_50),
            "RISC-OS-3.60" => Ok(Self::RiscOS3_60),
            "RISC-OS-3.70" => Ok(Self::RiscOS3_70),
            "RISC-OS-4.00" => Ok(Self::RiscOS4_00),
            "RMX/RDOS" => Ok(Self::RmxRDOS),
            "ROS" => Ok(Self::ROS),
            "RSX11M" => Ok(Self::RSX11M),
            "RTE-A" => Ok(Self::RteA),
            "SATOPS" => Ok(Self::SATOPS),
            "SCO-OPEN-DESKTOP-1.0" => Ok(Self::ScoOpenDesktop1_0),
            "SCO-OPEN-DESKTOP-1.1" => Ok(Self::ScoOpenDesktop1_1),
            "SCO-OPEN-DESKTOP-2.0" => Ok(Self::ScoOpenDesktop2_0),
            "SCO-OPEN-DESKTOP-3.0" => Ok(Self::ScoOpenDesktop3_0),
            "SCO-OPEN-DESKTOP-LITE-3.0" => Ok(Self::ScoOpenDesktopLite3_0),
            "SCO-OPEN-SERVER-3.0" => Ok(Self::ScoOpenServer3_0),
            "SCO-UNIX-3.2.0" => Ok(Self::ScoUnix3_2_0),
            "SCO-UNIX-3.2V2.0" => Ok(Self::ScoUnix3_2V2_0),
            "SCO-UNIX-3.2V2.1" => Ok(Self::ScoUnix3_2V2_1),
            "SCO-UNIX-3.2V4.0" => Ok(Self::ScoUnix3_2V4_0),
            "SCO-UNIX-3.2V4.1" => Ok(Self::ScoUnix3_2V4_1),
            "SCO-UNIX-3.2V4.2" => Ok(Self::ScoUnix3_2V4_2),
            "SCO-XENIX-386-2.3.2" => Ok(Self::ScoXenix386_2_3_2),
            "SCO-XENIX-386-2.3.3" => Ok(Self::ScoXenix386_2_3_3),
            "SCO-XENIX-386-2.3.4" => Ok(Self::ScoXenix386_2_3_4),
            "SCS" => Ok(Self::SCS),
            "SIMP" => Ok(Self::Simp),
            "SINIX" => Ok(Self::Sinix),
            "SUN" => Ok(Self::Sun),
            "SUN-OS-3.5" => Ok(Self::SunOs3_5),
            "SUN-OS-4.0" => Ok(Self::SunOs4_0),
            "SWIFT" => Ok(Self::Swift),
            "TAC" => Ok(Self::TAC),
            "TANDEM" => Ok(Self::Tandem),
            "TENEX" => Ok(Self::Tenex),
            "THE-MAJOR-BBS" => Ok(Self::TheMajorBBS),
            "TOPS10" => Ok(Self::Tops10),
            "TOPS20" => Ok(Self::Tops20),
            "TOS" => Ok(Self::TOS),
            "TP3010" => Ok(Self::TP3010),
            "TRSDOS" => Ok(Self::TRSDOS),
            "ULTRIX" => Ok(Self::Ultrix),
            "UNIX" => Ok(Self::Unix),
            "UNIX-BSD" => Ok(Self::UnixBSD),
            "UNIX-PC" => Ok(Self::UnixPC),
            "UNIX-V" => Ok(Self::UnixV),
            "UNIX-V.1" => Ok(Self::UnixV1),
            "UNIX-V.2" => Ok(Self::UnixV2),
            "UNIX-V.3" => Ok(Self::UnixV3),
            "UNIX-V1AT" => Ok(Self::UnixV1AT),
            "UNKNOWN" => Ok(Self::Unknown),
            "UT2D" => Ok(Self::UT2D),
            "V" => Ok(Self::V),
            "VM" => Ok(Self::VM),
            "VM/370" => Ok(Self::VM370),
            "VM/CMS" => Ok(Self::VMCMS),
            "VM/SP" => Ok(Self::VMSP),
            "VMS" => Ok(Self::VMS),
            "VMS/EUNICE" => Ok(Self::VMSEunice),
            "VRTX" => Ok(Self::VRTX),
            "WAITS" => Ok(Self::Waits),
            "WANG" => Ok(Self::Wang),
            "WIN32" => Ok(Self::Win32),
            "WINDOWS-95" => Ok(Self::Windows95),
            "WINDOWS-95-OSR1" => Ok(Self::Windows95OSR1),
            "WINDOWS-95-OSR2" => Ok(Self::Windows95OSR2),
            "WINDOWS-98" => Ok(Self::Windows98),
            "WINDOWS-CE" => Ok(Self::WindowsCE),
            "WINDOWS-NT" => Ok(Self::WindowsNT),
            "WINDOWS-NT-2" => Ok(Self::WindowsNT2),
            "WINDOWS-NT-3" => Ok(Self::WindowsNT3),
            "WINDOWS-NT-3.5" => Ok(Self::WindowsNT3_5),
            "WINDOWS-NT-3.51" => Ok(Self::WindowsNT3_51),
            "WINDOWS-NT-4" => Ok(Self::WindowsNT4),
            "WINDOWS-NT-5" => Ok(Self::WindowsNT5),
            "WINDOWS-NT-5.1" => Ok(Self::WindowsNT5_1),
            "WINDOWS-NT-6" => Ok(Self::WindowsNT6),
            "WINDOWS-NT-6.1" => Ok(Self::WindowsNT6_1),
            "WORLDGROUP" => Ok(Self::WorldGroup),
            "WYSE-WYXWARE" => Ok(Self::WyseWyxware),
            "X11R3" => Ok(Self::X11R3),
            "XDE" => Ok(Self::XDE),
            "XENIX" => Ok(Self::Xenix),
            _ => Err(Self::Err::OsNameParseError),
        }
    }
}

impl TryFrom<&str> for OperatingSystemName {
    type Error = IANARegistryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "AEGIS" => Ok(Self::Aegis),
            "AIX-PS/2" => Ok(Self::AixPs2),
            "AIX/370" => Ok(Self::Aix370),
            "AMIGA-OS-1.2" => Ok(Self::AmigaOS1_2),
            "AMIGA-OS-1.3" => Ok(Self::AmigaOS1_3),
            "AMIGA-OS-2.0" => Ok(Self::AmigaOS2_0),
            "AMIGA-OS-2.1" => Ok(Self::AmigaOS2_1),
            "AMIGA-OS-3.0" => Ok(Self::AmigaOS3_0),
            "AMIGA-OS-3.1" => Ok(Self::AmigaOS3_1),
            "AMIGA-OS-3.5" => Ok(Self::AmigaOS3_5),
            "AMIGA-OS-3.9" => Ok(Self::AmigaOS3_9),
            "APOLLO" => Ok(Self::Apollo),
            "BEOS-4.5.2" => Ok(Self::BeOS4_5_2),
            "BEOS-5.0" => Ok(Self::BeOS5_0),
            "BS-2000" => Ok(Self::BS2000),
            "CEDAR" => Ok(Self::Cedar),
            "CGW" => Ok(Self::CGW),
            "CHORUS" => Ok(Self::Chorus),
            "CHRYSALIS" => Ok(Self::Chrysalis),
            "CMOS" => Ok(Self::CMOS),
            "CMS" => Ok(Self::CMS),
            "COS" => Ok(Self::COS),
            "CPIX" => Ok(Self::CPIX),
            "CTOS" => Ok(Self::CTOS),
            "CTSS" => Ok(Self::CTSS),
            "CYGWIN" => Ok(Self::CYGWIN),
            "DCN" => Ok(Self::DCN),
            "DDNOS" => Ok(Self::DDNOS),
            "DOMAIN" => Ok(Self::DOMAIN),
            "DOS" => Ok(Self::DOS),
            "ECOS" => Ok(Self::ECOS),
            "ECOSPRO" => Ok(Self::ECOSPRO),
            "EDX" => Ok(Self::EDX),
            "ELF" => Ok(Self::ELF),
            "EMBOS" => Ok(Self::EMBOS),
            "EMMOS" => Ok(Self::EMMOS),
            "EPOS" => Ok(Self::EPOS),
            "FOONEX" => Ok(Self::FOONEX),
            "FORTH" => Ok(Self::FORTH),
            "FREEBSD" => Ok(Self::FREEBSD),
            "FUZZ" => Ok(Self::FUZZ),
            "GCOS" => Ok(Self::GCOS),
            "GPOS" => Ok(Self::GPOS),
            "HDOS" => Ok(Self::HDOS),
            "IMAGEN" => Ok(Self::IMAGEN),
            "IMPRESS" => Ok(Self::IMPRESS),
            "INSTANT-INTERNET" => Ok(Self::InstantInternet),
            "INTERCOM" => Ok(Self::INTERCOM),
            "INTERLISP" => Ok(Self::INTERLISP),
            "IOS" => Ok(Self::IOS),
            "IRIX" => Ok(Self::IRIX),
            "ISI-68020" => Ok(Self::ISI68020),
            "ITS" => Ok(Self::ITS),
            "KOSOS" => Ok(Self::KOSOS),
            "LINUX" => Ok(Self::Linux),
            "LINUX-1.0" => Ok(Self::Linux1_0),
            "LINUX-1.2" => Ok(Self::Linux1_2),
            "LINUX-2.0" => Ok(Self::Linux2_0),
            "LINUX-2.2" => Ok(Self::Linux2_2),
            "LINUX-2.4" => Ok(Self::Linux2_4),
            "LINUX-2.6" => Ok(Self::Linux2_6),
            "LISP" => Ok(Self::LISP),
            "LISPM" => Ok(Self::LISPM),
            "LOCUS" => Ok(Self::LOCUS),
            "MACOS" => Ok(Self::MacOS),
            "MINOS" => Ok(Self::MINOS),
            "MOS" => Ok(Self::MOS),
            "MPE/IX" => Ok(Self::MpeIx),
            "MPE/V" => Ok(Self::MpeV),
            "MPE5" => Ok(Self::Mpe5),
            "MSDOS" => Ok(Self::MsDOS),
            "MULTICS" => Ok(Self::Multics),
            "MUSIC" => Ok(Self::Music),
            "MUSIC/SP" => Ok(Self::MusicSP),
            "MVS" => Ok(Self::Mvs),
            "MVS/SP" => Ok(Self::MvsSP),
            "NETBSD-1.0" => Ok(Self::NetBSD1_0),
            "NETBSD-1.1" => Ok(Self::NetBSD1_1),
            "NETBSD-1.2" => Ok(Self::NetBSD1_2),
            "NETBSD-1.3" => Ok(Self::NetBSD1_3),
            "NETWARE-3" => Ok(Self::Netware3),
            "NETWARE-3.11" => Ok(Self::Netware3_11),
            "NETWARE-4.0" => Ok(Self::Netware4_0),
            "NETWARE-4.1" => Ok(Self::Netware4_1),
            "NETWARE-5.0" => Ok(Self::Netware5_0),
            "NEXUS" => Ok(Self::Nexus),
            "NMS" => Ok(Self::NMS),
            "NONSTOP" => Ok(Self::NonStop),
            "NOS-2" => Ok(Self::NOS2),
            "NTOS" => Ok(Self::NTOS),
            "OPENBSD" => Ok(Self::OpenBSD),
            "OPENVME" => Ok(Self::OpenVME),
            "OPENVMS" => Ok(Self::OpenVMS),
            "OS/2" => Ok(Self::Os2),
            "OS/DDP" => Ok(Self::OsDDP),
            "OS4" => Ok(Self::Os4),
            "OS86" => Ok(Self::Os86),
            "OSX" => Ok(Self::OSX),
            "PCDOS" => Ok(Self::PCDOS),
            "PERQ/OS" => Ok(Self::PerqOs),
            "PLI" => Ok(Self::PLI),
            "PRIMOS" => Ok(Self::PRIMOS),
            "PSDOS/MIT" => Ok(Self::PsdosMit),
            "PSOS" => Ok(Self::PSOS),
            "RISC-OS" => Ok(Self::RiscOS),
            "RISC-OS-3.10" => Ok(Self::RiscOS3_10),
            "RISC-OS-3.50" => Ok(Self::RiscOS3_50),
            "RISC-OS-3.60" => Ok(Self::RiscOS3_60),
            "RISC-OS-3.70" => Ok(Self::RiscOS3_70),
            "RISC-OS-4.00" => Ok(Self::RiscOS4_00),
            "RMX/RDOS" => Ok(Self::RmxRDOS),
            "ROS" => Ok(Self::ROS),
            "RSX11M" => Ok(Self::RSX11M),
            "RTE-A" => Ok(Self::RteA),
            "SATOPS" => Ok(Self::SATOPS),
            "SCO-OPEN-DESKTOP-1.0" => Ok(Self::ScoOpenDesktop1_0),
            "SCO-OPEN-DESKTOP-1.1" => Ok(Self::ScoOpenDesktop1_1),
            "SCO-OPEN-DESKTOP-2.0" => Ok(Self::ScoOpenDesktop2_0),
            "SCO-OPEN-DESKTOP-3.0" => Ok(Self::ScoOpenDesktop3_0),
            "SCO-OPEN-DESKTOP-LITE-3.0" => Ok(Self::ScoOpenDesktopLite3_0),
            "SCO-OPEN-SERVER-3.0" => Ok(Self::ScoOpenServer3_0),
            "SCO-UNIX-3.2.0" => Ok(Self::ScoUnix3_2_0),
            "SCO-UNIX-3.2V2.0" => Ok(Self::ScoUnix3_2V2_0),
            "SCO-UNIX-3.2V2.1" => Ok(Self::ScoUnix3_2V2_1),
            "SCO-UNIX-3.2V4.0" => Ok(Self::ScoUnix3_2V4_0),
            "SCO-UNIX-3.2V4.1" => Ok(Self::ScoUnix3_2V4_1),
            "SCO-UNIX-3.2V4.2" => Ok(Self::ScoUnix3_2V4_2),
            "SCO-XENIX-386-2.3.2" => Ok(Self::ScoXenix386_2_3_2),
            "SCO-XENIX-386-2.3.3" => Ok(Self::ScoXenix386_2_3_3),
            "SCO-XENIX-386-2.3.4" => Ok(Self::ScoXenix386_2_3_4),
            "SCS" => Ok(Self::SCS),
            "SIMP" => Ok(Self::Simp),
            "SINIX" => Ok(Self::Sinix),
            "SUN" => Ok(Self::Sun),
            "SUN-OS-3.5" => Ok(Self::SunOs3_5),
            "SUN-OS-4.0" => Ok(Self::SunOs4_0),
            "SWIFT" => Ok(Self::Swift),
            "TAC" => Ok(Self::TAC),
            "TANDEM" => Ok(Self::Tandem),
            "TENEX" => Ok(Self::Tenex),
            "THE-MAJOR-BBS" => Ok(Self::TheMajorBBS),
            "TOPS10" => Ok(Self::Tops10),
            "TOPS20" => Ok(Self::Tops20),
            "TOS" => Ok(Self::TOS),
            "TP3010" => Ok(Self::TP3010),
            "TRSDOS" => Ok(Self::TRSDOS),
            "ULTRIX" => Ok(Self::Ultrix),
            "UNIX" => Ok(Self::Unix),
            "UNIX-BSD" => Ok(Self::UnixBSD),
            "UNIX-PC" => Ok(Self::UnixPC),
            "UNIX-V" => Ok(Self::UnixV),
            "UNIX-V.1" => Ok(Self::UnixV1),
            "UNIX-V.2" => Ok(Self::UnixV2),
            "UNIX-V.3" => Ok(Self::UnixV3),
            "UNIX-V1AT" => Ok(Self::UnixV1AT),
            "UNKNOWN" => Ok(Self::Unknown),
            "UT2D" => Ok(Self::UT2D),
            "V" => Ok(Self::V),
            "VM" => Ok(Self::VM),
            "VM/370" => Ok(Self::VM370),
            "VM/CMS" => Ok(Self::VMCMS),
            "VM/SP" => Ok(Self::VMSP),
            "VMS" => Ok(Self::VMS),
            "VMS/EUNICE" => Ok(Self::VMSEunice),
            "VRTX" => Ok(Self::VRTX),
            "WAITS" => Ok(Self::Waits),
            "WANG" => Ok(Self::Wang),
            "WIN32" => Ok(Self::Win32),
            "WINDOWS-95" => Ok(Self::Windows95),
            "WINDOWS-95-OSR1" => Ok(Self::Windows95OSR1),
            "WINDOWS-95-OSR2" => Ok(Self::Windows95OSR2),
            "WINDOWS-98" => Ok(Self::Windows98),
            "WINDOWS-CE" => Ok(Self::WindowsCE),
            "WINDOWS-NT" => Ok(Self::WindowsNT),
            "WINDOWS-NT-2" => Ok(Self::WindowsNT2),
            "WINDOWS-NT-3" => Ok(Self::WindowsNT3),
            "WINDOWS-NT-3.5" => Ok(Self::WindowsNT3_5),
            "WINDOWS-NT-3.51" => Ok(Self::WindowsNT3_51),
            "WINDOWS-NT-4" => Ok(Self::WindowsNT4),
            "WINDOWS-NT-5" => Ok(Self::WindowsNT5),
            "WINDOWS-NT-5.1" => Ok(Self::WindowsNT5_1),
            "WINDOWS-NT-6" => Ok(Self::WindowsNT6),
            "WINDOWS-NT-6.1" => Ok(Self::WindowsNT6_1),
            "WORLDGROUP" => Ok(Self::WorldGroup),
            "WYSE-WYXWARE" => Ok(Self::WyseWyxware),
            "X11R3" => Ok(Self::X11R3),
            "XDE" => Ok(Self::XDE),
            "XENIX" => Ok(Self::Xenix),
            _ => Err(Self::Error::OsNameParseError),
        }
    }
}

impl Into<&'static str> for OperatingSystemName {
    fn into(self) -> &'static str {
        match self {
            OperatingSystemName::Aegis => "AEGIS",
            OperatingSystemName::AixPs2 => "AIX-PS/2",
            OperatingSystemName::Aix370 => "AIX/370",
            OperatingSystemName::AmigaOS1_2 => "AMIGA-OS-1.2",
            OperatingSystemName::AmigaOS1_3 => "AMIGA-OS-1.3",
            OperatingSystemName::AmigaOS2_0 => "AMIGA-OS-2.0",
            OperatingSystemName::AmigaOS2_1 => "AMIGA-OS-2.1",
            OperatingSystemName::AmigaOS3_0 => "AMIGA-OS-3.0",
            OperatingSystemName::AmigaOS3_1 => "AMIGA-OS-3.1",
            OperatingSystemName::AmigaOS3_5 => "AMIGA-OS-3.5",
            OperatingSystemName::AmigaOS3_9 => "AMIGA-OS-3.9",
            OperatingSystemName::Apollo => "APOLLO",
            OperatingSystemName::BeOS4_5_2 => "BEOS-4.5.2",
            OperatingSystemName::BeOS5_0 => "BEOS-5.0",
            OperatingSystemName::BS2000 => "BS-2000",
            OperatingSystemName::Cedar => "CEDAR",
            OperatingSystemName::CGW => "CGW",
            OperatingSystemName::Chorus => "CHORUS",
            OperatingSystemName::Chrysalis => "CHRYSALIS",
            OperatingSystemName::CMOS => "CMOS",
            OperatingSystemName::CMS => "CMS",
            OperatingSystemName::COS => "COS",
            OperatingSystemName::CPIX => "CPIX",
            OperatingSystemName::CTOS => "CTOS",
            OperatingSystemName::CTSS => "CTSS",
            OperatingSystemName::CYGWIN => "CYGWIN",
            OperatingSystemName::DCN => "DCN",
            OperatingSystemName::DDNOS => "DDNOS",
            OperatingSystemName::DOMAIN => "DOMAIN",
            OperatingSystemName::DOS => "DOS",
            OperatingSystemName::ECOS => "ECOS",
            OperatingSystemName::ECOSPRO => "ECOSPRO",
            OperatingSystemName::EDX => "EDX",
            OperatingSystemName::ELF => "ELF",
            OperatingSystemName::EMBOS => "EMBOS",
            OperatingSystemName::EMMOS => "EMMOS",
            OperatingSystemName::EPOS => "EPOS",
            OperatingSystemName::FOONEX => "FOONEX",
            OperatingSystemName::FORTH => "FORTH",
            OperatingSystemName::FREEBSD => "FREEBSD",
            OperatingSystemName::FUZZ => "FUZZ",
            OperatingSystemName::GCOS => "GCOS",
            OperatingSystemName::GPOS => "GPOS",
            OperatingSystemName::HDOS => "HDOS",
            OperatingSystemName::IMAGEN => "IMAGEN",
            OperatingSystemName::IMPRESS => "IMPRESS",
            OperatingSystemName::InstantInternet => "INSTANT-INTERNET",
            OperatingSystemName::INTERCOM => "INTERCOM",
            OperatingSystemName::INTERLISP => "INTERLISP",
            OperatingSystemName::IOS => "IOS",
            OperatingSystemName::IRIX => "IRIX",
            OperatingSystemName::ISI68020 => "ISI-68020",
            OperatingSystemName::ITS => "ITS",
            OperatingSystemName::KOSOS => "KOSOS",
            OperatingSystemName::Linux => "LINUX",
            OperatingSystemName::Linux1_0 => "LINUX-1.0",
            OperatingSystemName::Linux1_2 => "LINUX-1.2",
            OperatingSystemName::Linux2_0 => "LINUX-2.0",
            OperatingSystemName::Linux2_2 => "LINUX-2.2",
            OperatingSystemName::Linux2_4 => "LINUX-2.4",
            OperatingSystemName::Linux2_6 => "LINUX-2.6",
            OperatingSystemName::LISP => "LISP",
            OperatingSystemName::LISPM => "LISPM",
            OperatingSystemName::LOCUS => "LOCUS",
            OperatingSystemName::MacOS => "MACOS",
            OperatingSystemName::MINOS => "MINOS",
            OperatingSystemName::MOS => "MOS",
            OperatingSystemName::MpeIx => "MPE/IX",
            OperatingSystemName::MpeV => "MPE/V",
            OperatingSystemName::Mpe5 => "MPE5",
            OperatingSystemName::MsDOS => "MSDOS",
            OperatingSystemName::Multics => "MULTICS",
            OperatingSystemName::Music => "MUSIC",
            OperatingSystemName::MusicSP => "MUSIC/SP",
            OperatingSystemName::Mvs => "MVS",
            OperatingSystemName::MvsSP => "MVS/SP",
            OperatingSystemName::NetBSD1_0 => "NETBSD-1.0",
            OperatingSystemName::NetBSD1_1 => "NETBSD-1.1",
            OperatingSystemName::NetBSD1_2 => "NETBSD-1.2",
            OperatingSystemName::NetBSD1_3 => "NETBSD-1.3",
            OperatingSystemName::Netware3 => "NETWARE-3",
            OperatingSystemName::Netware3_11 => "NETWARE-3.11",
            OperatingSystemName::Netware4_0 => "NETWARE-4.0",
            OperatingSystemName::Netware4_1 => "NETWARE-4.1",
            OperatingSystemName::Netware5_0 => "NETWARE-5.0",
            OperatingSystemName::Nexus => "NEXUS",
            OperatingSystemName::NMS => "NMS",
            OperatingSystemName::NonStop => "NONSTOP",
            OperatingSystemName::NOS2 => "NOS-2",
            OperatingSystemName::NTOS => "NTOS",
            OperatingSystemName::OpenBSD => "OPENBSD",
            OperatingSystemName::OpenVME => "OPENVME",
            OperatingSystemName::OpenVMS => "OPENVMS",
            OperatingSystemName::Os2 => "OS/2",
            OperatingSystemName::OsDDP => "OS/DDP",
            OperatingSystemName::Os4 => "OS4",
            OperatingSystemName::Os86 => "OS86",
            OperatingSystemName::OSX => "OSX",
            OperatingSystemName::PCDOS => "PCDOS",
            OperatingSystemName::PerqOs => "PERQ/OS",
            OperatingSystemName::PLI => "PLI",
            OperatingSystemName::PRIMOS => "PRIMOS",
            OperatingSystemName::PsdosMit => "PSDOS/MIT",
            OperatingSystemName::PSOS => "PSOS",
            OperatingSystemName::RiscOS => "RISC-OS",
            OperatingSystemName::RiscOS3_10 => "RISC-OS-3.10",
            OperatingSystemName::RiscOS3_50 => "RISC-OS-3.50",
            OperatingSystemName::RiscOS3_60 => "RISC-OS-3.60",
            OperatingSystemName::RiscOS3_70 => "RISC-OS-3.70",
            OperatingSystemName::RiscOS4_00 => "RISC-OS-4.00",
            OperatingSystemName::RmxRDOS => "RMX/RDOS",
            OperatingSystemName::ROS => "ROS",
            OperatingSystemName::RSX11M => "RSX11M",
            OperatingSystemName::RteA => "RTE-A",
            OperatingSystemName::SATOPS => "SATOPS",
            OperatingSystemName::ScoOpenDesktop1_0 => "SCO-OPEN-DESKTOP-1.0",
            OperatingSystemName::ScoOpenDesktop1_1 => "SCO-OPEN-DESKTOP-1.1",
            OperatingSystemName::ScoOpenDesktop2_0 => "SCO-OPEN-DESKTOP-2.0",
            OperatingSystemName::ScoOpenDesktop3_0 => "SCO-OPEN-DESKTOP-3.0",
            OperatingSystemName::ScoOpenDesktopLite3_0 => "SCO-OPEN-DESKTOP-LITE-3.0",
            OperatingSystemName::ScoOpenServer3_0 => "SCO-OPEN-SERVER-3.0",
            OperatingSystemName::ScoUnix3_2_0 => "SCO-UNIX-3.2.0",
            OperatingSystemName::ScoUnix3_2V2_0 => "SCO-UNIX-3.2V2.0",
            OperatingSystemName::ScoUnix3_2V2_1 => "SCO-UNIX-3.2V2.1",
            OperatingSystemName::ScoUnix3_2V4_0 => "SCO-UNIX-3.2V4.0",
            OperatingSystemName::ScoUnix3_2V4_1 => "SCO-UNIX-3.2V4.1",
            OperatingSystemName::ScoUnix3_2V4_2 => "SCO-UNIX-3.2V4.2",
            OperatingSystemName::ScoXenix386_2_3_2 => "SCO-XENIX-386-2.3.2",
            OperatingSystemName::ScoXenix386_2_3_3 => "SCO-XENIX-386-2.3.3",
            OperatingSystemName::ScoXenix386_2_3_4 => "SCO-XENIX-386-2.3.4",
            OperatingSystemName::SCS => "SCS",
            OperatingSystemName::Simp => "SIMP",
            OperatingSystemName::Sinix => "SINIX",
            OperatingSystemName::Sun => "SUN",
            OperatingSystemName::SunOs3_5 => "SUN-OS-3.5",
            OperatingSystemName::SunOs4_0 => "SUN-OS-4.0",
            OperatingSystemName::Swift => "SWIFT",
            OperatingSystemName::TAC => "TAC",
            OperatingSystemName::Tandem => "TANDEM",
            OperatingSystemName::Tenex => "TENEX",
            OperatingSystemName::TheMajorBBS => "THE-MAJOR-BBS",
            OperatingSystemName::Tops10 => "TOPS10",
            OperatingSystemName::Tops20 => "TOPS20",
            OperatingSystemName::TOS => "TOS",
            OperatingSystemName::TP3010 => "TP3010",
            OperatingSystemName::TRSDOS => "TRSDOS",
            OperatingSystemName::Ultrix => "ULTRIX",
            OperatingSystemName::Unix => "UNIX",
            OperatingSystemName::UnixBSD => "UNIX-BSD",
            OperatingSystemName::UnixPC => "UNIX-PC",
            OperatingSystemName::UnixV => "UNIX-V",
            OperatingSystemName::UnixV1 => "UNIX-V.1",
            OperatingSystemName::UnixV2 => "UNIX-V.2",
            OperatingSystemName::UnixV3 => "UNIX-V.3",
            OperatingSystemName::UnixV1AT => "UNIX-V1AT",
            OperatingSystemName::Unknown => "UNKNOWN",
            OperatingSystemName::UT2D => "UT2D",
            OperatingSystemName::V => "V",
            OperatingSystemName::VM => "VM",
            OperatingSystemName::VM370 => "VM/370",
            OperatingSystemName::VMCMS => "VM/CMS",
            OperatingSystemName::VMSP => "VM/SP",
            OperatingSystemName::VMS => "VMS",
            OperatingSystemName::VMSEunice => "VMS/EUNICE",
            OperatingSystemName::VRTX => "VRTX",
            OperatingSystemName::Waits => "WAITS",
            OperatingSystemName::Wang => "WANG",
            OperatingSystemName::Win32 => "WIN32",
            OperatingSystemName::Windows95 => "WINDOWS-95",
            OperatingSystemName::Windows95OSR1 => "WINDOWS-95-OSR1",
            OperatingSystemName::Windows95OSR2 => "WINDOWS-95-OSR2",
            OperatingSystemName::Windows98 => "WINDOWS-98",
            OperatingSystemName::WindowsCE => "WINDOWS-CE",
            OperatingSystemName::WindowsNT => "WINDOWS-NT",
            OperatingSystemName::WindowsNT2 => "WINDOWS-NT-2",
            OperatingSystemName::WindowsNT3 => "WINDOWS-NT-3",
            OperatingSystemName::WindowsNT3_5 => "WINDOWS-NT-3.5",
            OperatingSystemName::WindowsNT3_51 => "WINDOWS-NT-3.51",
            OperatingSystemName::WindowsNT4 => "WINDOWS-NT-4",
            OperatingSystemName::WindowsNT5 => "WINDOWS-NT-5",
            OperatingSystemName::WindowsNT5_1 => "WINDOWS-NT-5.1",
            OperatingSystemName::WindowsNT6 => "WINDOWS-NT-6",
            OperatingSystemName::WindowsNT6_1 => "WINDOWS-NT-6.1",
            OperatingSystemName::WorldGroup => "WORLDGROUP",
            OperatingSystemName::WyseWyxware => "WYSE-WYXWARE",
            OperatingSystemName::X11R3 => "X11R3",
            OperatingSystemName::XDE => "XDE",
            OperatingSystemName::Xenix => "XENIX",
        }
    }
}
