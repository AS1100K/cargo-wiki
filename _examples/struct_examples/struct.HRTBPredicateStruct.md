# HRTBPredicateStruct

```rust
pub struct HRTBPredicateStruct<'a, T, U, F>
where
	for<'b> T: 'b + ?Sized,
	U: Default,
	F: FnFn(&'a T) -> U
{	pub reference: &'a T,
	pub value: U,
	pub transformer: F,
	/* private fields */
}
```

Uses Higher-Rank Trait Bounds (HRTBs)

## Fields

- `reference` : `&'a T`
- `value` : `U`
- `transformer` : `F`




## Trait Implementations

`impl<T> Borrow<T> for HRTBPredicateStruct<'a, T, U, F>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for HRTBPredicateStruct<'a, T, U, F>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for HRTBPredicateStruct<'a, T, U, F>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for HRTBPredicateStruct<'a, T, U, F>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for HRTBPredicateStruct<'a, T, U, F>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for HRTBPredicateStruct<'a, T, U, F>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for HRTBPredicateStruct<'a, T, U, F>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl<'a, T, U, F> Send for HRTBPredicateStruct<'a, T, U, F>
where
	U: Send,
	F: Send,
	T: Sync + ?Sized`

`impl<'a, T, U, F> Sync for HRTBPredicateStruct<'a, T, U, F>
where
	U: Sync,
	F: Sync,
	T: Sync + ?Sized`

`impl<'a, T, U, F> Freeze for HRTBPredicateStruct<'a, T, U, F>
where
	U: Freeze,
	F: Freeze,
	T: ?Sized`

`impl<'a, T, U, F> Unpin for HRTBPredicateStruct<'a, T, U, F>
where
	U: Unpin,
	F: Unpin,
	T: ?Sized`

`impl<'a, T, U, F> UnwindSafe for HRTBPredicateStruct<'a, T, U, F>
where
	U: UnwindSafe,
	F: UnwindSafe,
	T: RefUnwindSafe + ?Sized`

`impl<'a, T, U, F> RefUnwindSafe for HRTBPredicateStruct<'a, T, U, F>
where
	U: RefUnwindSafe,
	F: RefUnwindSafe,
	T: RefUnwindSafe + ?Sized`



