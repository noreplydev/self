# self

## opcode
`self` uses opcodes to known which instructions build. opcodes can have different meaning in different environments, the important thing is the base level. this is the main level where you're not lookeading bytes. at this level these are the opcodes:

- `0x00`: zero
- `0x01`: load_const
  - `0x01`: int64
- `0x02`: print
- `0x03`: add