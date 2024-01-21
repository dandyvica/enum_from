# _enum_from_ procedural macros
Straightforward procedural macros to auto-create implementations of ```Display``` , ```FromStr``` and ```TryFrom<u8/u16/...>``` for enums.

Just add ```#[derive(EnumTryFrom, EnumDisplay)]``` to the enum. From ```TryFrom``` ints, the proc-macro is looking at the ```#[repr()]``` attribute which is mandatory.

Supported repr sizes are ```u8/u16/u32/u64/i8/i16/i32/i64```. For the ```TryFrom``` method, if it fails, it returns the value wrapped as an ```Error``` or the value wrapped in the ```Reserved``` variant if such a variant is existing.

## C-like enums
For pure C-like enums:
```rust
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
```

## For unit having a fallback variant
```rust
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
```

For other kind of enums, compilation panics.

