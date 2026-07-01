# Useless Interpreter
My own programming language. It runs line by line. Based off ASM.

## Syntax

- `<?TRASHCAN>` marks the start, similar to html files.
- To declear a variable do it in the format of `[1x]`, so in that example we're declearing a variable `x` with a value of `1`. Variables are typeless like JavaScript and mutable. Variables created with `[]` are dropped after a newline while `{}` are kepts until the end of the program. Values must be decleared variables to be used.
- Only binary numbers are support by default, where `.` is `0` and `,` is `1`, numbers are also written in reverse. For example `10010` will be `.,..,`.
- Operations are done like so: `1 + 1` will be `[1x][1y] x y donate`. See the table below for infos

Normal Operations|Useless Version
|--|--|
+| |
-|/
=|~
≠|-
