import argparse
import os
import subprocess
import shutil

def run_cmd(cmd, cwd=None, check=True):
    print("Running:", " ".join(cmd))
    res = subprocess.run(cmd, cwd=cwd)
    if check and res.returncode != 0:
        raise subprocess.CalledProcessError(res.returncode, cmd)
    return res.returncode

def main(): 
    parser = argparse.ArgumentParser(description="Make cs50rust psets.")

    parser.add_argument(
        "target",
        type=str,
        help="The target to build (e.g., pset4/filter-less/filter). Inside this directory, a rust/ folder and rust/<target>.rs file are expected. For instance, for pset4/filter-less/filter, the rust file should be at pset4/filter-less/rust/filter.rs",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Run check50 on the built target",
    )
    parser.add_argument(
        "--include",
        nargs="*",
        default=["helpers.c", "testing.c"],
        help="Additional files to make when building the target (e.g., --include helpers.c testing.c). Silently ignores missing files unless --strict-include is set.",
    )
    parser.add_argument(
        "--strict-include",
        action="store_true",
        help="If set, files not found in --include will cause an error.",
    )
    parser.add_argument(
        "--check-path",
        type=str,
        help="Custom path to checks directory (e.g. --check-path filter/less)",
    )
    parser.add_argument(
        "--cc", "--compiler",
        type=str,
        help="C compiler to use (e.g., clang, gcc). If not set, will try to find clang, then gcc, then default to 'gcc'.",
    )
    args = parser.parse_args()
    # Normalize target and paths
    target = args.target
    target = target.rstrip("/")
    target_dir = os.path.dirname(target) or "."
    basename = os.path.basename(target)

    rust_src = os.path.join(target_dir, "rust", f"{basename}.rs")
    stub_c = os.path.join(target_dir, f"{basename}.c")
    rust_lib = os.path.join(target_dir, f".librust_{basename}.a")

    try:
        # Build Rust staticlib
        if not os.path.exists(rust_src):
            raise SystemExit(f"Rust source not found: {rust_src}")

        rustc = shutil.which("rustc") or "rustc"
        if not args.check:
            run_cmd([
                rustc, "--crate-type", "staticlib", "--edition", "2021", os.path.relpath(rust_src, start=target_dir), "-o", os.path.basename(rust_lib)
            ], cwd=target_dir)

        # Prepare C compile/link
        cc = args.cc or shutil.which("clang") or shutil.which("gcc") or "gcc"

        c_files = []
        # main stub (.c required)
        if os.path.exists(stub_c):
            c_files.append(os.path.relpath(stub_c, start=target_dir))
        else:
            raise SystemExit(f"C stub not found: {stub_c}")

        # additional includes (paths are relative to target_dir if not absolute)
        if args.include:
            for inc in args.include:
                inc_path = inc if os.path.isabs(inc) else os.path.join(target_dir, inc)
                if not os.path.exists(inc_path):
                    if args.strict_include:
                        raise SystemExit(f"Included file not found: {inc_path}")
                    else:
                        continue
                
                # only pass .c sources to the compiler; ignore headers
                if inc_path.lower().endswith('.c'):
                    rel = os.path.relpath(inc_path, start=target_dir)
                    if rel not in c_files:
                        c_files.append(rel)
                else:
                    print(f"Ignoring non-C include (not passed to compiler): {inc_path}")

        # add the rust staticlib
        c_files.append(os.path.basename(rust_lib))

        # link flags similar to Makefile
        link_libs = ["-lpthread", "-ldl", "-lm", "-lcs50"]

        compile_cmd = [cc, "-ggdb3", "-gdwarf-4", "-O0", "-Qunused-arguments", "-std=c11", "-Wall", "-Werror", "-Wextra", "-Wno-gnu-folding-constant", "-Wno-sign-compare", "-Wno-unused-parameter", "-Wno-unused-variable", "-Wshadow", "-lm", "-o", os.path.join(os.getcwd(), target), *c_files, *link_libs]

        if not args.check:
            run_cmd(compile_cmd, cwd=target_dir)

            # cleanup rust lib
            os.remove(rust_lib)
        
            print(f"Built: {target}")

        else:
            # Determine check directory/name
            if args.check_path:
                check_name = args.check_path
                # verify local checks folder exists (helps catch typos)
                local_checks_path = os.path.join("checks", check_name)
                if not os.path.isdir(local_checks_path):
                    raise SystemExit(f"Local checks path not found: {local_checks_path}")
                # Run check50 with the remote slug from the local checks dir so .cs50.yml is used
                run_cmd(["check50", f"ivanharvard/cs50rust/main/checks/{check_name}", "--local"], cwd=local_checks_path)
            else:
                # derive check name from parent directory of target
                parent = os.path.basename(os.path.normpath(os.path.dirname(target)))
                local_checks_path = os.path.join("checks", parent)
                if os.path.isdir(local_checks_path):
                    # run from the local checks dir so local .cs50.yml enables check50
                    run_cmd(["check50", f"ivanharvard/cs50rust/main/checks/{parent}", "--local"], cwd=local_checks_path)
                else:
                    # fallback: run from the pset dir (original behavior)
                    check_dir = os.path.dirname(target) or "."
                    run_cmd(["check50", f"ivanharvard/cs50rust/main/checks/{parent}", "--local"], cwd=check_dir)

    except subprocess.CalledProcessError as e:
        raise SystemExit(f"Command failed ({e.returncode}): {' '.join(e.cmd)}")
    except Exception as e:
        raise SystemExit(str(e))


if __name__ == "__main__":
    main()

    

    



