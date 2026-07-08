# options_object_and_overloads.cpp

## InDepth: When to move from defaults to options object

Use default parameters for small optional sets.  
Move to an options object when:
- parameters become many
- booleans become confusing
- readability drops at call sites

Overloads can keep the simple path simple (`renderReport(name)`), while advanced callers pass explicit options.
