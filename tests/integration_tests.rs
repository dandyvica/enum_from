use std::str::FromStr;

use enum_from::{EnumDisplay, EnumFromStr, EnumTryFrom};

// mod common;
// use enum_from::common::*;

#[test]
fn enum_opcode() {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr, EnumTryFrom)]
    #[repr(u16)]
    pub enum OpCode {
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
    assert_eq!(&code.to_string(), "Unassigned");

    let code = OpCode::try_from(6u16).unwrap();
    assert_eq!(code, OpCode::DOS);

    let code = OpCode::try_from(1000).unwrap_err();
    assert_eq!(code, 1000);
}

#[test]
fn enum_opcode_reserved() {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr, EnumTryFrom)]
    #[repr(u16)]
    pub enum OpCodeReserved {
        Query = 0,  //[RFC1035]
        IQuery = 1, // (Inverse Query, OBSOLETE)	[RFC3425]
        Status = 2, // [RFC1035]
        Unassigned = 3,
        Notify = 4, // [RFC1996]
        Update = 5, // [RFC2136]
        DOS = 6,    // DNS Stateful Operations (DSO)	[RFC8490]

        #[fallback]
        Reserved(u16),
    }

    // from_str
    let code = OpCodeReserved::from_str("Unassigned").unwrap();
    assert_eq!(code, OpCodeReserved::Unassigned);

    let code = OpCodeReserved::try_from(6u16).unwrap();
    assert_eq!(code, OpCodeReserved::DOS);
    assert_eq!(&code.to_string(), "DOS");

    let code = OpCodeReserved::try_from(1000).unwrap();
    assert_eq!(code, OpCodeReserved::Reserved(1000));
    assert_eq!(&code.to_string(), "Reserved1000");
}

#[test]
fn message() {
    #[derive(Debug, PartialEq, EnumDisplay, EnumFromStr)]
    #[repr(u8)]
    enum Message {
        Ok = 0,
        Quit = 1,
        Move { x: u16, y: u16 },
        Write(String),
        ChangeColor(u16, u16, u16),
    }

    // display
    let m = Message::Ok;
    assert_eq!(&m.to_string(), "Ok");
    let m = Message::Quit;
    assert_eq!(&m.to_string(), "Quit");
    let m = Message::Move { x: 1, y: 2 };
    assert_eq!(&m.to_string(), "12");
    let m = Message::Write("FOO".to_string());
    assert_eq!(&m.to_string(), "FOO");
    let m = Message::ChangeColor(1, 2, 3);
    assert_eq!(&m.to_string(), "123");

    // from_str
    let m = Message::from_str("Ok").unwrap();
    assert_eq!(m, Message::Ok);
    let m = Message::from_str("Quit").unwrap();
    assert_eq!(m, Message::Quit);
    assert!(Message::from_str("Move").is_err());
    assert!(Message::from_str("Write").is_err());
    assert!(Message::from_str("ChangeColor").is_err());

    // let code = OpCodeReserved::try_from(6u16).unwrap();
    // assert_eq!(code, OpCodeReserved::DOS);
    // assert_eq!(&code.to_string(), "DOS");

    // let code = OpCodeReserved::try_from(1000).unwrap();
    // assert_eq!(code, OpCodeReserved::Reserved(1000));
    // assert_eq!(&code.to_string(), "1000");
}
