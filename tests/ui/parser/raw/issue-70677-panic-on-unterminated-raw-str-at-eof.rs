// This won't actually panic because of the error comment -- the `"` needs to be
// the last byte in the file (including not having a trailing newline)
// Prior to the fix you get the error: 'expected item, found `r" ...`'
// because the string being unterminated wasn't properly detected.
<<<<<<< HEAD
r" //~ unterminated raw string

// ferrocene-annotations: fls_usr6iuwpwqqh
// Raw String Literals
=======
r" //~ ERROR unterminated raw string
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
