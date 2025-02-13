# ModuleField

```rust
pub struct ModuleField<'a> {
	pub name: &'a str,
	pub link: String,
	pub description: &'a str,
}
```



## Fields

- `name` : `&'a str`
- `link` : `String`
- `description` : `&'a str`


## Implementations

```rust
fn to_string(self: &Self) -> String;
```



## Trait Implementations

`impl<T> Borrow<T> for ModuleField<'a>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for ModuleField<'a>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T> CloneToUninit for ModuleField<'a>
where
	T: Clone`

```rust
unsafe fn clone_to_uninit(self: &Self, dst: *mut u8);
```

`impl<T, U> Into<U> for ModuleField<'a>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for ModuleField<'a>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for ModuleField<'a>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for ModuleField<'a>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for ModuleField<'a>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T> ToOwned for ModuleField<'a>
where
	T: Clone`

```rust
fn to_owned(self: &Self) -> T;
```

```rust
fn clone_into(self: &Self, target: &mut T);
```

`impl<'a> Debug for ModuleField<'a>`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl<'a> Clone for ModuleField<'a>`

```rust
fn clone(self: &Self) -> ModuleField<'a>;
```



## Auto Trait Implementations

`impl<'a> Send for ModuleField<'a>`

`impl<'a> Sync for ModuleField<'a>`

`impl<'a> Freeze for ModuleField<'a>`

`impl<'a> Unpin for ModuleField<'a>`

`impl<'a> UnwindSafe for ModuleField<'a>`

`impl<'a> RefUnwindSafe for ModuleField<'a>`



