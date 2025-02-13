# ModuleGenerator

```rust
pub struct ModuleGenerator<'a> {
	pub configuration: &'a Configuration,
	pub prefix_path: String,
	pub module_file_name: &'a str,
	pub root_item: &'a Item,
	pub index: &'a Index,
	pub paths: &'a Paths,
	pub external_crate: &'a ExternalCrates,
}
```



## Fields

- `configuration` : `&'a Configuration`
- `prefix_path` : `String`
- `module_file_name` : `&'a str`
- `root_item` : `&'a Item`
- `index` : `&'a Index`
- `paths` : `&'a Paths`
- `external_crate` : `&'a ExternalCrates`


## Implementations

```rust
fn new(configuration: &'a Configuration, prefix_path: String, module_file_name: &'a str, root_item: &'a Item, index: &'a Index, paths: &'a Paths, external_crate: &'a ExternalCrates) -> Self;
```

```rust
fn auto(self: Self) -> Result<ModuleDocumentation<'a>>;
```



## Trait Implementations

`impl<T> Borrow<T> for ModuleGenerator<'a>
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for ModuleGenerator<'a>
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for ModuleGenerator<'a>
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for ModuleGenerator<'a>`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for ModuleGenerator<'a>
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for ModuleGenerator<'a>
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for ModuleGenerator<'a>
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl<'a> Send for ModuleGenerator<'a>`

`impl<'a> Sync for ModuleGenerator<'a>`

`impl<'a> Freeze for ModuleGenerator<'a>`

`impl<'a> Unpin for ModuleGenerator<'a>`

`impl<'a> UnwindSafe for ModuleGenerator<'a>`

`impl<'a> RefUnwindSafe for ModuleGenerator<'a>`



