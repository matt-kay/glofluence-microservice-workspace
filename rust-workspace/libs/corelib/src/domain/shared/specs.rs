/// Specification
pub trait Specification<T>: Send + Sync + 'static {
    fn is_satisfied_by(&self, candidate: &T) -> bool;

    // DSL: A && B
    fn and<B>(self, other: B) -> AndSpec<Self, B>
    where
        Self: Sized,
        B: Specification<T>,
    {
        AndSpec(self, other)
    }

    // DSL: A || B
    fn or<B>(self, other: B) -> OrSpec<Self, B>
    where
        Self: Sized,
        B: Specification<T>,
    {
        OrSpec(self, other)
    }

    // DSL: !A
    fn not(self) -> NotSpec<Self>
    where
        Self: Sized,
    {
        NotSpec(self)
    }
}

/// AND Specification
pub struct AndSpec<A, B>(pub A, pub B);

impl<T, A, B> Specification<T> for AndSpec<A, B>
where
    A: Specification<T>,
    B: Specification<T>,
{
    fn is_satisfied_by(&self, t: &T) -> bool {
        self.0.is_satisfied_by(t) && self.1.is_satisfied_by(t)
    }
}

/// OR Specification
pub struct OrSpec<A, B>(pub A, pub B);

impl<T, A, B> Specification<T> for OrSpec<A, B>
where
    A: Specification<T>,
    B: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.0.is_satisfied_by(candidate) || self.1.is_satisfied_by(candidate)
    }
}

/// NOT Specification
pub struct NotSpec<A>(pub A);

impl<T, A> Specification<T> for NotSpec<A>
where
    A: Specification<T>,
{
    fn is_satisfied_by(&self, t: &T) -> bool {
        !self.0.is_satisfied_by(t)
    }
}

