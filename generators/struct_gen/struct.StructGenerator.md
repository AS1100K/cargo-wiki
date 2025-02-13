# StructGenerator

```rust
pub struct StructGenerator;
```







## Trait Implementations

`impl<T> Borrow<T> for StructGenerator
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for StructGenerator
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for StructGenerator
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for StructGenerator`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for StructGenerator
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for StructGenerator
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for StructGenerator
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl Generator for StructGenerator`

```rust
/// Generate Code Syntax for `Struct`. See [Template.html](https://github.com/AS1100K/cargo-wiki/blob/main/Template.md#struct-syntax-block)
/// for more info.
fn generate_page(item: &Item, index: &Index, paths: &Paths, external_crates: &ExternalCrates) -> Result<Vec<InnerModuleContent>>;
```



## Auto Trait Implementations

`impl Send for StructGenerator`

`impl Sync for StructGenerator`

`impl Freeze for StructGenerator`

`impl Unpin for StructGenerator`

`impl UnwindSafe for StructGenerator`

`impl RefUnwindSafe for StructGenerator`



