#![feature(arbitrary_self_types)]

use anyhow::{anyhow, Result};
use indexmap::{IndexMap, IndexSet};
use nxpkg_tasks::{debug::ValueDebug, Value, ValueToString, Vc};
use nxpkg_tasks_testing::{register, run};

register!();

#[tokio::test]
async fn all_in_one() {
    run! {
        let a: Vc<u32> = Vc::cell(4242);
        assert_eq!(*a.await?, 4242);

        let a: Vc<MyTransparentValue> = Vc::cell(4242);
        assert_eq!(*a.await?, 4242);

        let b = MyEnumValue::cell(MyEnumValue::More(MyEnumValue::Yeah(42).into()));
        assert_eq!(*b.to_string().await?, "42");

        let c = MyStructValue {
            value: 42,
            next: Some(MyStructValue::new(a)),
        }
        .into();

        let result = my_function(a, b.get_last(), c, Value::new(MyEnumValue::Yeah(42)));
        assert_eq!(*result.my_trait_function().await?, "42");
        assert_eq!(*result.my_trait_function2().await?, "42");
        assert_eq!(*result.my_trait_function3().await?, "4242");
        assert_eq!(*result.to_string().await?, "42");

        // Testing Vc<Self> in traits

        let a: Vc<Number> = Vc::cell(32);
        let b: Vc<Number> = Vc::cell(10);
        let c: Vc<Number> = a.add(b);

        assert_eq!(*c.await?, 42);

        let a_erased: Vc<Box<dyn Add>> = Vc::upcast(a);
        let b_erased: Vc<Box<dyn Add>> = Vc::upcast(b);
        let c_erased: Vc<Box<dyn Add>> = a_erased.add(b_erased);

        assert_eq!(*Vc::try_resolve_downcast_type::<Number>(c_erased).await?.unwrap().await?, 42);

        let b_erased_other: Vc<Box<dyn Add>> = Vc::upcast(Vc::<NumberB>::cell(10));
        let c_erased_invalid: Vc<Box<dyn Add>> = a_erased.add(b_erased_other);
        assert!(c_erased_invalid.resolve().await.is_err());

        // Testing generic types.

        let vc_42 = Vc::cell(42);

        let option: Vc<Option<Vc<u32>>> = Vc::cell(Some(vc_42));
        assert_eq!(*option.is_some().await?, true);
        assert_eq!(*option.is_none().await?, false);
        assert_eq!(&*option.await?, &Some(vc_42));
        assert_eq!(option.dbg().await?.to_string(), "Some(\n    42,\n)");

        let option: Vc<Option<Vc<u32>>> = Default::default();
        assert_eq!(*option.is_some().await?, false);
        assert_eq!(*option.is_none().await?, true);
        assert_eq!(&*option.await?, &None);
        assert_eq!(option.dbg().await?.to_string(), "None");

        let vec: Vc<Vec<Vc<u32>>> = Vc::cell(vec![vc_42]);
        assert_eq!(*vec.len().await?, 1);
        assert_eq!(*vec.is_empty().await?, false);
        assert_eq!(&*vec.await?, &[vc_42]);
        assert_eq!(vec.dbg().await?.to_string(), "[\n    42,\n]");

        let vec: Vc<Vec<Vc<u32>>> = Default::default();
        assert_eq!(*vec.len().await?, 0);
        assert_eq!(*vec.is_empty().await?, true);
        assert_eq!(vec.dbg().await?.to_string(), "[]");

        let vec: Vc<Vec<Vc<Vec<Vc<u32>>>>> = Default::default();
        assert_eq!(*vec.len().await?, 0);
        assert_eq!(vec.dbg().await?.to_string(), "[]");

        let set: Vc<IndexSet<Vc<u32>>> = Vc::cell(IndexSet::from([vc_42]));
        assert_eq!(*set.len().await?, 1);
        assert_eq!(*set.is_empty().await?, false);
        assert_eq!(&*set.await?, &IndexSet::from([vc_42]));
        assert_eq!(set.dbg().await?.to_string(), "{\n    42,\n}");

        let set: Vc<IndexSet<Vc<u32>>> = Default::default();
        assert_eq!(*set.len().await?, 0);
        assert_eq!(*set.is_empty().await?, true);
        assert_eq!(&*set.await?, &IndexSet::<Vc<u32>>::default());
        assert_eq!(set.dbg().await?.to_string(), "{}");

        let map: Vc<IndexMap<_, _>> = Vc::cell(IndexMap::from([(vc_42, vc_42)]));
        assert_eq!(*map.len().await?, 1);
        assert_eq!(*map.is_empty().await?, false);
        assert_eq!(&*map.await?, &IndexMap::from([(vc_42, vc_42)]));
        assert_eq!(map.dbg().await?.to_string(), "{\n    42: 42,\n}");

        let map: Vc<IndexMap<Vc<u32>, Vc<u32>>> = Default::default();
        assert_eq!(*map.len().await?, 0);
        assert_eq!(*map.is_empty().await?, true);
        assert_eq!(&*map.await?, &IndexMap::<Vc<u32>, Vc<u32>>::default());
        assert_eq!(map.dbg().await?.to_string(), "{}");
    }
}

