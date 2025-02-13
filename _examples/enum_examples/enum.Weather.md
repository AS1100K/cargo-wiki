# Weather

```rust
pub enum Weather {
	Sunny(0),
	Rainy(0),
	Cloudy,
	Snowy(0),
}
```

An enumeration representing different types of weather

## Variants

- `Sunny`

	Sunny weather with optional temperature
- `Rainy`

	Rainy weather with precipitation amount
- `Cloudy`

	Cloudy weather
- `Snowy`

	Snowy weather with snowfall amount




## Trait Implementations

`impl<T> Borrow<T> for Weather
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for Weather
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for Weather
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for Weather`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for Weather
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for Weather
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for Weather
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl Debug for Weather`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```



## Auto Trait Implementations

`impl Send for Weather`

`impl Sync for Weather`

`impl Freeze for Weather`

`impl Unpin for Weather`

`impl UnwindSafe for Weather`

`impl RefUnwindSafe for Weather`



