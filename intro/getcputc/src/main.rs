use libc::*;
use libc_stdhandle::*;

fn main() {
    unsafe {
        let mut c: c_int = fgetc(stdin());

        while c != EOF {
            if fputc(c, stdout()) == EOF {
                println!("output error\n");
            }
            c = fgetc(stdin());
        }
        if ferror(stdin()) != 0 {
            println!("input error\n");
        }
        exit(0);
    }
}

/*

#include "apue.h"

int
main(void)
{
	int		c;

	while ((c = getc(stdin)) != EOF)
		if (putc(c, stdout) == EOF)
			err_sys("output error");

	if (ferror(stdin))
		err_sys("input error");

	exit(0);
}

*/