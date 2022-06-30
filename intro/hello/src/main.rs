use libc::*;

fn main() {
    unsafe {
        println!("hello world from process ID {}", getpid());
    }
}

/*

#include "apue.h"

int
main(void)
{
	printf("hello world from process ID %ld\n", (long)getpid());
	exit(0);
}

*/