//! Based from [`proc-macro2`](https://github.com/alexcrichton/proc-macro2).

use std::str::Chars;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Cursor<'a> {
    pub rest: &'a str,
    pub off: u32,
}

impl<'a> Cursor<'a> {
    pub fn adv(&self, amt: usize) -> Cursor<'a> {
        Cursor {
            rest: &self.rest[amt..],
            off: self.off + (amt as u32),
        }
    }

    pub fn find(&self, p: char) -> Option<usize> {
        self.rest.find(p)
    }

    pub fn adv_find(&self, amt: usize, p: char) -> Option<usize> {
        self.rest[amt..].find(p)
    }

    pub fn adv_starts_with(&self, amt: usize, s: &str) -> bool {
        self.rest[amt..].starts_with(s)
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.rest.starts_with(s)
    }

    pub fn is_empty(&self) -> bool {
        self.rest.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rest.len()
    }

    pub fn chars(&self) -> Chars<'a> {
        self.rest.chars()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LexError {
    Fail,
    Next,
}

pub type PResult<'a, O> = Result<(Cursor<'a>, O), LexError>;

macro_rules! do_parse {
    ($i:expr, ( $($rest:expr),* )) => {
        Ok(($i, ( $($rest),* )))
    };

    ($i:expr, $e:ident >> $($rest:tt)*) => {
        do_parse!($i, call!($e) >> $($rest)*)
    };

    ($i:expr, $submac:ident!( $($args:tt)* ) >> $($rest:tt)*) => {
        match $submac!($i, $($args)*) {
            Err(e) => Err(e),
            Ok((i, _)) => do_parse!(i, $($rest)*),
        }
    };

    ($i:expr, $field:ident : $e:ident >> $($rest:tt)*) => {
        do_parse!($i, $field: call!($e) >> $($rest)*)
    };

    ($i:expr, $field:ident : $submac:ident!( $($args:tt)* ) >> $($rest:tt)*) => {
        match $submac!($i, $($args)*) {
            Err(e) => Err(e),
            Ok((i, o)) => {
                let $field = o;
                do_parse!(i, $($rest)*)
            },
        }
    };
}

macro_rules! call {
    ($i:expr, $fun:expr $(, $args:expr)*) => {
        $fun($i $(, $args)*)
    };
}

macro_rules! opt {
    ($i:expr, $submac:ident!($($args:tt)*)) => {
        match $submac!($i, $($args)*) {
            Ok((i, o)) => Ok((i, Some(o))),
            Err(_) => Ok(($i, None)),
        }
    };
    ($i:expr, $f:expr) => {
        match $f($i) {
            Ok((i, o)) => Ok((i, Some(o))),
            Err(_) => Ok(($i, None)),
        }
    };
}

macro_rules! take_while {
    ($i:expr, $f:expr) => {{
        if $i.len() == 0 {
            Ok(($i, ""))
        } else {
            match $i.chars().position(|c| !$f(c)) {
                Some(i) => Ok(($i.adv(i), &$i.rest[..i])),
                None => Ok(($i.adv($i.len()), &$i.rest[..$i.len()])),
            }
        }
    }};
}

macro_rules! tag {
    ($i:expr, $tag:expr) => {
        if $i.starts_with($tag) {
            Ok(($i.adv($tag.len()), &$i.rest[..$tag.len()]))
        } else {
            Err(LexError::Next)
        }
    };
}

macro_rules! map_fail {
    ($($t:tt)*) => { ($($t)*).map_err(|_| LexError::Fail) };
}

#[inline]
pub fn ws(i: Cursor) -> PResult<&str> {
    take_while!(i, is_ws)
}

#[inline]
pub fn skip_ws(input: Cursor) -> Cursor {
    match ws(input) {
        Ok((rest, _)) => rest,
        Err(_) => input,
    }
}

#[inline]
pub fn is_ws(ch: char) -> bool {
    // Rust treats left-to-right mark and right-to-left mark as whitespace
    ch.is_whitespace() || ch == '\u{200e}' || ch == '\u{200f}'
}
