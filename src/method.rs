//! Marker types for requests.

use std::fmt::Debug;

pub type Limit = usize;

/// A marker trait used by [`Request`](../request/struct.Request.html)
/// to signal which execution path should be taken.
pub trait Method: Default + Debug + Clone + Copy {}

/// A list method uses a GET request with pagination.
///
/// This method enables the [`limit()`](../request/struct.Request.html#method.limit) call on requests.
#[derive(Debug, Clone, Copy, Default)]
pub struct List(pub Option<Limit>);
impl Method for List {}

/// A get method uses a GET request.
#[derive(Debug, Clone, Copy, Default)]
pub struct Get;
impl Method for Get {}

/// A create method uses a POST request.
#[derive(Debug, Clone, Copy, Default)]
pub struct Create;
impl Method for Create {}

/// An update method uses a PUT request.
#[derive(Debug, Clone, Copy, Default)]
pub struct Update;
impl Method for Update {}

/// A delete method uses a DELETE request.
#[derive(Debug, Clone, Copy, Default)]
pub struct Delete;
impl Method for Delete {}
