# self

## opcode
`self` uses opcodes to known which instructions build. opcodes can have different meaning in different environments, the important thing is the base level. this is the main level where you're not lookeading bytes. at this level these are the opcodes:

- `0x00`: zero
- `0x01`: load_const
  - `0x00`: nothing
  - `0x01`: u32
  - `0x02`: u64
  - `0x03`: i32
  - `0x04`: i64
- `0x02`: print
- `0x03`: add