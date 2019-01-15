package bridge;

import java.io.*;
import java.net.ConnectException;
import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.nio.channels.SocketChannel;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.TimeUnit;

public class TCPBridge extends Bridge {

    private SocketChannel socketChannel;
    private ByteBuffer bb;

    public TCPBridge(Path script, int port, int bufferSize) {
        assert (Files.exists(script));

        bb = ByteBuffer.allocateDirect(bufferSize);

        startServer(
                "python-TCP",
                "python3", "-u", script.toAbsolutePath().toString()
        );

        openSocket(port);
    }

    private void openSocket(int port) {
        int triesLeft = 20;
        try {
            while(true){
                try{
                    socketChannel = SocketChannel.open();
                    socketChannel.connect(new InetSocketAddress("localhost", port));
                    socketChannel.finishConnect();
                    log.info("Connection established");
                    break;
                } catch (ConnectException e) {
                    log.info("Not ready ({} attempts left)", triesLeft);
                    if(triesLeft < 1) throw e;
                }
                triesLeft--;

                TimeUnit.MILLISECONDS.sleep(1);
            }
        } catch (IOException | InterruptedException e) {
            reportWithCleanupTime(e);
        }
    }

    public byte[] ask(byte[] msg) {
        try {
            bb.clear();
            bb.limit(4 + msg.length);
            bb.putInt(msg.length);
            bb.put(msg);
            bb.flip();
            while(bb.hasRemaining())
                socketChannel.write(bb);

            readLength(4);
            int length = bb.getInt();

            readLength(length);
        } catch (IOException e) {
            reportWithCleanupTime(e);
        }

        byte[] arr = new byte[bb.remaining()];
        bb.get(arr);
        return arr;
    }

    private void readLength(int length) throws IOException {
        bb.clear();
        bb.limit(length);
        while(bb.remaining() > 0 && socketChannel.read(bb) > 0);
        if (bb.remaining() > 0) throw new EOFException();
        bb.flip();
    }

    public void terminate(){
        try {
            socketChannel.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        super.terminate();
    }
}
