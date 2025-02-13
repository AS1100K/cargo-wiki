# TypeGenerator

```rust
pub struct TypeGenerator;
```





## Implementations

```rust
fn type_to_string(type_: &Type) -> String;
```

```rust
fn path_to_string(path: &Path) -> String;
```



## Trait Implementations

`impl<T> Borrow<T> for TypeGenerator
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for TypeGenerator
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for TypeGenerator
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for TypeGenerator`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for TypeGenerator
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for TypeGenerator
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for TypeGenerator
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl Send for TypeGenerator`

`impl Sync for TypeGenerator`

`impl Freeze for TypeGenerator`

`impl Unpin for TypeGenerator`

`impl UnwindSafe for TypeGenerator`

`impl RefUnwindSafe for TypeGenerator`



