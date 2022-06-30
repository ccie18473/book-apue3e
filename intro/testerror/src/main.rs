use errno::*;
use libc::*;
use libc_stdhandle::*;
use std::env;
use std::ffi::CStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    unsafe {
        let error = CStr::from_ptr(strerror(EACCES));
        let format = format!("EACCES: {:?}\n", error);

        fprintf(stderr(), format.as_ptr() as *const i8);

        set_errno(errno::Errno(ENOENT));
        perror(args[0].as_ptr() as *const i8);
        exit(0);
    }
}

/*

#include "apue.h"
#include <errno.h>

int
main(int argc, char *argv[])
{
	fprintf(stderr, "EACCES: %s\n", strerror(EACCES));
	errno = ENOENT;
	perror(argv[0]);
	exit(0);
}

*/