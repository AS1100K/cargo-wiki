# TrafficLight

```rust
pub enum TrafficLight {
	Red,
	Yellow,
	Green,
}
```

An enumeration for traffic light states

## Variants

- `Red`

	Red light - Stop
- `Yellow`

	Yellow light - Caution
- `Green`

	Green light - Go




## Trait Implementations

`impl<T> Borrow<T> for TrafficLight
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for TrafficLight
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for TrafficLight
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for TrafficLight`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for TrafficLight
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for TrafficLight
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for TrafficLight
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl Debug for TrafficLight`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```



## Auto Trait Implementations

`impl Send for TrafficLight`

`impl Sync for TrafficLight`

`impl Freeze for TrafficLight`

`impl Unpin for TrafficLight`

`impl UnwindSafe for TrafficLight`

`impl RefUnwindSafe for TrafficLight`



