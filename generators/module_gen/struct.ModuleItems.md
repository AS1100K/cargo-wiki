# ModuleItems

```rust
pub struct ModuleItems<'a> {
	pub modules: Vec<ModuleField<'a>>,
	pub traits: Vec<ModuleField<'a>>,
	pub functions: Vec<ModuleField<'a>>,
	pub macros: Vec<ModuleField<'a>>,
	pub re_exports: Vec<ModuleField<'a>>,
	pub structs: Vec<ModuleField<'a>>,
	pub enums: Vec<ModuleField<'a>>,
	pub consts: Vec<ModuleField<'a>>,
}
```



## Fields

- `modules` : `Vec<ModuleField<'a>>`
- `traits` : `Vec<ModuleField<'a>>`
- `functions` : `Vec<ModuleField<'a>>`
- `macros` : `Vec<ModuleField<'a>>`
- `re_exports` : `Vec<ModuleField<'a>>`
- `structs` : `Vec<ModuleField<'a>>`
- `enums` : `Vec<ModuleField<'a>>`
- `consts` : `Vec<ModuleField<'a>>`


## Implementations

```rust
fn generate_docs(self: &Self) -> String;
```



## Trait Implementations

`impl<T> Borrow<T> for ModuleItems<'a>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for ModuleItems<'a>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T> CloneToUninit for ModuleItems<'a>
where
	T: Clone`

```rust
unsafe fn clone_to_uninit(self: &Self, dst: *mut u8);
```

`impl<T, U> Into<U> for ModuleItems<'a>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for ModuleItems<'a>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for ModuleItems<'a>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for ModuleItems<'a>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for ModuleItems<'a>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl<T> ToOwned for ModuleItems<'a>
where
	T: Clone`

```rust
fn to_owned(self: &Self) -> T;
```

```rust
fn clone_into(self: &Self, target: &mut T);
```

`impl<'a> Default for ModuleItems<'a>`

```rust
fn default() -> ModuleItems<'a>;
```

`impl<'a> Debug for ModuleItems<'a>`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```

`impl<'a> Clone for ModuleItems<'a>`

```rust
fn clone(self: &Self) -> ModuleItems<'a>;
```



## Auto Trait Implementations

`impl<'a> Send for ModuleItems<'a>`

`impl<'a> Sync for ModuleItems<'a>`

`impl<'a> Freeze for ModuleItems<'a>`

`impl<'a> Unpin for ModuleItems<'a>`

`impl<'a> UnwindSafe for ModuleItems<'a>`

`impl<'a> RefUnwindSafe for ModuleItems<'a>`



