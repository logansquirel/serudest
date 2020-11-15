# Serudest

Human readable serialization format.

## Format

### Notation

#### Comment

> // Comment

#### Token

> token

#### Optional `?`

> token?

#### One or more

> token+

#### Zero or more

> token*

#### Character

> `a`

#### String

> `string`

#### Or 

> token | other

#### Range

> [ - ]

#### Group

> ( )

### Boolean

> boolean = `true` | `false`

### Integers

#### Unsigned

> integer = `0` | ([`1`-`9`] digits*)  
> digits = [`0`-`9`]


#### Signed 

> integer = sign? (`0` | ([`1`-`9`] digits*))  
> sign = `-` | `+`  
> digits = [`0`-`9`]

### Floats

> float = sign? mantissa fraction? exponent?  
> sign = `+` | `-`  
> mantissa = `0` | ([`1`-`9`] digits*)  
> fraction = `.` digits*  
> exponent = (`e`|`E`) sign? digits+  
> digits = [`0`-`9`]

### Char

> character = `'` char `'`  
> char = [UTF-8]

### String

> string = `"` str `"`  
> str = [UTF-8]*

### Option

#### None

> none = `none`

#### Some

> some = `some(` value `)`

### Unit

> unit = `()`

### Tuple

> tuple = `(` (elem `,`)* `)`

### Array/Sequence

> sequence = `[` (elem `,`)* `]`


### Map/Dictionnary

> map = `{` (key `:` value `,`)* `}`


### Unit struct

> structure = struct_name

### Tuple struct

> structure = struct_name `(` (value `,`)* `)`

### Struct

> structure = struct_name `{` (field `=` value `,`)* `}`

### Enum unit variant
 
> variant = enum_name `::` variant_name

### Enum tuple variant

> variant = enum_name `::` variant_name `(` (value `,`)* `)`

### Enum struct variant

> variant = enum_name `::` variant_name `{` (field `=` value `,`)* `}`
