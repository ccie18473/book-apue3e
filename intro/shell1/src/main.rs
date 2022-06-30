use libc::*;
use libc_stdhandle::*;
use std::io;
use std::io::prelude::*;
use std::{mem, ptr, str};

fn main() {
    unsafe {
        // buffer size
        const MAXLINE: usize = 4096;
        
        // c types
        let mut pid: pid_t;
        let status: c_int = 0;
        
        // buf is a pointer to cmd array
        let mut cmd = [0 as i8; MAXLINE];
        let buf: *mut c_char = cmd.as_mut_ptr();
        
        // null pointer
        let null_ptr: *mut c_char = ptr::null_mut();
        
        // print prompt
        print!("% ");
        let mut output = io::stdout();
        output.flush().unwrap();

        // let's read the commands
        while fgets(buf, MAXLINE as i32, stdin()) != null_ptr {
            // command size
            let cmd_size = strlen(buf) - 1;

            // check where is the command end
            if *buf.offset(cmd_size as isize) == '\n' as i8 {
                *buf.offset(cmd_size as isize) = 0;
            }

            pid = fork();

            // fork error
            if pid < 0 {
                println!("fork error");
            // child
            } else if pid == 0 { 
                // execute the command
                execlp(buf, buf, 0 as *const c_char);
                // print error if command is invalid
                let cmd_str = mem::transmute::<[i8; MAXLINE], [u8; MAXLINE]>(cmd);
                let cmd_str = str::from_utf8(&cmd_str[0..cmd_size]).unwrap();
                println!("couldn't execute: {:?}", cmd_str);
                exit(127);
            }

            // parent
            pid = waitpid(pid, status as *mut i32, 0);
            if pid < 0 {
                println!("waitpid error");
            }

            // print prompt
            print!("% ");
            let mut output = io::stdout();
            output.flush().unwrap();
        }

        exit(0);
    }
}


/*

#include "apue.h"
#include <sys/wait.h>

int
main(void)
{
	char	buf[MAXLINE];	/* from apue.h */
	pid_t	pid;
	int		status;

	printf("%% ");	/* print prompt (printf requires %% to print %) */
	while (fgets(buf, MAXLINE, stdin) != NULL) {
		if (buf[strlen(buf) - 1] == '\n')
			buf[strlen(buf) - 1] = 0; /* replace newline with null */

		if ((pid = fork()) < 0) {
			err_sys("fork error");
		} else if (pid == 0) {		/* child */
			execlp(buf, buf, (char *)0);
			err_ret("couldn't execute: %s", buf);
			exit(127);
		}

		/* parent */
		if ((pid = waitpid(pid, &status, 0)) < 0)
			err_sys("waitpid error");
		printf("%% ");
	}
	exit(0);
}

*/