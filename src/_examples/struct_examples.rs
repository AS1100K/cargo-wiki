pub struct UnitStruct;

pub struct TupleStruct(i32, u64);

pub struct Basic {
    pub type_: String,
    pub r#type: String,
}

pub struct ComplexStruct<'a, T: 'a + ?Sized, U: Default = i32> {
    /// This is a reference
    pub reference: &'a T,
    pub value: U,
    pub slice: &'a [u32; 50],
}

pub struct PredicateStruct<'a, T, U>
where
    T: 'a + ?Sized,
    U: Default,
{
    pub reference: &'a T,
    pub value: U,
    private_field: String,
}

/// Uses Higher-Rank Trait Bounds (HRTBs)
pub struct HRTBPredicateStruct<'a, T, U, F>
where
    for<'b> T: 'b + ?Sized,
    U: Default,
    F: Fn(&'a T) -> U,
{
    pub reference: &'a T,
    pub value: U,
    pub transformer: F,
    private_field: String,
}
