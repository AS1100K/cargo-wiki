# ModuleContent

```rust
pub struct ModuleContent<'a> {
	pub file_path: String,
	pub kind: ItemKind,
	pub title: &'a str,
	pub inner: Vec<InnerModuleContent>,
}
```



## Fields

- `file_path` : `String`
- `kind` : `ItemKind`
- `title` : `&'a str`
- `inner` : `Vec<InnerModuleContent>`


## Implementations

```rust
fn save_to_file(self: &Self) -> Result<()>;
```



## Trait Implementations

`impl<T> Borrow<T> for ModuleContent<'a>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for ModuleContent<'a>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T> CloneToUninit for ModuleContent<'a>
where
	T: Clone`

```rust
unsafe fn clone_to_uninit(self: &Self, dst: *mut u8);
```

`impl<T, U> Into<U> for ModuleContent<'a>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for ModuleContent<'a>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for ModuleContent<'a>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for ModuleContent<'a>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for ModuleContent<'a>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T> ToOwned for ModuleContent<'a>
where
	T: Clone`

```rust
fn to_owned(self: &Self) -> T;
```

```rust
fn clone_into(self: &Self, target: &mut T);
```

`impl<'a> Debug for ModuleContent<'a>`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl<'a> Clone for ModuleContent<'a>`

```rust
fn clone(self: &Self) -> ModuleContent<'a>;
```



## Auto Trait Implementations

`impl<'a> Send for ModuleContent<'a>`

`impl<'a> Sync for ModuleContent<'a>`

`impl<'a> Freeze for ModuleContent<'a>`

`impl<'a> Unpin for ModuleContent<'a>`

`impl<'a> UnwindSafe for ModuleContent<'a>`

`impl<'a> RefUnwindSafe for ModuleContent<'a>`



