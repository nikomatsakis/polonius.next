#![allow(unused)]

use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub const fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    pub const fn dummy() -> Self {
        Span::new(0, 0)
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T> {
    span: Span,
    inner: T,
}

impl<T> Spanned<T> {
    pub const fn new(inner: T, span: Span) -> Self {
        Spanned { span, inner }
    }

    pub const fn span(&self) -> Span {
        self.span
    }

    pub fn as_ref(this: &Self) -> Spanned<&T> {
        Spanned {
            inner: &this.inner,
            span: this.span,
        }
    }

    pub fn map<U>(this: Self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            inner: f(this.inner),
            span: this.span,
        }
    }
}

pub trait WithSpan: Sized {
    fn at(self, span: Span) -> Spanned<Self>;
}

impl<T> WithSpan for T {
    fn at(self, span: Span) -> Spanned<Self> {
        Spanned { inner: self, span }
    }
}

impl<T> ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> ops::DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