#[nxpkg_tasks::value(transparent, serialization = "auto_for_input")]
#[derive(Debug, Clone, PartialOrd, Ord, Hash)]
struct MyTransparentValue(u32);

#[nxpkg_tasks::value(shared, serialization = "auto_for_input")]
#[derive(Debug, Clone, PartialOrd, Ord, Hash)]
enum MyEnumValue {
    Yeah(u32),
    Nah,
    More(Vc<MyEnumValue>),
}

#[nxpkg_tasks::value_impl]
impl MyEnumValue {
    #[nxpkg_tasks::function]
    pub async fn get_last(self: Vc<Self>) -> Result<Vc<Self>> {
        let mut current = self;
        while let MyEnumValue::More(more) = &*current.await? {
            current = *more;
        }
        Ok(current)
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for MyEnumValue {
    #[nxpkg_tasks::function]
    fn to_string(&self) -> Vc<String> {
        match self {
            MyEnumValue::Yeah(value) => Vc::cell(value.to_string()),
            MyEnumValue::Nah => Vc::cell("nah".to_string()),
            MyEnumValue::More(more) => more.to_string(),
        }
    }
}

#[nxpkg_tasks::value(shared)]
struct MyStructValue {
    value: u32,
    next: Option<Vc<MyStructValue>>,
}

#[nxpkg_tasks::value_impl]
impl MyStructValue {
    #[nxpkg_tasks::function]
    pub async fn new(value: Vc<MyTransparentValue>) -> Result<Vc<Self>> {
        Ok(Self::cell(MyStructValue {
            value: *value.await?,
            next: None,
        }))
    }
}

#[nxpkg_tasks::value_impl]
impl ValueToString for MyStructValue {
    #[nxpkg_tasks::function]
    fn to_string(&self) -> Vc<String> {
        Vc::cell(self.value.to_string())
    }
}

#[nxpkg_tasks::value_impl]
impl MyTrait for MyStructValue {
    #[nxpkg_tasks::function]
    fn my_trait_function2(self: Vc<Self>) -> Vc<String> {
        self.to_string()
    }
    #[nxpkg_tasks::function]
    async fn my_trait_function3(&self) -> Result<Vc<String>> {
        if let Some(next) = self.next {
            return Ok(next.my_trait_function3());
        }
        Ok(Vc::cell(self.value.to_string()))
    }
}

#[nxpkg_tasks::value_trait]
trait MyTrait: ValueToString {
    // TODO #[nxpkg_tasks::function]
    async fn my_trait_function(self: Vc<Self>) -> Result<Vc<String>> {
        if *self.to_string().await? != "42" {
            return Err(anyhow!(
                "my_trait_function must only be called with 42 as value"
            ));
        }
        // Calling a function twice
        Ok(self.to_string())
    }

    fn my_trait_function2(self: Vc<Self>) -> Vc<String>;
    fn my_trait_function3(self: Vc<Self>) -> Vc<String>;
}

#[nxpkg_tasks::function]
async fn my_function(
    a: Vc<MyTransparentValue>,
    b: Vc<MyEnumValue>,
    c: Vc<MyStructValue>,
    d: Value<MyEnumValue>,
) -> Result<Vc<MyStructValue>> {
    assert_eq!(*a.await?, 4242);
    assert_eq!(*b.await?, MyEnumValue::Yeah(42));
    assert_eq!(c.await?.value, 42);
    assert_eq!(d.into_value(), MyEnumValue::Yeah(42));
    Ok(c)
}

#[nxpkg_tasks::value_trait]
trait Add {
    fn add(self: Vc<Self>, other: Vc<Self>) -> Vc<Self>;
}

#[nxpkg_tasks::value(transparent)]
struct Number(u32);

#[nxpkg_tasks::value_impl]
impl Add for Number {
    #[nxpkg_tasks::function]
    async fn add(self: Vc<Self>, other: Vc<Self>) -> Result<Vc<Self>> {
        Ok(Vc::cell(*self.await? + *other.await?))
    }
}

#[nxpkg_tasks::value(transparent)]
struct NumberB(u32);

#[nxpkg_tasks::value_impl]
impl Add for NumberB {
    #[nxpkg_tasks::function]
    async fn add(self: Vc<Self>, other: Vc<Self>) -> Result<Vc<Self>> {
        Ok(Vc::cell(*self.await? + *other.await?))
    }
}
