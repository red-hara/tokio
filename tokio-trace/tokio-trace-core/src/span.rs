//! Spans represent periods of time in the execution of a program.

use ::{Metadata, field};

/// Identifies a span within the context of a process.
///
/// Span IDs are used primarily to determine if two handles refer to the same
/// span, without requiring the comparison of the span's fields.
///
/// They are generated by [`Subscriber`](::Subscriber)s for each span as it is
/// created, through the [`new_id`](::Subscriber::new_span_id) trait
/// method. See the documentation for that method for more information on span
/// ID generation.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Span(u64);

/// Attributes provided to a `Subscriber` describing a new span when it is
/// created.
#[derive(Debug)]
pub struct NewSpan<'a> {
    metadata: &'a Metadata<'a>,
    values: &'a field::ValueSet<'a>,
    parent: Parent,
}

#[derive(Debug)]
enum Parent {
    /// The new span will be a root span.
    Root,
    /// The new span will be rooted in the current span.
    Current,
    /// The new span has an explicitly-specified parent.
    Explicit(Span),
}

// ===== impl Span =====

impl Span {
    /// Constructs a new span ID from the given `u64`.
    pub fn from_u64(u: u64) -> Self {
        Span(u)
    }

    /// Returns the span's ID as a  `u64`.
    pub fn into_u64(&self) -> u64 {
        self.0
    }
}

// ===== impl NewSpan =====

impl<'a> NewSpan<'a> {
    /// Returns a new `NewSpan` as a child of the current span, with the
    /// specified metadata and values.
    pub fn new(metadata: &'a Metadata<'a>, values: &'a field::ValueSet<'a>) -> Self {
        Self {
            metadata,
            values,
            parent: Parent::Current,
        }
    }

    /// Returns a new `NewSpan` at the root of its own trace tree, with the specified metadata and values.
    pub fn new_root(metadata: &'a Metadata<'a>, values: &'a field::ValueSet<'a>) -> Self {
        Self {
            metadata,
            values,
            parent: Parent::Root,
        }
    }

    /// Returns a new `NewSpan` as a child of the specified parent span, with the
    /// specified metadata and values.
    pub fn child_of(parent: Span, metadata: &'a Metadata<'a>, values: &'a field::ValueSet<'a>) -> Self {
        Self {
            metadata,
            values,
            parent: Parent::Explicit(parent)
        }
    }

    /// Returns a reference to the new span's metadata.
    pub fn metadata(&self) -> &Metadata<'a> {
        self.metadata
    }

    /// Returns a reference to a `ValueSet` containing any values the new span
    /// was created with.
    pub fn values(&self) -> &field::ValueSet<'a> {
        self.values
    }

    /// Returns true if the new span shoold be a root.
    pub fn is_root(&self) -> bool {
        match self.parent {
            Parent::Root => true,
            _ => false,
        }
    }

    /// Returns true if the new span should be a child of the current span.
    pub fn is_in_current(&self) -> bool {
        match self.parent {
            Parent::Current => true,
            _ => false,
        }
    }

    /// Returns the new span's explicitly-specified parent, if there is one.
    ///
    /// Otherwise (if the new span is a root or is a child of the current span),
    /// returns false.
    pub fn parent(&self) -> Option<&Span> {
        match self.parent {
            Parent::Explicit(ref p) => Some(p),
            _ => None,
        }
    }
}
