use std::str::FromStr;

use enum_from::{EnumDisplay, EnumFromStr, EnumTryFrom};

// test with a standard C-like enum
#[test]
fn c_like_enum() {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr, EnumTryFrom)]
    #[repr(u16)]
    enum OpCode {
        Query = 0,  //[RFC1035]
        IQuery = 1, // (Inverse Query, OBSOLETE)	[RFC3425]
        Status = 2, // [RFC1035]
        Unassigned = 3,
        Notify = 4, // [RFC1996]
        Update = 5, // [RFC2136]
        DOS = 6,    // DNS Stateful Operations (DSO)	[RFC8490]
    }

    // from_str
    let code = OpCode::from_str("Unassigned").unwrap();
    assert_eq!(code, OpCode::Unassigned);
    let code = OpCode::from_str("foo").unwrap_err();
    assert_eq!(code, format!("no variant corresponding to value 'foo'"));

    // try_from
    let code = OpCode::try_from(6u16).unwrap();
    assert_eq!(code, OpCode::DOS);
    let code = OpCode::try_from(1000u16).unwrap_err();
    assert_eq!(code, 1000);

    // display
    let code = OpCode::from_str("Unassigned").unwrap();
    assert_eq!(&code.to_string(), "Unassigned");
    let code = OpCode::try_from(6u16).unwrap();
    assert_eq!(&code.to_string(), "DOS");
}

// test with a standard C-like enum with a fallback variant which captures other values
#[test]
fn c_like_enum_fallback() {
    #[derive(Debug, Default, PartialEq, EnumDisplay, EnumFromStr, EnumTryFrom)]
    #[repr(u16)]
    pub enum QClass {
        #[default]
        IN = 1, // the Internet
        CS = 2, // the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
        CH = 3, // the CHAOS class
        HS = 4, // Hesiod [Dyer 87]
        ANY = 255,

        #[fallback]
        CLASS(u16),
    }

    // from_str
    let code = QClass::from_str("IN").unwrap();
    assert_eq!(code, QClass::IN);
    let code = QClass::from_str("foo").unwrap_err();
    assert_eq!(code, format!("no variant corresponding to value 'foo'"));
    let code = QClass::from_str("CLASS1234").unwrap();
    assert_eq!(code, QClass::CLASS(1234));
    let code = QClass::from_str("CLASSA234").unwrap_err();
    assert_eq!(code, format!("no variant corresponding to value 'CLASSA234'"));

    // try_from
    let code = QClass::try_from(4u16).unwrap();
    assert_eq!(code, QClass::HS);
    let code = QClass::try_from(1000u16).unwrap();
    assert_eq!(code, QClass::CLASS(1000));

    // display
    let code = QClass::from_str("IN").unwrap();
    assert_eq!(&code.to_string(), "IN");
    let code = QClass::try_from(2u16).unwrap();
    assert_eq!(&code.to_string(), "CS");
    let code = QClass::try_from(1000u16).unwrap();
    assert_eq!(&code.to_string(), "CLASS1000");
    let code = QClass::from_str("CLASS1234").unwrap();
    assert_eq!(&code.to_string(), "CLASS1234");
}
