use gix::bstr::BString;

#[derive(Hash, PartialOrd, Ord, Eq, PartialEq, Debug)]
pub struct Sig {
    pub name: BString,
    pub email: BString,
    pub time: gix::date::Time,
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
            email: BString::from(value.email),
            time: value.time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gix::actor::{Signature, SignatureRef};
    use gix::bstr::BString;
    use gix::date::Time;

    // Helper function to create a dummy Time struct
    fn dummy_time() -> Time {
        Time {
            seconds: 1609459200, // Jan 1, 2021, 00:00:00 UTC
            offset: 0,             // UTC offset
            sign: gix::date::time::Sign::Plus,
        }
    }

    #[test]
    fn test_sig_from_signature() {
        let name = BString::from("John Doe");
        let email = BString::from("john@example.com");
        let time = dummy_time();

        let signature = Signature { name: name.clone(), email: email.clone(), time };

        let sig: Sig = signature.into();

        assert_eq!(sig.name, name);
        assert_eq!(sig.email, email);
        assert_eq!(sig.time, time);
    }

    #[test]
    fn test_sig_from_signature_ref() {
        let name = BString::from("Jane Doe");
        let email = BString::from("jane@example.com");
        let time = dummy_time();

        let signature_ref = SignatureRef {
            name: (&name).as_ref(),
            email: (&email).as_ref(),
            time,
        };

        let sig: Sig = signature_ref.into();

        assert_eq!(sig.name, name);
        assert_eq!(sig.email, email);
        assert_eq!(sig.time, time);
    }

    #[test]
    fn test_sig_equality() {
        let name = BString::from("Alice");
        let email = BString::from("alice@example.com");
        let time = dummy_time();

        let sig1 = Sig {
            name: name.clone(),
            email: email.clone(),
            time,
        };

        let sig2 = Sig {
            name,
            email,
            time,
        };

        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_sig_ordering() {
        let name1 = BString::from("Charlie");
        let email1 = BString::from("charlie@example.com");
        let time1 = dummy_time();

        let name2 = BString::from("Bob");
        let email2 = BString::from("bob@example.com");
        let time2 = dummy_time();

        let sig1 = Sig {
            name: name1,
            email: email1,
            time: time1,
        };

        let sig2 = Sig {
            name: name2,
            email: email2,
            time: time2,
        };

        assert!(sig1 > sig2); // Ord implemented via name, email, and time
    }

    #[test]
    fn test_sig_debug_format() {
        let name = BString::from("Diane");
        let email = BString::from("diane@example.com");
        let time = dummy_time();

        let sig = Sig {
            name: name.clone(),
            email: email.clone(),
            time,
        };

        let debug_output = format!("{:?}", sig);
        assert!(debug_output.contains("Diane"));
        assert!(debug_output.contains("diane@example.com"));
    }
}

