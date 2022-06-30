use libc::*;

fn main() {
    unsafe {
        println!("uid = {} gid = {}", getuid(), getgid());
    }
}

/*

#include "apue.h"

int
main(void)
{
	printf("uid = %d, gid = %d\n", getuid(), getgid());
	exit(0);
}

*/