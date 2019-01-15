package bridge;

import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.net.ConnectException;
import java.net.Socket;
import java.nio.ByteBuffer;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.concurrent.TimeUnit;

public class LegacyTCPBridge extends Bridge {

    private Socket socket;
    private DataOutputStream dOut;
    private DataInputStream dIn;

    public LegacyTCPBridge(Path script, int port) {
        assert (Files.exists(script));

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
                    socket = new Socket("localhost", port);
                    socket.setTcpNoDelay(true);
                    dOut = new DataOutputStream(socket.getOutputStream());
                    dOut.writeInt(0);
                    dIn = new DataInputStream(socket.getInputStream());
                    if(dIn.readInt() != 0)
                        throw new ConnectException("Unexpected item in bagging area");
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
        byte[] response = new byte[msg.length];

        try {
            dOut.write(ByteBuffer.allocate(4).putInt(msg.length).array());
            dOut.write(msg);
            dOut.flush();

            byte[] b = new byte[4];
            dIn.read(b);

            int expectedLength = ByteBuffer.wrap(b).getInt();
            if(expectedLength != msg.length)
                throw new RuntimeException("Response declared unexpected length ("+expectedLength+"), expected "+msg.length);

            dIn.readFully(response);
            if(dIn.available() != 0)
                throw new RuntimeException("Data still to be read?!");
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
            socket.close();
        } catch (IOException e) {
            e.printStackTrace();
        }
        super.terminate();
    }
}
