# Generics vs Templates (Why Separate Folder)

In C++ terms, the language feature is **templates**.

In Java/C# terms, the feature is called **generics**.

This folder is about the *generic programming idea* without template syntax: one API that handles multiple concrete types through a common contract.

```cpp
class ITextConvertible {
public:
    virtual string toText() const = 0;
};
```

Then different types implement the contract, and one function works across them.

## InDepths

- This style is runtime polymorphism (interface-based), not C++ template metaprogramming.
- It is useful for plugin-style behavior and late binding.
- Template-based generic code is covered separately in `15_templates`.

