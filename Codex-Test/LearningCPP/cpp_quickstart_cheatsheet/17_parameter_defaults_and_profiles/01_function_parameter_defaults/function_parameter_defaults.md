# function_parameter_defaults.cpp

## InDepth: Good defaults reduce call-site noise

If most callers use the same values, make those values defaults.

Goal:
- keep required business inputs explicit
- hide optional knobs until needed
- avoid `""`, `0`, `false`, or `nullptr` placeholders
- compare `buildEmailMessage(...)` with `buildEmailMessageLegacyPlaceholderStyle(...)` to see how defaults remove fake call-site values

## InDepth: Default order

Put required parameters first, optional parameters with defaults after them.  
That keeps the function readable and predictable for new users.
