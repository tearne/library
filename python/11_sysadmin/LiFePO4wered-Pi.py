#!/usr/bin/python

import subprocess
import sys
import os
import pexpect
import getpass
import importlib
import site

def main():
    tempfile = ensure_import("tempfile")

    with tempfile.TemporaryDirectory() as temp_dir:
        print(temp_dir)
        run("git clone https://github.com/xorbit/LiFePO4wered-Pi.git", cwd=temp_dir)
        cwd = os.path.join(temp_dir, "LiFePO4wered-Pi")
        run("make all", cwd=cwd)
        sudo("sudo make user-install", password.get(), cwd=cwd)

    print("Installed.  Get current battery voltage:")
    sudo("lifepo4wered-cli get vbat", password.get(), 5)


#
# Setup
#

class Password:
    def __init__(self):
        self.p = None

    def get(self):
        if self.p: return self.p
        else:
            self.p = getpass.getpass()
            return self.p


password = Password()

def run(cmd, cwd=None):
    subprocess.run(
        cmd,
        shell=True,
        stdout=sys.stdout,
        stderr=sys.stderr,
        encoding='utf8',
        cwd=cwd
    )

def sudo(cmd, password, timeout = -1, cwd=None):
    child = pexpect.spawnu(f"sudo {cmd}", cwd=cwd)
    child.logfile_read=sys.stdout
    options = ['password', pexpect.TIMEOUT, pexpect.EOF]
    index = child.expect(options, timeout = 1)
    if index > 0:
        print(f"Error waiting for password prompt: {options[index]} - {child.before.decode()}")
        sys.exit(1)
    
    child.sendline(password)
    
    options = [pexpect.EOF, 'try again', pexpect.TIMEOUT]
    index = child.expect(options, timeout=timeout)
    if index == 0:
        return
    elif index == 1:
        print(f"Authentication failure: {options[index]}")
        sys.exit(1)
    else:
        print(f"Command failure: {options[index]}")
        sys.exit(1)

sudo("apt install -y python3-pip", password.get(), 60)

def ensure_import(package_name):
    try:
        pkg = importlib.import_module(package_name)
        print(f"Dependency '{package_name}' already installed")
        return pkg
    except ImportError as err:
        print(f"Dependency {package_name} not installed yet: {err}")
        subprocess.check_call([sys.executable, '-m', 'pip', 'install', package_name])
        importlib.reload(site)  # reloads sys.path
        importlib.invalidate_caches()
        return importlib.import_module(package_name)


if __name__ == '__main__':
    main()