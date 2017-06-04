# rustc-math-bench

A simple benchmark of how fast rust is at floating point math compared to C.

See this rust issue for a discussion on how architecture-specific optimization could be leveraged to make rustc produce binaries that are both fast (because they take advantage of all available instructions) and portable (because they figure out at runtime which instructions the machine supports):

https://github.com/rust-lang/rust/issues/42432

Contributing
------------

Feel free to send pull requests if you have a better idea how to benchmark at https://github.com/pedrocr/chimper
