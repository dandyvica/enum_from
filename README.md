# enum_from procedural macros
Straightforward procedural macros to auto-create implementations of ```Display``` and ```TryFrom<&str>``` and ```TryFrom<u8/u16/...>``` for [unit-only enums](https://doc.rust-lang.org/reference/items/enumerations.html) 

```rust
```
Just add ```#[derive(EnumTryFrom, EnumDisplay)]``` to the enum. From ```TryFrom``` ints, the proc-macro is looking at the ```#[repr()]``` attribute which is mandatory.

Supported repr sizes are ```u8/u16/u32/u64/i8/i16/i32/i64```. For the ```TryFrom``` method, if it fails, it returns the value wrapped as an ```Error```.

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
                // 7-15 Unassigned
}

let code = OpCode::try_from("Unassigned").unwrap();
assert_eq!(code, OpCode::Unassigned);

let code = OpCode::try_from("Foo").unwrap_err();
assert_eq!(code, "Foo");

let code = OpCode::try_from(6).unwrap();
assert_eq!(code, OpCode::DOS);    
```

This is only possible for unit-only enums. Otherwise, compilation panics. 

