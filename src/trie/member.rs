pub trait MemberType
where
    Self: Default + Copy + PartialEq,
{
    fn add(&mut self) -> Self;
    fn is_member(&self) -> bool {
        *self != Default::default()
    }
}

impl MemberType for bool {
    fn add(&mut self) -> Self {
        let ret = *self;
        *self = true;
        ret
    }
}

impl MemberType for u32 {
    fn add(&mut self) -> Self {
        let ret = *self;
        *self += 1;
        ret
    }
}
