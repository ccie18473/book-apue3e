use libc::*;

fn main() {
    unsafe {
        let buffsize = 4096;
        let mut vec = vec![0; buffsize];
        let buf: *mut c_void = vec.as_mut_ptr() as *mut c_void;
        
        let mut n: ssize_t = read(STDIN_FILENO, buf, buffsize);

        while n > 0 {
            if write(STDOUT_FILENO, buf, n as size_t) != n {
                println!("write error\n");
            }
            n = read(STDIN_FILENO, buf, buffsize);
        }
        if n < 0 {
            println!("read error\n");
        }
        exit(0);
    }
}

/*

#include "apue.h"

#define	BUFFSIZE	4096

int
main(void)
{
	int		n;
	char	buf[BUFFSIZE];

	while ((n = read(STDIN_FILENO, buf, BUFFSIZE)) > 0)
		if (write(STDOUT_FILENO, buf, n) != n)
			err_sys("write error");

	if (n < 0)
		err_sys("read error");

	exit(0);
}

*/