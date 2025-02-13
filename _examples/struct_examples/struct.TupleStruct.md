# TupleStruct

```rust
pub struct TupleStruct(/* private field */, /* private field */)
```







## Trait Implementations

`impl<T> Borrow<T> for TupleStruct
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for TupleStruct
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for TupleStruct
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for TupleStruct`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for TupleStruct
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for TupleStruct
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for TupleStruct
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl Send for TupleStruct`

`impl Sync for TupleStruct`

`impl Freeze for TupleStruct`

`impl Unpin for TupleStruct`

`impl UnwindSafe for TupleStruct`

`impl RefUnwindSafe for TupleStruct`



