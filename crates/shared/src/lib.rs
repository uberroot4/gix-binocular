use gix::bstr::BString;

#[derive(Hash, PartialOrd, Ord, Eq, PartialEq, Debug)]
pub struct Sig {
    pub name: BString,
    pub email: BString,
    pub time: gix::date::Time
}

impl From<gix::actor::Signature> for Sig {
    fn from(gix::actor::Signature { name, email, time }: gix::actor::Signature) -> Self {
        Self { name, email, time }
    }
}

impl From<gix::actor::SignatureRef<'_>> for Sig {

    fn from(value: gix::actor::SignatureRef<'_>) -> Self {
        Self {
            name: BString::from(value.name),
            email: BString::from(value.name),
            time: value.time,
        }
    }
}