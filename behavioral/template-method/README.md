# Template Method

_**Template Method** is a default implementation of a trait method that defines a skeleton
of an algorithm, and other methods can be redefined in concrete types._

## Conceptual Example

A referenced example: [Template Method in Python](https://refactoring.guru/design-patterns/template-method/python/example)

```bash
cargo run --bin template-method
```

Output:

```
Same client code can work with different concrete implementations:
TemplateMethod says: I am doing the bulk of the work
ConcreteStruct1 says: Implemented Operation1
TemplateMethod says: But I let subclasses override some operations
ConcreteStruct1 says: Implemented Operation2
TemplateMethod says: But I am doing the bulk of the work anyway

Same client code can work with different concrete implementations:
TemplateMethod says: I am doing the bulk of the work
ConcreteStruct2 says: Implemented Operation1
TemplateMethod says: But I let subclasses override some operations
ConcreteStruct2 says: Implemented Operation2
TemplateMethod says: But I am doing the bulk of the work anyway
```
