#!/usr/bin/env -S uv --quiet run --script
# /// script
# requires-python = ">=3.13"
# dependencies = [
#   "rich",
#   "requests",
# ]
# ///

import os
import requests
from rich.console import Console

os.environ['REQUESTS_CA_BUNDLE'] = "/etc/ssl/certs/ca-certificates.crt"
console = Console()

if "VIRTUAL_ENV" not in os.environ:
    exit("Run this script from a venv to avoid polluting your system.")
else:
    console.print(f"You're running from venv: {os.environ["VIRTUAL_ENV"]}")

console.print("\n[bold]Languages\n")

console.rule("Python", style="blue")
console.print('''Python is a general-purpose, dynamicly typed, \
object-oriented programming language that emphasizes productivity \
and readability.\n''')

console.rule("Rust", style="red")
console.print('''Rust is a multi-paradigm, general-purpose programming \
language that emphasizes performance, type safety, and concurrency.\n''')

resp = requests.get("https://peps.python.org/api/peps.json")
data = resp.json()
console.rule("The first 10 PEPs", style="green")
console.print([(k, v["title"]) for k, v in data.items()][:10])

