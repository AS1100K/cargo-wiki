# VisibilityGenerator

```rust
pub struct VisibilityGenerator;
```





## Implementations

```rust
fn generate_visibility(visibility: &Visibility) -> String;
```



## Trait Implementations

`impl<T> Borrow<T> for VisibilityGenerator
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for VisibilityGenerator
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for VisibilityGenerator
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for VisibilityGenerator`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for VisibilityGenerator
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for VisibilityGenerator
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for VisibilityGenerator
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl Send for VisibilityGenerator`

`impl Sync for VisibilityGenerator`

`impl Freeze for VisibilityGenerator`

`impl Unpin for VisibilityGenerator`

`impl UnwindSafe for VisibilityGenerator`

`impl RefUnwindSafe for VisibilityGenerator`



