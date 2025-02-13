# PredicateStruct

```rust
pub struct PredicateStruct<'a, T, U>
where
	T: 'a + ?Sized,
	U: Default
{	pub reference: &'a T,
	pub value: U,
	/* private fields */
}
```



## Fields

- `reference` : `&'a T`
- `value` : `U`




## Trait Implementations

`impl<T> Borrow<T> for PredicateStruct<'a, T, U>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for PredicateStruct<'a, T, U>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for PredicateStruct<'a, T, U>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for PredicateStruct<'a, T, U>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for PredicateStruct<'a, T, U>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for PredicateStruct<'a, T, U>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for PredicateStruct<'a, T, U>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl<'a, T, U> Send for PredicateStruct<'a, T, U>
where
	U: Send,
	T: Sync + ?Sized`

`impl<'a, T, U> Sync for PredicateStruct<'a, T, U>
where
	U: Sync,
	T: Sync + ?Sized`

`impl<'a, T, U> Freeze for PredicateStruct<'a, T, U>
where
	U: Freeze,
	T: ?Sized`

`impl<'a, T, U> Unpin for PredicateStruct<'a, T, U>
where
	U: Unpin,
	T: ?Sized`

`impl<'a, T, U> UnwindSafe for PredicateStruct<'a, T, U>
where
	U: UnwindSafe,
	T: RefUnwindSafe + ?Sized`

`impl<'a, T, U> RefUnwindSafe for PredicateStruct<'a, T, U>
where
	U: RefUnwindSafe,
	T: RefUnwindSafe + ?Sized`



