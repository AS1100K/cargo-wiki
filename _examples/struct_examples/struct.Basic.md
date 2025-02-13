# Basic

```rust
pub struct Basic {
	pub type_: String,
	pub type: String,
}
```

This is a very basic struct

## Fields

- `type_` : `String`
- `type` : `String`




## Trait Implementations

`impl<T> Borrow<T> for Basic
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for Basic
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for Basic
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for Basic`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for Basic
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for Basic
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for Basic
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl Send for Basic`

`impl Sync for Basic`

`impl Freeze for Basic`

`impl Unpin for Basic`

`impl UnwindSafe for Basic`

`impl RefUnwindSafe for Basic`



