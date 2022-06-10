#!/usr/bin/env python

# https://github.com/xorbit/LiFePO4wered-Pi

import subprocess
import sys

def ensure_import(package_name):
    try:
        return __import__(package_name, globals(), locals(), [], 0)
    except ImportError as err:
        print(package_name, "is not installed yet -", err)
        subprocess.check_call([sys.executable, '-m', 'pip', 'install', package_name])
        return __import__(package_name, globals(), locals(), [], 0)

pexpect = ensure_import("pandas")


# import os
# import sys
# from subprocess import run

# def apt_install(pkgs):
#     cmd = ['pkexec', 'apt-get', 'install', '-y'] + pkgs
#     print('Running command: {}'.format(' '.join(cmd)))
#     result = run(
#         cmd,
#         stdout=sys.stdout,
#         stderr=sys.stderr,
#         encoding='utf8',
#         env={**os.environ, 'DEBIAN_FRONTEND': 'noninteractive'}
#     )
#     result.check_returncode()