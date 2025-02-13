# Direction

```rust
pub enum Direction {
	North,
	South,
	East,
	West,
}
```

An enumeration for directions

## Variants

- `North`

	Points North
- `South`

	Points South
- `East`

	Points East
- `West`

	Points West




## Trait Implementations

`impl<T> Borrow<T> for Direction
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for Direction
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for Direction
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for Direction`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for Direction
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for Direction
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for Direction
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl Debug for Direction`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl StructuralPartialEq for Direction`

`impl PartialEq for Direction`

```rust
fn eq(self: &Self, other: &Direction) -> bool;
```



## Auto Trait Implementations

`impl Send for Direction`

`impl Sync for Direction`

`impl Freeze for Direction`

`impl Unpin for Direction`

`impl UnwindSafe for Direction`

`impl RefUnwindSafe for Direction`



