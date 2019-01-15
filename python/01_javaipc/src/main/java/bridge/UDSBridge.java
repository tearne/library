package bridge;

import jnr.unixsocket.UnixSocketAddress;
import jnr.unixsocket.UnixSocketChannel;

import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.channels.Channels;
import java.nio.file.Path;
import java.util.concurrent.TimeUnit;

public class UDSBridge extends Bridge {

    private UnixSocketChannel channel;
    private DataOutputStream dOut;
    private DataInputStream dIn;

    public UDSBridge(Path script, Path sockFile){
        startServer(
                "python-UDS",
                "python3", "-u", script.toAbsolutePath().toString()
        );

        openSocket(sockFile);
    }

    private void openSocket(Path sockFile) {
        int triesLeft = 20;
        try {
            while(true){
                try{
                    UnixSocketAddress addr = new UnixSocketAddress(sockFile.toFile());
                    channel = UnixSocketChannel.open(addr);
                    dOut = new DataOutputStream(Channels.newOutputStream(channel));
                    dIn = new DataInputStream(Channels.newInputStream(channel));

                    log.info("Connection established");
                    break;
                } catch (IOException e) {
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
        byte[] response = new byte[msg.length];

        try {
            dOut.write(ByteBuffer.allocate(4).putInt(msg.length).array());
            dOut.write(msg);
            dOut.flush();

            byte[] b = new byte[4];
            dIn.readFully(b);
            int expectedLength = ByteBuffer.wrap(b).getInt();
            if(expectedLength != msg.length)
                throw new RuntimeException("Response was wrong length, declared "+expectedLength+", got "+msg.length);

            dIn.readFully(response);
        } catch (IOException e) {
            reportWithCleanupTime(e);
        }

        return response;
    }

    public void terminate(){
        try {
            dIn.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        try {
            dOut.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        try {
            channel.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        super.terminate();
    }
}
