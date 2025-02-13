# WikiStructure

```rust
pub enum WikiStructure {
	Directory,
	SingleFile,
}
```

Structure of the wiki

## Variants

- `Directory`

	Structure via directory. This will create multiple directories.
- `SingleFile`

	A Single file for the entire crate.




## Trait Implementations

`impl<T> Borrow<T> for WikiStructure
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for WikiStructure
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T> CloneToUninit for WikiStructure
where
	T: Clone`

```rust
unsafe fn clone_to_uninit(self: &Self, dst: *mut u8);
```

`impl<T, U> Into<U> for WikiStructure
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for WikiStructure`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for WikiStructure
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for WikiStructure
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for WikiStructure
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T> ToOwned for WikiStructure
where
	T: Clone`

```rust
fn to_owned(self: &Self) -> T;
```

```rust
fn clone_into(self: &Self, target: &mut T);
```

`impl Debug for WikiStructure`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl ValueEnum for WikiStructure`

```rust
fn value_variants<'a>() -> &'a [Self];
```

```rust
fn to_possible_value<'a>(self: &Self) -> Option<PossibleValue>;
```

`impl Clone for WikiStructure`

```rust
fn clone(self: &Self) -> WikiStructure;
```

`impl StructuralPartialEq for WikiStructure`

`impl PartialEq for WikiStructure`

```rust
fn eq(self: &Self, other: &WikiStructure) -> bool;
```



## Auto Trait Implementations

`impl Send for WikiStructure`

`impl Sync for WikiStructure`

`impl Freeze for WikiStructure`

`impl Unpin for WikiStructure`

`impl UnwindSafe for WikiStructure`

`impl RefUnwindSafe for WikiStructure`



