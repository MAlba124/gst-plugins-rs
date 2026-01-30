// SPDX-License-Identifier: MPL-2.0

use crate::signaller::Signallable;
use gst::{glib, prelude::ObjectExt, subclass::prelude::ObjectSubclassIsExt};

#[cfg(feature = "whep-client")]
mod client;
#[cfg(feature = "whep")]
mod server;

#[cfg(feature = "whep-client")]
glib::wrapper! {
    pub struct WhepClientSignaller(ObjectSubclass<client::WhepClient>) @implements Signallable;
}

#[cfg(feature = "whep")]
glib::wrapper! {
    pub struct WhepServerSignaller(ObjectSubclass<server::WhepServer>) @implements Signallable;
}

#[cfg(feature = "whep-client")]
unsafe impl Send for WhepClientSignaller {}
#[cfg(feature = "whep-client")]
unsafe impl Sync for WhepClientSignaller {}

#[cfg(feature = "whep-client")]
impl Default for WhepClientSignaller {
    fn default() -> Self {
        let sig: WhepClientSignaller = glib::Object::new();
        sig.connect_closure("webrtcbin-ready", false, sig.imp().on_webrtcbin_ready());
        sig
    }
}

#[cfg(feature = "whep")]
unsafe impl Send for WhepServerSignaller {}
#[cfg(feature = "whep")]
unsafe impl Sync for WhepServerSignaller {}

#[cfg(feature = "whep")]
impl Default for WhepServerSignaller {
    fn default() -> Self {
        let sig: WhepServerSignaller = glib::Object::new();
        sig.connect_closure("webrtcbin-ready", false, sig.imp().on_webrtcbin_ready());
        sig
    }
}
