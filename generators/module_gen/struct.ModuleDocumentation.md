# ModuleDocumentation

```rust
pub struct ModuleDocumentation<'a> {
	pub file_path: String,
	pub title: &'a str,
	pub module_items: ModuleItems<'a>,
	pub content: Vec<ModuleContent<'a>>,
	pub inner_modules: Vec<ModuleDocumentation<'a>>,
}
```



## Fields

- `file_path` : `String`
- `title` : `&'a str`
- `module_items` : `ModuleItems<'a>`
- `content` : `Vec<ModuleContent<'a>>`
- `inner_modules` : `Vec<ModuleDocumentation<'a>>`


## Implementations

```rust
fn generate_docs(self: &Self, configuration: &Configuration) -> Result<String>;
```



## Trait Implementations

`impl<T> Borrow<T> for ModuleDocumentation<'a>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for ModuleDocumentation<'a>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T> CloneToUninit for ModuleDocumentation<'a>
where
	T: Clone`

```rust
unsafe fn clone_to_uninit(self: &Self, dst: *mut u8);
```

`impl<T, U> Into<U> for ModuleDocumentation<'a>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for ModuleDocumentation<'a>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for ModuleDocumentation<'a>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for ModuleDocumentation<'a>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for ModuleDocumentation<'a>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T> ToOwned for ModuleDocumentation<'a>
where
	T: Clone`

```rust
fn to_owned(self: &Self) -> T;
```

```rust
fn clone_into(self: &Self, target: &mut T);
```

`impl<'a> Default for ModuleDocumentation<'a>`

```rust
fn default() -> ModuleDocumentation<'a>;
```

`impl<'a> Debug for ModuleDocumentation<'a>`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl<'a> Clone for ModuleDocumentation<'a>`

```rust
fn clone(self: &Self) -> ModuleDocumentation<'a>;
```



## Auto Trait Implementations

`impl<'a> Send for ModuleDocumentation<'a>`

`impl<'a> Sync for ModuleDocumentation<'a>`

`impl<'a> Freeze for ModuleDocumentation<'a>`

`impl<'a> Unpin for ModuleDocumentation<'a>`

`impl<'a> UnwindSafe for ModuleDocumentation<'a>`

`impl<'a> RefUnwindSafe for ModuleDocumentation<'a>`



