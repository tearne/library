import socket
import os, os.path

sockfile = '/tmp/socket.sock'

if os.path.exists(sockfile):
    os.remove(sockfile)

sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.bind(sockfile)
sock.listen(1)

print("Listening on Unix Domain Socket:", sockfile)

(connect, address) = sock.accept()

while True:
    length = int.from_bytes(connect.recv(4), byteorder='big')

    request = bytearray()
    while len(request) < length:
        packet = connect.recv(4096)
        request += packet

    respLength = length.to_bytes(4,  byteorder='big')
    response = request

    connect.sendall(respLength + response)