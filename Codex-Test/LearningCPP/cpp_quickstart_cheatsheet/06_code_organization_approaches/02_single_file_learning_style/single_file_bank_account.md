# Single-File Style (Used In This Package)

Pattern:

```cpp
// one .cpp contains class + methods + optional demo main
class BankAccount { ... };
```

Upsides:
- Fast to read and scan
- Great for learning and focused examples
- Minimal build complexity

Downsides:
- Weaker separation between API and implementation
- Becomes noisy in larger systems
- Harder to scale to many developers/files

Use this for learning, experiments, and tiny tools.

## InDepths

- Single-file organization is ideal for teaching and spikes because context stays in one place.
- As behavior grows, extract into headers/sources before the file becomes a bottleneck.
- Treat this style as a stepping stone, not a permanent architecture for large systems.

