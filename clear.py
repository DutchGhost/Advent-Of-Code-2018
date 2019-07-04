import os
import subprocess

for item in os.listdir(os.getcwd()):
    try:
        os.chdir(item)
        print("[*] cleaning {item}", item)
        subprocess.call(["cargo", "clean"])
        print("[*] cleaned {item}", item)
        os.chdir('..')
    except Exception as e:
        print("[*] could not clear {item}: {e}", item, e)