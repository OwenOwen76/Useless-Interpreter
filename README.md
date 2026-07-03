# Useless Interpreter
My own programming language. It runs line by line. Based off ASM.

## Syntax

- `<?EMPTYTRASHCAN>` marks the start, the file end with `<!xthTRASHCAN>`, where `x` is the number of characters in the code, excluding `<!xthTRASHCAN>`.
- To declear a variable do it in the format of `[1x]`, so in that example we're declearing a variable `x` with a value of `1`. Variables are typeless like JavaScript and inmutable. Variables are decleared with `[]` and exist only while executing the line on which they are declared. Literal must be decleared variables to be used. Non-numeric values are single characters. Numbers may contain any number of binary digits. Variable names must be 10 alphabetical characters, from A-z and a-z. Variables may contain expressions.
- Only binary numbers are support by default, where `.` is `0` and `,` is `1`, numbers are also written in reverse. For example `10010` will be `.,..,`, only support numbers up to 1,000,000,000.
- Operations are done like so: `1 + 1` will be `[,xxxxxxxxxx][,yyyyyyyyyy]xxxxxxxxxxyyyyyyyyy|`. See the table below for infos.

Normal Operations|Useless Version
|--|--|
+|&#124;
-|/
==|~
≠|-
if|^
else|*

- If-Else is written like so: `:else block;*:if block;^:condition;`, example code: `:[,,xxxxxxxxxx];*:[,.xxxxxxxxxx];^:aaaaaaaaaabbbbbbbbbb~;`
- The interpreter read everything bottom up, right to left, so the last line's last character will be read first,and the top left character will be read last.
- No whitespaces at all.
- Return a the value of a variable with `"`, keep in mind this does not terminates the program.

### Example Code
The code below calculate `1 + 4 * 2 - 3`
```
<?EMPTYTRASHCAN>
"gggggggggg[ffffffffffdddddddddd/gggggggggg][eeeeeeeeeeaaaaaaaaaa|ffffffffff][bbbbbbbbbbbbbbbbbbbb|eeeeeeeeee][,,dddddddddd][.,cccccccccc][..,bbbbbbbbbb][,aaaaaaaaaa]
<!194thTRASHCAN>
```
