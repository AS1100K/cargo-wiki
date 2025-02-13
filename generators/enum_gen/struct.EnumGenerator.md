# EnumGenerator

```rust
pub struct EnumGenerator;
```







## Trait Implementations

`impl<T> Borrow<T> for EnumGenerator
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for EnumGenerator
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for EnumGenerator
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for EnumGenerator`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for EnumGenerator
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for EnumGenerator
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for EnumGenerator
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl Generator for EnumGenerator`

```rust
fn generate_page(item: &Item, index: &Index, paths: &Paths, external_crates: &ExternalCrates) -> Result<Vec<InnerModuleContent>>;
```



## Auto Trait Implementations

`impl Send for EnumGenerator`

`impl Sync for EnumGenerator`

`impl Freeze for EnumGenerator`

`impl Unpin for EnumGenerator`

`impl UnwindSafe for EnumGenerator`

`impl RefUnwindSafe for EnumGenerator`



