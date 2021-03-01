pub fn run() {
    print!("===Std misc===");
    // Thread
    let mut children = vec![];

    for i in 0..10 {
        children.push(std::thread::spawn(move || {
            println!("this is thread number: {}", i);
        }));
    }
    for child in children {
        let _ = child.join();
    }

    // Channels
    // Channels allow a unidirectional flow of infomation between two end-points: the Sender and the Receiver.
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;

    static NTHREADS: i32 = 3;

    {
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut children = Vec::new();

        for id in 0..NTHREADS {
            let thread_tx = tx.clone();

            children.push(thread::spawn(move || {
                thread_tx.send(id).unwrap();
                println!("thread {} finished", id);
            }));
        }

        let mut ids = Vec::with_capacity(NTHREADS as usize);
        for _ in 0..NTHREADS {
            ids.push(rx.recv().unwrap());
        }

        for child in children {
            child.join().expect("oops!");
        }

        println!("{:?}", ids);
    }

    // Path
    {
        use std::path::Path;

        let path = Path::new(".");

        let display = path.display();

        println!("display: {}", display);

        let new_path = path.join("a").join("b");

        match new_path.to_str() {
            None => println!("new path is not a valid UTF-8 sequence"),
            Some(s) => println!("new path is: {}", s),
        }
    }

    // FILE I/O
    {
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let path = Path::new("hello.txt");

        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {} {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {} {}", display, why),
            Ok(_) => println!("{} contains: \n {}", display, s),
        }

        println!("s {}", s);
    }

    // Create
    {
        static LOREM_IPSUM: &str =
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
        tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
        quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
        consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
        cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
        proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
        ";
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        let path = Path::new("lorem_ipsum.txt");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {} {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    // read lines
    {
        use std::fs::File;
        use std::io::{self, BufRead};
        use std::path::Path;

        fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where
            P: AsRef<Path>,
        {
            let file = File::open(filename)?;
            Ok(io::BufReader::new(file).lines())
        }

        {
            if let Ok(lines) = read_lines("./hosts") {
                for line in lines {
                    if let Ok(ip) = line {
                        println!("{}", ip);
                    }
                }
            }
        }
    }

    // Child processes

    {
        // The process::Output struct represents the output of a finished child process,
        // and the process::Command struct is a process builder

        use std::process::Command;

        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .unwrap_or_else(|e| {
                panic!("failed to execute process: {}", e);
            });

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            println!("rustc successed and stdout was:\n{}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);
            println!("rustc failed and stderr was:\n{}", s);
        }
    }

    // Pipes
    {
        use std::io::prelude::*;
        use std::process::{Command, Stdio};

        static PANGRAM: &'static str = "the quick brown fox jumped over the layz dog\n";

        {
            let process = match Command::new("wc")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
            {
                Err(why) => panic!("couldn't spawn wc: {}", why),
                Ok(process) => process,
            };

            match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
                Err(why) => panic!("couldn't write to wc stdin: {}", why),
                Ok(_) => println!("sent pangram to wc"),
            }

            let mut s = String::new();
            match process.stdout.unwrap().read_to_string(&mut s) {
                Err(why) => panic!("couldn't read wc stdout: {}", why),
                Ok(_) => print!("wc responded with:\n{}", s),
            }
        }
    }

    // Wait
    {
        use std::process::Command;

        {
            println!("start======",);
            let mut child = Command::new("sleep").arg("5").spawn().unwrap();
            let _result = child.wait().unwrap();

            println!("reached end of main");
        }
    }

    // Filesystem
    {
        use std::fs;
        use std::fs::{File, OpenOptions};
        use std::io;
        use std::io::prelude::*;
        use std::os::unix;
        use std::path::Path;

        {
            fn cat(path: &Path) -> io::Result<String> {
                let mut f = File::open(path)?;
                let mut s = String::new();

                match f.read_to_string(&mut s) {
                    Ok(_) => Ok(s),
                    Err(e) => Err(e),
                }
            }

            fn echo(s: &str, path: &Path) -> io::Result<()> {
                let mut f = File::create(path)?;
                f.write_all(s.as_bytes())
            }

            fn touch(path: &Path) -> io::Result<()> {
                match OpenOptions::new().create(true).write(true).open(path) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }

            {
                println!("mkdir a");

                match fs::create_dir("a") {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(_) => {}
                }

                println!("echo hello > a/b.txt");
                echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });

                println!("mkdir -p a/c/d");
                fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });

                println!("touch a/c/e.txt");

                touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });

                println!("ln -s ../b.txt a/c/b.txt");
                if cfg!(target_family = "unix") {
                    unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
                        println!("! {:?}", why.kind());
                    });
                }

                println!("cat a/c/b.txt");

                match cat(&Path::new("a/c/b.txt")) {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(s) => println!("> {}", s),
                }

                println!("ls a");

                match fs::read_dir("a") {
                    Err(why) => println!("! {:?}", why.kind()),
                    Ok(paths) => {
                        for path in paths {
                            println!("> {:?}", path.unwrap().path());
                        }
                    }
                }

                println!("rm a/c/e.txt");

                fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });

                println!("rmdir a/c/d",);
                fs::remove_dir("a/c/d").unwrap_or_else(|why| {
                    println!("! {:?}", why.kind());
                });
            }
        }
    }

    // Program arguments
    {
        use std::env;
        {
            let args: Vec<String> = env::args().collect();
            println!("My path is {}", args[0]);

            println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
        }
    }
    // Foreign Function Interface
    {
        use std::fmt;

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct Complex {
            re: f32,
            im: f32,
        }

        impl fmt::Debug for Complex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                if self.im < 0. {
                    write!(f, "{}-{}i", self.re, -self.im)
                } else {
                    write!(f, "{}+{}i", self.re, self.im)
                }
            }
        }

        #[link(name = "m")]
        extern "C" {
            fn csqrtf(z: Complex) -> Complex;
            fn ccosf(z: Complex) -> Complex;
        }

        fn cos(z: Complex) -> Complex {
            unsafe { ccosf(z) }
        }

        {
            let z = Complex { re: -1., im: 0. };

            let z_sqrt = unsafe { csqrtf(z) };

            println!("the square root of {:?} is {:?}", z, z_sqrt);
            println!("cos({:?}) = {:?}", z, cos(z));
        }
    }
}
