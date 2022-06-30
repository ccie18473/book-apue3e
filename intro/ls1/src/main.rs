#![feature(cstr_from_bytes_until_nul)]

use libc::*;
use std::ffi::CStr;
use std::{env, mem, process};

fn main() {
    // need one argument, the directory name
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: ls directory_name");
        process::exit(1);
    }

    unsafe {
        // let's open the diretory
        let dp = opendir(args[1].as_ptr() as *const c_char);
        // exit if we can't open
        if dp.is_null() {
            println!("can't open {}", args[1]);
            process::exit(1);
        }
        // let's read the directory info
        let mut dirp = readdir(dp);

        // loop until the end of the dirent struct
        while !dirp.is_null() {
            // convert from i8 to u8 the file name
            let stream = mem::transmute::<[i8; 256], [u8; 256]>((*dirp).d_name);
            // get file name bytes until null
            let cstr = CStr::from_bytes_until_nul(&stream);
            // let's print the file name
            match cstr {
                Ok(cstr) => println!("{:?}", cstr),
                Err(err) => panic!("{}", err),
            }
            dirp = readdir(dp);
        }

        closedir(dp);
    }
}

/*

#include "apue.h"
#include <dirent.h>

int
main(int argc, char *argv[])
{
	DIR				*dp;
	struct dirent	*dirp;

	if (argc != 2)
		err_quit("usage: ls directory_name");

	if ((dp = opendir(argv[1])) == NULL)
		err_sys("can't open %s", argv[1]);
	
	dirp = readdir(dp);

	while ((dirp = readdir(dp)) != NULL)
		printf("%s\n", dirp->d_name);

	closedir(dp);
	exit(0);
}

*/