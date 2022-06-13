#!/usr/bin/env python

# https://github.com/xorbit/LiFePO4wered-Pi

import subprocess
import sys
import os

def ensure_import(package_name):
    try:
        pkg = __import__(package_name, globals(), locals(), [], 0)
        print(f"Dependency '{package_name}' already installed")
        return pkg
    except ImportError as err:
        print(f"Dependency {package_name} not installed yet: {err}")
        subprocess.check_call([sys.executable, '-m', 'pip', 'install', package_name])
        return __import__(package_name, globals(), locals(), [], 0)

pexpect = ensure_import("pexpect")
getpass = ensure_import("getpass")
tempfile = ensure_import("tempfile")

def sudo(cmd, timeout = -1, cwd=None):
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


def run(cmd, cwd=None):
    subprocess.run(
        cmd,
        shell=True,
        stdout=sys.stdout,
        stderr=sys.stderr,
        encoding='utf8',
        cwd=cwd
    )

password = getpass.getpass()

sudo("apt update", 30)
sudo("sudo apt-get -y install build-essential git libsystemd-dev", 60)

with tempfile.TemporaryDirectory() as temp_dir:
    print(temp_dir)
    run("git clone https://github.com/xorbit/LiFePO4wered-Pi.git", cwd=temp_dir)
    cwd = os.path.join(temp_dir, "LiFePO4wered-Pi")
    run("make all", cwd=cwd)
    sudo("sudo make user-install", cwd=cwd)

print("Installed.  Current battery voltage:")
sudo("lifepo4wered-cli get vbat", 5)
