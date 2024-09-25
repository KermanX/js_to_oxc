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
  self.ast_builder.vec1(
    self.ast_builder.argument_expression(
      self.ast_builder
        .expression_string_literal(SPAN, "Hello, World!"),
    ),
  ),
  self.ast_builder.expression_member(
    self.ast_builder.member_expression_static(
      SPAN,
      self.ast_builder
        .expression_identifier_reference(SPAN, "console"),
      self.ast_builder.identifier_name(SPAN, "log"),
      false,
    ),
  ),
  NONE,
  false,
)
```

The execution result of the generated Rust code is the AST of the given JS code.

This tool supports both expression mode and program mode.

### Holes

This tool supports holes in the JS code. Identifier starting with `$` will be treated as a hole. For example:

```js
log($1)
```

Will be converted to:

```rust
self.ast_builder.expression_call(
  SPAN,
  self.ast_builder.vec1(
    self.ast_builder.argument_expression(
      __1__  // hole
    ),
  ),
  self.ast_builder
    .expression_identifier_reference(SPAN, "log"),
  NONE,
  false,
)
```

### Motivation

I was porting a plugin from [Rollup](https://rollupjs.org) to [Rolldown](https://rolldown.rs). The plugin injects some **dynamic** JS code in the transform hook. In Rollup, the plugin simply uses string concatenation to inject the JS code. However, in Rolldown, the plugin needs to build the AST of the JS code - which is about 20 times longer than the implementation in Rollup. So, I decided to write this tool to convert the JS code to Oxc AST builder code.

Why not use a Rust macro which accepts JS and generates AST? Because the JS code is quite dynamic, and the macro will be too complex to write and maintain.

> [Here](https://github.com/rolldown/rolldown/blob/main/crates/rolldown_plugin_glob_import/src/lib.rs#L150-L306) is a Rolldown plugin that uses lots of code to generate a dynamic AST, which is a good example of the use case of this tool.
