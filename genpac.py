
import sys
import os
def main():
    file_name = sys.argv[1]
    os.system("svd2rust -i" + file_name)
    os.system("rm -rf src/")
    os.system("form -i lib.rs -o src/ && rm lib.rs")
    os.system("cargo fmt")