# FunctionGenerator

```rust
pub struct FunctionGenerator;
```





## Implementations

```rust
fn generate_syntax(function: &Function, name: &str) -> Result<String>;
```



## Trait Implementations

`impl<T> Borrow<T> for FunctionGenerator
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for FunctionGenerator
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for FunctionGenerator
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for FunctionGenerator`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for FunctionGenerator
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for FunctionGenerator
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for FunctionGenerator
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl Send for FunctionGenerator`

`impl Sync for FunctionGenerator`

`impl Freeze for FunctionGenerator`

`impl Unpin for FunctionGenerator`

`impl UnwindSafe for FunctionGenerator`

`impl RefUnwindSafe for FunctionGenerator`



