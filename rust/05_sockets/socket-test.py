import socket
import os, os.path
import struct

sockfile = '/tmp/socket.sock'

if os.path.exists(sockfile):
    os.remove(sockfile)

sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.bind(sockfile)
sock.listen(1)

print("Listening on Unix Domain Socket:", sockfile)

(connect, address) = sock.accept()

length = int.from_bytes(connect.recv(2), byteorder='big')
print("Data length: ",length)

data = []
while len(data) < length:
    new_int = int.from_bytes(connect.recv(2), byteorder='big')
    data.append(new_int)
    print("Received ",new_int)

print(data)

response = list(map(lambda x: 2*x, data))
print("sending", response)
for value in response:
    print("sending ", value, " -> ", value.to_bytes(2,  byteorder='big'))
    connect.sendall(value.to_bytes(2,  byteorder='big'))

print("done")