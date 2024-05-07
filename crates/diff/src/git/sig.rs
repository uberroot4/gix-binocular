
#[derive(Hash, PartialOrd, Ord, Eq, PartialEq, Debug)]
pub struct Sig {
    pub name: gix::bstr::BString,
    pub email: gix::bstr::BString,
    pub time: gix::date::Time
}

impl From<gix::actor::Signature> for Sig {
    fn from(gix::actor::Signature { name, email, time }: gix::actor::Signature) -> Self {
        Self { name, email, time }
    }
}
