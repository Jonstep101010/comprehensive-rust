Rust has great support for calling functions through a foreign function interface (FFI).

We will use this to build a safe wrapper for the libc functions you would use from C to read the names of files in a directory.

You will want to consult the manual pages:

    opendir(3)
    readdir(3)
    closedir(3)

You will also want to browse the std::ffi module. There you find a number of string types which you need for the exercise:
| Types | Encoding | Use |
|-------|----------|-----|
| str and String | UTF-8 | Text processing in Rust |
| CStr and CString | NUL-terminated | Communicating with C functions |
| OsStr and OsString | OS-specific | Communicating with the OS |

You will convert between all these types:

- &str to CString: you need to allocate space for a trailing \0 character,
- CString to *const i8: you need a pointer to call C functions,
- *const i8 to &CStr: you need something which can find the trailing \0 character,
- &CStr to &[u8]: a slice of bytes is the universal interface for “some unknown data”,
- &[u8] to &OsStr: &OsStr is a step towards OsString, use OsStrExt to create it,
- &OsStr to OsString: you need to clone the data in &OsStr to be able to return it and call readdir again.

The Nomicon also has a very useful chapter about FFI.