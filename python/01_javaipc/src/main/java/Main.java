import bridge.*;
import org.apache.commons.lang3.time.StopWatch;

import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Random;

public class Main {

    public static void main(String[] args) throws InterruptedException {
        Bridge[] bridges = new Bridge[3];

        bridges[0] = new UDSBridge(
                Paths.get("src", "main", "python", "DomainSocket.py"),
                Paths.get("/", "tmp", "socket.sock")
        );

        bridges[1] = new LegacyTCPBridge(
                Paths.get("src", "main", "python", "NetSocket2.py"),
                6780
        );

        bridges[2] = new TCPBridge(
                Paths.get("src", "main", "python", "NetSocket.py"),
                6789  ,
                1024 * 1024
        );

        runTests(bridges);
    }

    private static void runTests(Bridge[] bridges) {
        Path path = Paths.get("out.csv");
        List<Integer> messageSizes = new ArrayList<>();
        messageSizes.add(5000);
        messageSizes.add(10000);
        messageSizes.add(20000);
        messageSizes.add(40000);
        messageSizes.add(80000);

        try (BufferedWriter writer = Files.newBufferedWriter(path)) {
            writer.write("Method , Time, Bytes");
            for (int i = 0; i < 100; i++) {
                for(int msgSize: messageSizes)
                    for(Bridge b : bridges) {
                        double time = run(b, msgSize);
                        String name = b.getClass().getSimpleName();
                        writer.newLine();
                        writer.write(name + "," + time + ", " + msgSize);
                        writer.flush();
                        System.out.println(i+", "+name+", "+time+", "+msgSize);
                    }
            }

            Path script = Paths.get("src", "main", "r", "plotDensity.sh");
            Path wd = Paths.get("").toAbsolutePath();
            System.out.println("Running in "+wd);
            ProcessBuilder pb = new ProcessBuilder(script.toAbsolutePath().toString());
            pb.directory(wd.toFile());
            pb.inheritIO();
            pb.start();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    private static double run(Bridge bridge, int msgSize) {
        byte[] msg = new byte[msgSize];
        Random r = new Random();
        for (int i = 0; i < msg.length; i++) msg[i] = (byte) (r.nextInt() % 100);

        byte[] resp = null;

        StopWatch sw = new StopWatch();
        sw.start();

        int reps = 1000;
        for(int i=0; i<1000; i++) {
            resp = bridge.ask(msg);
        }

        sw.stop();

        if(!Arrays.equals(resp,msg))
            throw new RuntimeException("Last response failed verification");

        return sw.getTime() / (double)reps;
    }
}
