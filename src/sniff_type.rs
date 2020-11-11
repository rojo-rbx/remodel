/// Detect whether the given buffer contains an XML document, a binary document,
/// or neither.
///
/// Binary document start with `<roblox!`, while XML documents start with either
/// `<roblox ` or `<roblox>`.
pub fn sniff_type(buffer: &[u8]) -> Option<DocumentType> {
    let header = buffer.get(0..8)?;

    if &header[0..7] != b"<roblox" {
        return None;
    }

    match header[7] {
        b'!' => Some(DocumentType::Binary),
        b' ' | b'>' => Some(DocumentType::Xml),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentType {
    Xml,
    Binary,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sniff_examples() {
        assert_eq!(sniff_type(b"<roblox!hello"), Some(DocumentType::Binary));
        assert_eq!(sniff_type(b"<roblox!"), Some(DocumentType::Binary));
        assert_eq!(
            sniff_type(b"<roblox xml:someschemajunk>"),
            Some(DocumentType::Xml)
        );
        assert_eq!(sniff_type(b"<roblox>"), Some(DocumentType::Xml));

        // not enough characters in these
        assert_eq!(sniff_type(b""), None);
        assert_eq!(sniff_type(b"<roblox"), None);
    }
}
