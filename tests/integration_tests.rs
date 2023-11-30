use enum_from::{EnumDisplay, EnumTryFrom};

#[test]
fn enum_color() {
    #[derive(Debug, PartialEq, EnumTryFrom, EnumDisplay)]
    #[repr(u8)]
    enum Color {
        White,
        Black,
    }

    let c = Color::try_from("White").unwrap();
    assert_eq!(c, Color::White);
    assert_eq!(&c.to_string(), "White");
    let c = Color::try_from("Foo").unwrap_err();
    assert_eq!(c, "Foo");

    let c = Color::try_from(1).unwrap();
    assert_eq!(c, Color::Black);
}

#[test]
fn enum_opcode() {
    #[derive(Debug, PartialEq, EnumTryFrom, EnumDisplay)]
    #[repr(u16)]
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

    let code = OpCode::try_from("Unassigned").unwrap();
    assert_eq!(code, OpCode::Unassigned);
    assert_eq!(&code.to_string(), "Unassigned");

    let code = OpCode::try_from("Foo").unwrap_err();
    assert_eq!(code, "Foo");

    let code = OpCode::try_from(6u16).unwrap();
    assert_eq!(code, OpCode::DOS);
    assert_eq!(&code.to_string(), "DOS");
}

#[test]
fn enum_choice() {
    #[derive(Debug, PartialEq, EnumTryFrom, EnumDisplay)]
    #[repr(u32)]
    enum Choice {
        Yes = 0 * 0,
        No = 1 * 10 / 10,
    }

    let c = Choice::try_from("Yes").unwrap();
    assert_eq!(c, Choice::Yes);

    let c = Choice::try_from(1u32).unwrap();
    assert_eq!(c, Choice::No);
}

#[test]
fn enum_answer() {
    #[derive(Debug, PartialEq, EnumTryFrom, EnumDisplay)]
    #[repr(u64)]
    enum Answer {
        A = 1000,
        B,
        C,
    }

    let a = Answer::try_from("C").unwrap();
    assert_eq!(a, Answer::C);

    let a = Answer::try_from(1002u64).unwrap();
    assert_eq!(a, Answer::C);
}
