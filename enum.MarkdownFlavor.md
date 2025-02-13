# MarkdownFlavor

```rust
pub enum MarkdownFlavor {
	GitHub,
	GitLab,
}
```

Markdown Flavor to be used

## Variants

- `GitHub`

	GitHub Markdown Flavor
- `GitLab`

	GitLab Markdown Flavor




## Trait Implementations

`impl<T> Borrow<T> for MarkdownFlavor
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for MarkdownFlavor
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T> CloneToUninit for MarkdownFlavor
where
	T: Clone`

```rust
unsafe fn clone_to_uninit(self: &Self, dst: *mut u8);
```

`impl<T, U> Into<U> for MarkdownFlavor
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for MarkdownFlavor`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for MarkdownFlavor
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for MarkdownFlavor
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for MarkdownFlavor
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T> ToOwned for MarkdownFlavor
where
	T: Clone`

```rust
fn to_owned(self: &Self) -> T;
```

```rust
fn clone_into(self: &Self, target: &mut T);
```

`impl Debug for MarkdownFlavor`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl ValueEnum for MarkdownFlavor`

```rust
fn value_variants<'a>() -> &'a [Self];
```

```rust
fn to_possible_value<'a>(self: &Self) -> Option<PossibleValue>;
```

`impl Clone for MarkdownFlavor`

```rust
fn clone(self: &Self) -> MarkdownFlavor;
```

`impl StructuralPartialEq for MarkdownFlavor`

`impl PartialEq for MarkdownFlavor`

```rust
fn eq(self: &Self, other: &MarkdownFlavor) -> bool;
```



## Auto Trait Implementations

`impl Send for MarkdownFlavor`

`impl Sync for MarkdownFlavor`

`impl Freeze for MarkdownFlavor`

`impl Unpin for MarkdownFlavor`

`impl UnwindSafe for MarkdownFlavor`

`impl RefUnwindSafe for MarkdownFlavor`



