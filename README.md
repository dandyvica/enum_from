# enum_from procedural macros
Straightforward procedural macros to create [unit-only enums](https://doc.rust-lang.org/reference/items/enumerations.html) from either a string or u64.
Just add ```#[derive(EnumFromStr, EnumTryFrom)]``` to the enum.

Example:

```rust
// DNS opcodes
#[derive(Debug, PartialEq, EnumFromStr, EnumTryFrom)]
#[repr(u8)]
pub enum OpCode {
    Query = 0, //[RFC1035]
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

let code = OpCode::from_str("Foo");
assert!(code.is_err());

let code = OpCode::try_from(6).unwrap();
assert_eq!(code, OpCode::DOS);    

let code = OpCode::try_from(u64::MAX);
assert!(code.is_err());
```

This is only possible for unit-only enums. Otherwise, compilation panics. 

> Note: Only ```TryFrom<u64>``` is defined.