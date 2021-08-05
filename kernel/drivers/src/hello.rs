use kernel::syscall::{SyscallDriver,CommandReturn};
use kernel::process::{Error,ProcessId};
use kernel::{ErrorCode,debug};
use core::cell::Cell;
pub const DRIVER_NUM: usize =0xa0000;

pub struct Hello{
	n:Cell<u32>,
}

impl Hello{	
	pub fn new()->Hello{
		Hello{
			n: Cell::new(1)
		}
	}
}

impl SyscallDriver for Hello{
	
	fn allocate_grant(&self, _processid: ProcessId) -> Result<(),Error> {
		Ok(())
	}
	fn command(
        &self,
        command_num: usize,
        r2: usize,
        r3: usize,
        process_id: ProcessId,
    ) -> CommandReturn {
        	match command_num {
        		0 => CommandReturn::success(),
        		1 =>{
        			debug!("Hello n is {}",self.n.get());
        			CommandReturn::success()
        		}
        		2 =>{
        			self.n.set (self.n.get() +1);
        			CommandReturn::success()
        		}
        		3=>{
        			if self.n.get() >0{
        				self.n.set(self.n.get()-1);
        				CommandReturn::success()
        			}else{
        				CommandReturn::failure(ErrorCode::INVAL)
        			}
			}
			4=>{
				self.n.set(r2 as u32);
				CommandReturn::success()
			}
        		_ => CommandReturn::failure(ErrorCode::NOSUPPORT)
        	}
        }
}
