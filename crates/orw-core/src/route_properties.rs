use std::fs;
use std::io::BufReader;
use std::path::Path;

use quick_xml::events::Event;
use quick_xml::Reader;

/// Read human-readable route title from loose `RouteProperties.xml` (TSC route list name).
pub fn parse_route_properties_xml(content: &[u8]) -> Option<String> {
    let mut reader = Reader::from_reader(BufReader::new(content));
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut in_display = false;
    let mut name = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let local = e.local_name();
                let tag = String::from_utf8_lossy(local.as_ref());
                if tag.eq_ignore_ascii_case("DisplayName") {
                    in_display = true;
                }
            }
            Ok(Event::Text(t)) if in_display => {
                let s = t.unescape().ok()?.trim().to_string();
                if !s.is_empty() {
                    name = Some(s);
                }
                in_display = false;
            }
            Ok(Event::End(e)) => {
                let local = e.local_name();
                let tag = String::from_utf8_lossy(local.as_ref());
                if tag.eq_ignore_ascii_case("DisplayName") {
                    in_display = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => return name,
            _ => {}
        }
        buf.clear();
    }
    name
}

pub fn read_display_name_from_file(path: &Path) -> Option<String> {
    let data = fs::read(path).ok()?;
    parse_route_properties_xml(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_display_name() {
        let xml = br#"<?xml version="1.0"?>
        <cHcRouteProperties>
          <DisplayName d:type="cDeltaString">Ringbahn Berlin</DisplayName>
        </cHcRouteProperties>"#;
        assert_eq!(
            parse_route_properties_xml(xml).as_deref(),
            Some("Ringbahn Berlin")
        );
    }
}