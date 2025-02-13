# InnerModuleContent

```rust
pub struct InnerModuleContent {
	pub title: String,
	pub content: String,
}
```



## Fields

- `title` : `String`
- `content` : `String`


## Implementations

```rust
fn to_string(self: &Self) -> String;
```



## Trait Implementations

`impl<T> Borrow<T> for InnerModuleContent
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for InnerModuleContent
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T> CloneToUninit for InnerModuleContent
where
	T: Clone`

```rust
unsafe fn clone_to_uninit(self: &Self, dst: *mut u8);
```

`impl<T, U> Into<U> for InnerModuleContent
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for InnerModuleContent`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for InnerModuleContent
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for InnerModuleContent
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for InnerModuleContent
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T> ToOwned for InnerModuleContent
where
	T: Clone`

```rust
fn to_owned(self: &Self) -> T;
```

```rust
fn clone_into(self: &Self, target: &mut T);
```

`impl Debug for InnerModuleContent`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl Clone for InnerModuleContent`

```rust
fn clone(self: &Self) -> InnerModuleContent;
```



## Auto Trait Implementations

`impl Send for InnerModuleContent`

`impl Sync for InnerModuleContent`

`impl Freeze for InnerModuleContent`

`impl Unpin for InnerModuleContent`

`impl UnwindSafe for InnerModuleContent`

`impl RefUnwindSafe for InnerModuleContent`



