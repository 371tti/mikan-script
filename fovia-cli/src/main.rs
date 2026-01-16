pub const LOGO: &'static str = "\x1b[1;32mFoVia ver. dev0\x1b[0m";


fn main() {
    println!("{}", LOGO);

    let mut args = std::env::args().skip(1);
    let command = match args.next() {
        Some(cmd) => cmd,
        None => {
            print_usage();
            std::process::exit(1);
        }
    };

    let target = args.next().unwrap_or_else(|| "main".to_string());

    let (input_fv, output_fvo) = normalize_paths(&target);

    match command.as_str() {
        "build" => {
            if !run_build(&input_fv, &output_fvo) {
                std::process::exit(1);
            }
        }
        "run" => {
            if !run_build(&input_fv, &output_fvo) {
                std::process::exit(1);
            }
            if !run_vm_with_file(&output_fvo) {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("unknown command: {command}");
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("usage:");
    eprintln!("  fv build [name|path]  # default: main.fv -> main.fvo");
    eprintln!("  fv run [name|path]    # default: build main.fv then run via fovia-vm");
}

fn normalize_paths(target: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut path = std::path::PathBuf::from(target);
    if path.is_relative() {
        path = cwd.join(path);
    }

    let input_fv = if path.extension().is_some() {
        path.clone()
    } else {
        path.with_extension("fv")
    };

    let output_fvo = if input_fv.extension().map(|e| e == "fvo").unwrap_or(false) {
        input_fv.clone()
    } else {
        input_fv.with_extension("fvo")
    };

    (input_fv, output_fvo)
}

fn run_build(input_fv: &std::path::Path, output_fvo: &std::path::Path) -> bool {
    let bin_dir = match std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())) {
        Some(dir) => dir,
        None => {
            eprintln!("failed to locate fv executable directory");
            return false;
        }
    };

    let fovia_c = bin_dir.join("fovia-c.exe");
    let fovia_vm = bin_dir.join("fovia-vm.exe");

    let start = std::time::Instant::now();
    let status = std::process::Command::new(&fovia_c)
        .arg(input_fv)
        .arg(output_fvo)
        .status();

    match status {
        Ok(status) if status.success() => {
            let elapsed = start.elapsed();
            eprintln!(
                "    Finished compile in {:.3}s",
                elapsed.as_secs_f64()
            );
            if !emit_executable(&fovia_vm, output_fvo) {
                return false;
            }
            true
        }
        Ok(status) => {
            eprintln!("build failed with status {status}");
            false
        }
        Err(err) => {
            eprintln!("failed to run fovia-c: {err}");
            false
        }
    }
}

fn run_vm_with_file(output_fvo: &std::path::Path) -> bool {
    let bin_dir = match std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.to_path_buf())) {
        Some(dir) => dir,
        None => {
            eprintln!("failed to locate fv executable directory");
            return false;
        }
    };

    let fovia_vm = bin_dir.join("fovia-vm.exe");
    let status = std::process::Command::new(&fovia_vm)
        .arg(output_fvo)
        .status();

    match status {
        Ok(status) if status.success() => true,
        Ok(status) => {
            eprintln!("fovia-vm exited with status {status}");
            false
        }
        Err(err) => {
            eprintln!("failed to run fovia-vm: {err}");
            false
        }
    }
}

fn emit_executable(vm_path: &std::path::Path, output_fvo: &std::path::Path) -> bool {
    let exe_path = output_fvo.with_extension("exe");
    match std::fs::copy(vm_path, &exe_path) {
        Ok(_) => true,
        Err(err) => {
            eprintln!("failed to create executable {}: {err}", exe_path.display());
            false
        }
    }
}

