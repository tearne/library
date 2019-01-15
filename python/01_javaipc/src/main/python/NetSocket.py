import socket

port = 6789

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
sock.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
sock.bind(('localhost', port))
sock.listen(1)

print("Listening on port:", port)

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


