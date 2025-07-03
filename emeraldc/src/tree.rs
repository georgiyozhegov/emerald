use std::ops::Index;

pub trait Tree: Index<Self::Id> {
    type Node;
    type Id;

    fn allocate(&mut self, node: Self::Node) -> Self::Id;
}

pub trait Visit<T>
where
    T: Tree,
{
    type Output;

    fn visit(&mut self, id: T::Id, node: &T::Node) -> Self::Output;
}

pub trait Convert<T>
where
    T: Tree,
{
    type Output;

    fn convert(&mut self, id: T::Id, node: T::Node) -> Self::Output;
}
