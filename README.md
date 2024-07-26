# js_to_oxc

\[WIP\] Convert JS source to Oxc AST builder in Rust.

**UI: https://KermanX.github.io/js_to_oxc**

Example:

```js
console.log('Hello, World!')
```

Will be converted to:

```rust
self.ast_builder.expression_call(
  SPAN,
  {
    let mut __arguments = self.ast_builder.vec();
    __arguments.push(
      self.ast_builder.argument_expression(
        self.ast_builder
            .expression_string_literal(SPAN, "Hello, World!"),
      ),
    );
    __arguments
  },
  self.ast_builder.expression_member(
    self.ast_builder.member_expression_static(
      SPAN,
      self.ast_builder
          .expression_identifier_reference(SPAN, "console"),
      "log",
      false,
    ),
  ),
  Option::<TSTypeParameterInstantiation>::None,
  false,
)
```
