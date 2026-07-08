# namespace_basics.cpp

## InDepth: Why namespaces exist

Namespaces prevent name collisions across larger codebases.

In this example both modules need a `TaxCalculator`.  
Without namespaces, those class names would collide.

Pattern:
- group by domain (`billing`, `shipping`)
- use qualified names at call-site (`billing::TaxCalculator`)
