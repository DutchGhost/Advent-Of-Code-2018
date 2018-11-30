import os
from subprocess import run

if __name__ == '__main__':
    for day in range(1, 26):
        if day < 10:
            day = "0{}".format(day)
        for part in range(0, 2):
            if part == 0:
                bin_name = "day{}".format(day)
            else:
                bin_name = "day{}_2".format(day)
            run(["cargo", "new", bin_name])
            