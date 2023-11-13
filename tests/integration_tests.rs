use enum_from::{EnumFromStr, EnumTryFrom, EnumDisplay};
use std::str::FromStr;

#[test]
fn enum_color() {
    #[derive(Debug, PartialEq, EnumFromStr, EnumTryFrom)]
    enum Color {
        White,
        Black,
    }

    let c = Color::from_str("White").unwrap();
    assert_eq!(c, Color::White);

    let c = Color::try_from(1).unwrap();
    assert_eq!(c, Color::Black);    
}

#[test]
fn enum_opcode() {
    #[derive(Debug, PartialEq, EnumFromStr, EnumTryFrom, EnumDisplay)]
    #[repr(u8)]
    pub enum OpCode {
        Query = 0,  //[RFC1035]
        IQuery = 1, // (Inverse Query, OBSOLETE)	[RFC3425]
        Status = 2, // [RFC1035]
        Unassigned = 3,
        Notify = 4, // [RFC1996]
        Update = 5, // [RFC2136]
        DOS = 6,    // DNS Stateful Operations (DSO)	[RFC8490]
                    // 7-15 Unassigned
    }

    let code = OpCode::from_str("Unassigned").unwrap();
    assert_eq!(code, OpCode::Unassigned);
    assert_eq!(&code.to_string(), "Unassigned");

    let code = OpCode::from_str("Foo");
    assert!(code.is_err());

    let code = OpCode::try_from(6).unwrap();
    assert_eq!(code, OpCode::DOS); 
    assert_eq!(&code.to_string(), "DOS");

    let code = OpCode::try_from(u64::MAX);
    assert!(code.is_err());
   
}

#[test]
fn enum_choice() {
    #[derive(Debug, PartialEq, EnumFromStr, EnumTryFrom)]
    enum Choice {
        Yes = 0*0,
        No = 1*10/10,
    }

    let c = Choice::from_str("Yes").unwrap();
    assert_eq!(c, Choice::Yes);

    let c = Choice::try_from(1).unwrap();
    assert_eq!(c, Choice::No);    
}

#[test]
fn enum_answer() {
    #[derive(Debug, PartialEq, EnumFromStr, EnumTryFrom)]
    #[repr(u16)]
    enum Answer {
        A = 1000,
        B,
        C,
    }

    let a = Answer::from_str("C").unwrap();
    assert_eq!(a, Answer::C);

    let a = Answer::try_from(1002).unwrap();
    assert_eq!(a, Answer::C);     
}

// #[test]
// fn enum_message() {
//     #[derive(Debug, PartialEq, FromStr)]
//     enum Message {
//         Move { x: u16, y: u16 },
//         Write(String),
//         ChangeColor(u16, u16, u16),
//     }
// }
