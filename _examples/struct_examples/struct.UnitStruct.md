# UnitStruct

```rust
pub struct UnitStruct;
```







## Trait Implementations

`impl<T> Borrow<T> for UnitStruct
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for UnitStruct
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for UnitStruct
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for UnitStruct`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for UnitStruct
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for UnitStruct
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for UnitStruct
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl Send for UnitStruct`

`impl Sync for UnitStruct`

`impl Freeze for UnitStruct`

`impl Unpin for UnitStruct`

`impl UnwindSafe for UnitStruct`

`impl RefUnwindSafe for UnitStruct`



