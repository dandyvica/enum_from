# enum_from procedural macros
Straightforward procedural macros to auto-create implementations of ```Display``` , ```FromStr``` and ```TryFrom<u8/u16/...>``` for enums.

Just add ```#[derive(EnumTryFrom, EnumDisplay)]``` to the enum. From ```TryFrom``` ints, the proc-macro is looking at the ```#[repr()]``` attribute which is mandatory.

Supported repr sizes are ```u8/u16/u32/u64/i8/i16/i32/i64```. For the ```TryFrom``` method, if it fails, it returns the value wrapped as an ```Error``` or the value wrapped in the ```Reserved``` variant..

Example:

```rust
// DNS opcodes
#[derive(Debug, PartialEq, EnumTryFrom, EnumDisplay)]
#[repr(u8)]
pub enum OpCode {
    Query = 0, //[RFC1035]
    IQuery = 1, // (Inverse Query, OBSOLETE)	[RFC3425]
    Status = 2, // [RFC1035]
    Unassigned = 3,
    Notify = 4, // [RFC1996]
    Update = 5, // [RFC2136]
    DOS = 6,    // DNS Stateful Operations (DSO)	[RFC8490]
    
    // other values unassigned
    Reserved(u8),

}

// from_str
let code = OpCodeReserved::from_str("Unassigned").unwrap();
assert_eq!(code, OpCodeReserved::Unassigned);

let code = OpCodeReserved::try_from(6u16).unwrap();
assert_eq!(code, OpCodeReserved::DOS);
assert_eq!(&code.to_string(), "DOS");

let code = OpCodeReserved::try_from(1000).unwrap();
assert_eq!(code, OpCodeReserved::Reserved(1000));
assert_eq!(&code.to_string(), "1000");   
```

This is only possible for unit-only enums. Otherwise, compilation panics. 

