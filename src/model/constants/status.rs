use core::fmt;
use std::convert::TryFrom;

macro_rules! status {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $name:ident, $phrase:expr);
        )+
    ) => {
        impl Status {
        $(
            $(#[$docs])*
            pub const $name: Status = Status({ $num });
        )+

        }

        fn canonical_reason(num: i16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}
status! {
    /// 0 Success
    (0, SUCCESS, "Success");
    /// -1 Fail
    (-1, FAIL, "Fail");
    /// 102 Processing
    (102, PROCESSING, "Processing");
    /// 102 Unknown
    (900, UNKNOWN, "Unknown");
}

#[derive(Clone, Copy, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Status(i16);

impl Status {
    #[inline]
    pub fn from_i16(src: i16) -> Result<Status, ()> {
        Some(src).map(Status).ok_or(())
    }

    #[inline]
    pub fn as_i16(&self) -> i16 {
        (*self).into()
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0)
    }
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// Formats the status code, *including* the canonical reason.
///
/// # Example
///
/// ```
/// # use http::StatusCode;
/// assert_eq!(format!("{}", StatusCode::OK), "200 OK");
/// ```
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            i16::from(*self),
            self.canonical_reason().unwrap_or("<unknown status code>")
        )
    }
}

impl From<Status> for i16 {
    #[inline]
    fn from(status: Status) -> i16 {
        status.0
    }
}

impl PartialEq<i16> for Status {
    #[inline]
    fn eq(&self, other: &i16) -> bool {
        self.as_i16() == *other
    }
}

impl PartialEq<Status> for i16 {
    #[inline]
    fn eq(&self, other: &Status) -> bool {
        *self == other.as_i16()
    }
}

impl<'a> From<&'a Status> for Status {
    #[inline]
    fn from(t: &'a Status) -> Self {
        *t
    }
}

impl TryFrom<i16> for Status {
    type Error = ();

    #[inline]
    fn try_from(t: i16) -> Result<Self, Self::Error> {
        Status::from_i16(t)
    }
}
