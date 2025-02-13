# Configuration

```rust
pub struct Configuration {
	pub package: String,
	pub workspace: bool,
	pub features: String,
	pub all_features: bool,
	pub no_default_features: bool,
	pub document_private_items: bool,
	pub no_deps: bool,
	pub markdown_flavor: MarkdownFlavor,
	pub structure: WikiStructure,
}
```



## Fields

- `package` : `String`

	The package to document. See [cargo-pkgid](https://doc.rust-lang.org/cargo/commands/cargo-pkgid.html)
for the SPEC format.
- `workspace` : `bool`
- `features` : `String`

	Space or comma separated list of features to activate. Features of workspace members may be
enabled with package-name/feature-name syntax. This flag may be specified multiple times,
which enables all specified features.
- `all_features` : `bool`

	Activate all available features of all selected packages.
- `no_default_features` : `bool`

	Do not activate the default feature of the selected packages.
- `document_private_items` : `bool`
- `no_deps` : `bool`
- `markdown_flavor` : `MarkdownFlavor`
- `structure` : `WikiStructure`




## Trait Implementations

`impl<T> Borrow<T> for Configuration
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for Configuration
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for Configuration
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for Configuration`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for Configuration
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for Configuration
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for Configuration
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```

`impl Parser for Configuration`

`impl CommandFactory for Configuration`

```rust
fn command<'b>() -> Command;
```

```rust
fn command_for_update<'b>() -> Command;
```

`impl FromArgMatches for Configuration`

```rust
fn from_arg_matches(__clap_arg_matches: &ArgMatches) -> Result<Self, Error>;
```

```rust
fn from_arg_matches_mut(__clap_arg_matches: &mut ArgMatches) -> Result<Self, Error>;
```

```rust
fn update_from_arg_matches(self: &mut Self, __clap_arg_matches: &ArgMatches) -> Result<(), Error>;
```

```rust
fn update_from_arg_matches_mut(self: &mut Self, __clap_arg_matches: &mut ArgMatches) -> Result<(), Error>;
```

`impl Args for Configuration`

```rust
fn group_id() -> Option<Id>;
```

```rust
fn augment_args<'b>(__clap_app: Command) -> Command;
```

```rust
fn augment_args_for_update<'b>(__clap_app: Command) -> Command;
```

`impl Debug for Configuration`

```rust
fn fmt(self: &Self, f: &mut Formatter<'_>) -> Result;
```



## Auto Trait Implementations

`impl Send for Configuration`

`impl Sync for Configuration`

`impl Freeze for Configuration`

`impl Unpin for Configuration`

`impl UnwindSafe for Configuration`

`impl RefUnwindSafe for Configuration`



