# Template

This file is mainly for keeping format snippets that helps us overall design the `cargo wiki` generator.

## Index

1. [Syntax Block](#syntax-block)
    - [Struct](#struct-syntax-block)
    - [Struct Field](#struct-syntax-block)
    - [Enum](#enum-syntax-block)
    - [Variant](#enum-syntax-block)
    - [Union - TODO]()
    - [Trait - TODO]()
    - [Trait Alias - TODO]()
    - [Impl - TODO]()
    - [TypeAlias - TODO]()
    - [Function - TODO]()
    - [Constant - TODO]()
    - [Static - TODO]()
    - [External Type - TODO]()
    - [Macro - TODO]()
    - [Proc Macro - TODO]()
    - [Primitive - TODO]()
    - [Assoc Const - TODO]()
    - [Assoc Type - TODO]()
    - [Module - TODO]()
    - [Extern Crate - TODO]()
    - [Use - TODO]()

## Syntax Block

### Struct Syntax Block
```rust
pub struct Crate {
    pub root: Id,
    pub crate_version: Option<String>,
    pub includes_private: bool,
    pub index: HashMap<Id, Item>,
    pub paths: HashMap<Id, ItemSummary>,
    pub external_crates: HashMap<u32, ExternalCrate>,
    pub format_version: u32,
}
```

<details>
  <summary>Fields</summary>

- `root`: [`Id`](./<PATH-TO-LOCAL-DOCUMENTATION>)

    The id of the root Module item of the local crate.
- `crate_version`: [Option](path-to-option)<[String](path-to-string)>

    The version string given to `--crate-version`, if any.
- `includes_private`: [bool](path-to-bool)

    Whether or not the output includes private items.
- `index`: [HashMap](path-to-hashmap)<[Id](path-to-id), [Item](path-to-item)>

    A collection of all items in the local crate as well as some external traits and their items that are referenced locally.
- `paths`: [HashMap](path-to-hashmap)<[Id](path-to-id), [ItemSummary](path-to-item-summary)>

    Maps IDs to fully qualified paths and other info helpful for generating links.
- `external_crates`: [HashMap](path-to-hashmap)<[u32](path-to-u32), [ExternalCrate](path-to-external-crate)>

    Maps `crate_id` of items to a crate name and html_root_url if it exists.
- `format_version`: [u32](path-to-u32)

    A single version number to be used in the future when making backwards incompatible changes to the JSON output.

</details>

## Enum Block

```rust
pub enum HelloWorld {
    VariantOne,
    VariantTuple(Box<HelloWorld>),
    VariantStruct {
        say_hello: bool
    }
}
```

<details>
    <summary>Variants</summary>

- `VariantOne`

  Base Variant
- `VariantTuple`

  Tuple Variant
- `VariantStruct`

  Struct Variant
</details>
