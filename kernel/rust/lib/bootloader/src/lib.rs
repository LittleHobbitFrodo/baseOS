//	bootloader.rs (bootloader crate)
//	this file originally belonged to baseOS project
//		an OS template on which to build

#![no_std]

//  put bootloader requests here

pub use limine_rs;

use limine_rs::BaseRevision;
use limine_rs::request::{FramebufferRequest, StackSizeRequest, RequestsStartMarker, RequestsEndMarker};


#[unsafe(link_section = ".requests_start_marker")]
pub static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
pub static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(link_section = ".requests")]
pub static STACK_SIZE: StackSizeRequest = StackSizeRequest::new().with_size(64*1024);

#[unsafe(link_section = ".requests")]
pub static revision: BaseRevision = BaseRevision::with_revision(3);






//  REQUESTS NEEDED BY LOCAL MINISTD CRATE
//  please do no change names or delete it

//  reserved by [`ministd::renderer`]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::with_revision(4);


