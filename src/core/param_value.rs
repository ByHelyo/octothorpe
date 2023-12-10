use std::borrow::Cow;

pub trait ParamValue<'a> {
    fn as_value(&self) -> Cow<'a, str>;
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).clone()
    }
}