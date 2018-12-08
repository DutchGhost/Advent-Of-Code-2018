from subprocess import run
if __name__ == '__main__':
    with open("Cargo.toml", 'r') as f:
        for line in f:
            if "day" in line:
                x = line.split("\t\t")
                x[0] = x[0].lstrip().rstrip().replace(r'"', "").replace(",", "")
                print("[*] compiling {}...".format(x[0]))
                run(["cargo", "build", "--release", "--bin", x[0]])