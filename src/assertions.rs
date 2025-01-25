use speculoos::{AssertionFailure, Spec};

pub trait IsContainedIn<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn is_contained_in(&mut self, collection: &[T]);
}

#[cfg_attr(coverage_nightly, coverage(off))]
impl<T> IsContainedIn<T> for Spec<'_, T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn is_contained_in(&mut self, collection: &[T]) {
        let subject = self.subject;
        if !collection.contains(subject) {
            AssertionFailure::from_spec(self)
                .with_expected(format!("<{subject:?}> to be contained in <{collection:?}>"))
                .with_actual("but wasn't.".into())
                .fail();
        }
    }
}
