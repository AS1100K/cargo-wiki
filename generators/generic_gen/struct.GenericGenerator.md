# GenericGenerator

```rust
pub struct GenericGenerator;
```





## Implementations

```rust
fn generate_generics(generics: &Generics) -> Result<(, String)>;
```

```rust
fn generate_generic_params(generic_params: &Vec<GenericParamDef>) -> Result<String>;
```

```rust
fn generate_generic_bounds(bound: &GenericBound) -> Result<String>;
```

```rust
fn generate_generic_args(generic_args: &GenericArgs) -> String;
```



## Trait Implementations

`impl<T> Borrow<T> for GenericGenerator
where
	T: ?Sized`

```rust
fn borrow(self: &Self) -> &T;
```

`impl<T> BorrowMut<T> for GenericGenerator
where
	T: ?Sized`

```rust
fn borrow_mut(self: &mut Self) -> &mut T;
```

`impl<T, U> Into<U> for GenericGenerator
where
	U: From<T>`

```rust
/// Calls `U::from(self)`.
/// 
/// That is, this conversion is whatever the implementation of
/// <code>[From]&lt;T&gt; for U</code> chooses to do.
fn into(self: Self) -> U;
```

`impl<T> From<T> for GenericGenerator`

```rust
/// Returns the argument unchanged.
fn from(t: T) -> T;
```

`impl<T, U> TryInto<U> for GenericGenerator
where
	U: TryFrom<T>`

```rust
fn try_into(self: Self) -> Result<U, >;
```

`impl<T, U> TryFrom<U> for GenericGenerator
where
	U: Into<T>`

```rust
fn try_from(value: U) -> Result<T, >;
```

`impl<T> Any for GenericGenerator
where
	T: 'static + ?Sized`

```rust
fn type_id(self: &Self) -> TypeId;
```



## Auto Trait Implementations

`impl Send for GenericGenerator`

`impl Sync for GenericGenerator`

`impl Freeze for GenericGenerator`

`impl Unpin for GenericGenerator`

`impl UnwindSafe for GenericGenerator`

`impl RefUnwindSafe for GenericGenerator`



