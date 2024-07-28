# js_to_oxc

Convert JS source to Oxc AST builder in Rust. Useful for injecting JS code when developing Rolldown plugins.

[**Online version**](https://KermanX.github.io/js_to_oxc)

### Example

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

The execution result of the generated Rust code is the AST of the given JS code.

This tool supports both expression mode and program mode.

### Motivation

I was porting a plugin from [Rollup](https://rollupjs.org) to [Rolldown](https://rolldown.rs). The plugin injects some **dynamic** JS code in the transform hook. In Rollup, the plugin simply uses string concatenation to inject the JS code. However, in Rolldown, the plugin needs to build the AST of the JS code - which is about 20 times longer than the implementation in Rollup. So, I decided to write this tool to convert the JS code to Oxc AST builder code.

Why not use a Rust macro which accepts JS and generates AST? Because the JS code is quite dynamic, and the macro will be too complex to write and maintain.

> [Here] is a Rolldown plugin that uses lots of code to generate a dynamic AST, which is a good example of the use case of this tool.
