//	bootloader.rs
//	this file originally belonged to baseOS project
//		an OS template on which to build

//  put bootloader requests here

use crate::limine::BaseRevision;
use crate::limine::request::{FramebufferRequest, StackSizeRequest};



/*#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();*/

#[unsafe(link_section = ".requests")]
pub static STACK_SIZE: StackSizeRequest = StackSizeRequest::new().with_size(64*1024);

#[unsafe(link_section = ".requests")]
pub static revision: BaseRevision = BaseRevision::with_revision(3);


#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::with_revision(4);


