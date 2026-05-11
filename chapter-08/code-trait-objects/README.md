# Trait Dispatch Tradeoffs

This example compares a few ways to call different `LLMProvider`
implementations in Rust.

## Generics

Generics are best when one concrete type is chosen at compile time.

```rust
fn send_prompt<P: LLMProvider>(provider: P, prompt: String) -> String {
    provider.send_request(prompt)
}
```

Here, `P` might be `Claude`, `Gemini`, or `OpenAI`, but each call to
`send_prompt` is compiled for one concrete type.

The generic function is not defined on the trait. The method `send_request` is
defined by the trait. The generic function just says: accept any concrete type
`P`, as long as `P` implements `LLMProvider`.

For example:

```rust
call_provider(Claude {});
call_provider(Gemini {});
```

Rust can compile those like separate concrete functions:

```rust
fn call_provider_for_claude(provider: Claude) {
    provider.send_request("barbaz".to_string());
}

fn call_provider_for_gemini(provider: Gemini) {
    provider.send_request("barbaz".to_string());
}
```

That is static dispatch: Rust knows the concrete type at compile time and can
call the correct implementation directly.

This is type-safe and fast, but it does not let you put different provider
types into the same `Vec<P>`.

## Trait Objects

Trait objects are best when you want many implementations behind one shared
runtime interface.

```rust
let providers: Vec<Box<dyn LLMProvider>> = vec![
    Box::new(Claude {}),
    Box::new(Gemini {}),
    Box::new(OpenAI {}),
];
```

This lets one collection hold different concrete provider types. Rust checks
that every value implements `LLMProvider`, then uses dynamic dispatch at
runtime to call the right implementation.

This is still type-safe. The tradeoff is an extra level of indirection through
a trait object.

### Downsides

Trait objects trade compile-time specialization for runtime flexibility.

- Method calls use dynamic dispatch through a vtable, so the compiler usually
  cannot inline and specialize them as directly as generic calls.
- You only know the value through the trait interface, so you can only call
  methods defined on `LLMProvider`, not methods specific to `Claude`, `Gemini`,
  or `OpenAI`.
- The trait must be dyn compatible.
- A trait object is a fat pointer: a data pointer plus a vtable pointer.
- Heap allocation is not required by `dyn Trait` itself, but it is common when
  using owned heterogeneous collections like `Vec<Box<dyn LLMProvider>>`.

The trait also has to be dyn compatible. For this example, that means
`send_request` must be a method on a value:

```rust
trait LLMProvider {
    fn send_request(&self, prompt: String) -> String;
}
```

This version can be called through a trait object:

```rust
provider.send_request("barbaz".to_string())
```

This version is not dyn compatible:

```rust
trait LLMProvider {
    fn send_request(prompt: String) -> String;
}
```

That defines an associated function on the type, such as
`Claude::send_request(...)`, not a method on a provider value. A trait object
needs methods that can be called through a value, because the dynamic dispatch
happens through that value's vtable.

## Enum And Match

An enum is best when the set of providers is fixed and you want every case to
be explicit.

```rust
enum Provider {
    Claude(Claude),
    Gemini(Gemini),
    OpenAI(OpenAI),
}
```

Then you can dispatch manually:

```rust
match provider {
    Provider::Claude(provider) => provider.send_request(prompt),
    Provider::Gemini(provider) => provider.send_request(prompt),
    Provider::OpenAI(provider) => provider.send_request(prompt),
}
```

This is also type-safe, and the compiler can force you to handle every
provider variant. The tradeoff is that adding a new provider means updating
the enum and every relevant `match`.

## String Match

The current `generics_dyn_dispatch.rs` uses a string match:

```rust
match *provider_name {
    "claude" => Claude {}.send_request("barbaz".to_string()),
    "gemini" => Gemini {}.send_request("barbaz".to_string()),
    "openai" => OpenAI {}.send_request("barbaz".to_string()),
    _ => unreachable!(),
}
```

This is manual runtime dispatch. There is no trait object or vtable here; the
`match` is the dispatch table.

This is fine for a small tutorial example, but it is weaker than an enum
because strings can be misspelled and the compiler cannot know all possible
string values.

## Summary

- Use generics when one concrete type is selected at compile time.
- Use `Box<dyn LLMProvider>` when you need a mixed runtime collection.
- Use an enum and `match` when the provider set is closed and explicit.
- Avoid string matching for real provider dispatch unless the strings come
  from outside the program and must be parsed.

## Comparison With Go

Go interfaces are structurally satisfied. A type does not need to explicitly
say that it implements an interface. It only needs to have the methods required
by that interface.

```go
type LLMProvider interface {
    SendRequest(prompt string) string
}

func CallProvider(provider LLMProvider) {
    fmt.Println(provider.SendRequest("barbaz"))
}
```

This is not like Rust generic static dispatch. A normal Go interface parameter
is dynamic dispatch. The interface value carries runtime information about the
concrete value and its method table, and the method call is dispatched through
that interface.

The closest Rust equivalent is a trait object:

```rust
fn call_provider(provider: Box<dyn LLMProvider>) {
    println!("{}", provider.send_request("barbaz".to_string()));
}
```

Go also has generics:

```go
func CallProvider[P LLMProvider](provider P) {
    fmt.Println(provider.SendRequest("barbaz"))
}
```

This is syntactically closer to Rust generics:

```rust
fn call_provider<P: LLMProvider>(provider: P) {
    println!("{}", provider.send_request("barbaz".to_string()));
}
```

But Go's generics implementation is not the same as Rust's monomorphization
model. For this tutorial, the main distinction is:

- Rust `fn call_provider<P: LLMProvider>(provider: P)` is the static dispatch
  example.
- Rust `Box<dyn LLMProvider>` is dynamic dispatch through a trait object.
- Go `func CallProvider(provider LLMProvider)` is dynamic dispatch through an
  interface.
- Go `func CallProvider[P LLMProvider](provider P)` is the closest Go shape to
  the Rust generic example.
- Rust `match` on strings is manual runtime dispatch; the Go equivalent would
  be a `switch` that chooses which concrete implementation to call.
