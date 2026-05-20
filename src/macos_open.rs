//! Finder "Open With" support on macOS.
//!
//! winit owns the `NSApplication` delegate, so we can't add
//! `application:openURLs:` there. Instead we register our own
//! `kAEOpenDocuments` Apple Event handler directly with the
//! `NSAppleEventManager`, which overrides the default routing that
//! otherwise fails with "cannot open files in the Markdown format".

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, NSObjectProtocol};
use objc2::{AnyThread, define_class, msg_send, sel};
use objc2_foundation::{NSAppleEventDescriptor, NSAppleEventManager};

// FourCC codes for the open-documents Apple Event.
const CORE_EVENT_CLASS: u32 = u32::from_be_bytes(*b"aevt");
const AE_OPEN_DOCUMENTS: u32 = u32::from_be_bytes(*b"odoc");
const KEY_DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");

static PENDING: OnceLock<Mutex<Vec<PathBuf>>> = OnceLock::new();
static INSTALLED: AtomicBool = AtomicBool::new(false);
static CTX: OnceLock<egui::Context> = OnceLock::new();

fn pending() -> &'static Mutex<Vec<PathBuf>> {
    PENDING.get_or_init(|| Mutex::new(Vec::new()))
}

/// Drain any files macOS asked us to open since the last call.
pub fn take_pending() -> Vec<PathBuf> {
    pending()
        .lock()
        .map(|mut v| std::mem::take(&mut *v))
        .unwrap_or_default()
}

define_class!(
    #[unsafe(super(NSObject))]
    #[name = "ZenOpenDocHandler"]
    struct OpenDocHandler;

    unsafe impl NSObjectProtocol for OpenDocHandler {}

    impl OpenDocHandler {
        #[unsafe(method(handleAppleEvent:withReplyEvent:))]
        fn handle_apple_event(
            &self,
            event: &NSAppleEventDescriptor,
            _reply: &NSAppleEventDescriptor,
        ) {
            collect_files(event);
        }
    }
);

impl OpenDocHandler {
    fn new() -> Retained<Self> {
        let this = Self::alloc().set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}

fn collect_files(event: &NSAppleEventDescriptor) {
    let Some(list) = event.paramDescriptorForKeyword(KEY_DIRECT_OBJECT) else {
        return;
    };
    let count = list.numberOfItems();
    let mut found = Vec::new();
    // Apple Event lists are 1-indexed.
    for i in 1..=count {
        let Some(item) = list.descriptorAtIndex(i) else {
            continue;
        };
        let Some(url) = item.fileURLValue() else {
            continue;
        };
        if let Some(path) = url.path() {
            found.push(PathBuf::from(path.to_string()));
        }
    }
    if !found.is_empty() {
        if let Ok(mut guard) = pending().lock() {
            guard.extend(found);
        }
        // wake the egui loop so the queue is drained promptly
        if let Some(ctx) = CTX.get() {
            ctx.request_repaint();
        }
    }
}

/// Register the open-documents Apple Event handler. Idempotent.
///
/// Call this as early as possible (before the event loop starts) so the
/// launch "open document" event — fired when the app is launched onto a
/// file from Finder — is caught before it is dispatched.
pub fn register() {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    let handler = OpenDocHandler::new();
    let manager = NSAppleEventManager::sharedAppleEventManager();
    let obj: &AnyObject = &handler;
    unsafe {
        manager.setEventHandler_andSelector_forEventClass_andEventID(
            obj,
            sel!(handleAppleEvent:withReplyEvent:),
            CORE_EVENT_CLASS,
            AE_OPEN_DOCUMENTS,
        );
    }
    // The manager does not retain the handler — keep it alive forever.
    std::mem::forget(handler);
}

/// Store the egui context so the handler can wake the loop when a file
/// arrives while the app is already running.
pub fn set_context(ctx: &egui::Context) {
    let _ = CTX.set(ctx.clone());
}
