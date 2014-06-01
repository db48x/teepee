//! defines all headers that contain urls

extern crate url = "url#0.1";

use std;
use std::io::IoResult;
use std::str::{SendStr, Slice};
use super::{Header, HeaderMarker};
use self::url::{Url, Host};

header!(#[doc="The Location response-header field is used to redirect the recipient to a location other than the Request-URI for completion of the request or identification of a new resource."]
        LOCATION, "location", Url)

header!(#[doc="The Content-Location entity-header field MAY be used to supply the resource location for the entity enclosed in the message when that entity is accessible from a location separate from the requested resource's URI."]
        CONTENT_LOCATION, "content-location", PossiblyRelativeUrl)

header!(#[doc="The Host request-header field specifies the Internet host and port number of the resource being requested, as obtained from the original URI given by the user or referring resource (generally an HTTP URL, as described in section 3.2.2)."]
        HOST, "host", Host)

header!(#[doc="The Referer[sic] request-header field allows the client to specify, for the server's benefit, the address (URI) of the resource from which the Request-URI was obtained (the 'referrer', although the header field is misspelled.)"]
        REFERRER, "referer", PossiblyRelativeUrl)

impl Header for Url {
    fn parse_header(raw: &[Vec<u8>]) -> Option<Url> {
        let raw = require_single_field!(raw);
        let raw = match std::str::from_utf8(raw) {
            Some(raw) => raw,
            None => return None,
        };
        match Url::parse(raw, None) {
            Ok(url) => Some(url),
            Err(_) => None,
        }
    }
    fn fmt_header(&self, w: &mut Writer) -> IoResult<()> {
        write!(w, "{}", self.serialize())
    }
}

impl Header for Host {
    fn parse_header(raw: &[Vec<u8>]) -> Option<Host> {
        let raw = require_single_field!(raw);
        let raw = match std::str::from_utf8(raw) {
            Some(raw) => raw,
            None => return None,
        };
        match Host::parse(raw) {
            Ok(url) => Some(url),
            Err(_) => None,
        }
    }
    fn fmt_header(&self, w: &mut Writer) -> IoResult<()> {
        write!(w, "{}", self.serialize())
    }
}

/// The data type for urls that may be relative
#[deriving(Clone, Show)]
pub enum PossiblyRelativeUrl {
    /// an absolute url
    Absolute(Url),
    /// a relative url
    Relative(Url),
}

impl Header for PossiblyRelativeUrl {
    fn parse_header(raw: &[Vec<u8>]) -> Option<PossiblyRelativeUrl> {
        let _ = require_single_field!(raw);
        match Header::parse_header(raw) {
            Some(url) => Some(Absolute(url)),
            None => match Header::parse_header(raw) {
                Some(url) => Some(Relative(url)),
                None => None,
            }
        }
    }
 
    fn fmt_header(&self, w: &mut Writer) -> IoResult<()> {
        match *self {
            Absolute(ref url) => url.fmt_header(w),
            Relative(ref url) => url.fmt_header(w),
        }
    }
}

#[cfg(test)]
mod test {
    use std::string::String;
    use super::url::*;
    use super::super::Header;

    #[test]
    fn test_url() {
        let url_str = "http://example.com/";
        let url: Url = Header::parse_header([Vec::from_slice(url_str.as_bytes())]).unwrap();
        assert_eq!(url.serialize(), String::from_str(url_str));
    }
}
