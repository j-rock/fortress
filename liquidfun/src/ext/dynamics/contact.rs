use super::super::super::box2d::dynamics::contact::Contact;

pub struct ContactIterator {
    contact: Option<Contact>
}

impl Contact {
    /// Get a world contact iterator.
    pub fn iter(&self) -> ContactIterator {
        ContactIterator { contact: Some(self.clone()) }
    }
}

impl Iterator for ContactIterator {
    type Item = Contact;
    fn next(&mut self) -> Option<Contact> {
        let contact = self.contact.clone();

        self.contact = match self.contact {
            Some(ref x) => x.get_next(),
            None => None,
        };

        contact
    }
}
