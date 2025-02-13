# Status

```rust
pub enum Status<T, E> {
	Success(0),
	Failure(0),
	Pending,
}
```

An enumeration for result status

## Variants

- `Success`

	Operation succeeded with value
- `Failure`

	Operation failed with error
- `Pending`

	Operation pending




## Trait Implementations

`impl<T> Borrow<T> for Status<T, E>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for Status<T, E>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for Status<T, E>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for Status<T, E>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for Status<T, E>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for Status<T, E>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for Status<T, E>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T: Debug, E: Debug> Debug for Status<T, E>`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```



## Auto Trait Implementations

`impl<T, E> Send for Status<T, E>
where
	T: Send,
	E: Send`

`impl<T, E> Sync for Status<T, E>
where
	T: Sync,
	E: Sync`

`impl<T, E> Freeze for Status<T, E>
where
	T: Freeze,
	E: Freeze`

`impl<T, E> Unpin for Status<T, E>
where
	T: Unpin,
	E: Unpin`

`impl<T, E> UnwindSafe for Status<T, E>
where
	T: UnwindSafe,
	E: UnwindSafe`

`impl<T, E> RefUnwindSafe for Status<T, E>
where
	T: RefUnwindSafe,
	E: RefUnwindSafe`



