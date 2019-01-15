package bridge;

import bridge.util.StreamGobbler;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;

public abstract class Bridge {

    protected Logger log = LoggerFactory.getLogger(this.getClass());
    private Process proc;
    private ExecutorService pool;

    protected void startServer(String threadName, String... cmd) {
        try {
            proc = new ProcessBuilder(cmd).start();

            pool = Executors.newSingleThreadExecutor(runnable -> {
                Thread thread = Executors.defaultThreadFactory().newThread(runnable);
                thread.setDaemon(true);
                thread.setName(threadName);
                return thread;
            });

            StreamGobbler outputGobbler = new StreamGobbler(proc.getInputStream(), log::info);
            StreamGobbler errorGobbler = new StreamGobbler(proc.getErrorStream(), log::error);

            pool.submit(outputGobbler);
            pool.submit(errorGobbler);
        } catch (IOException e){
            reportWithCleanupTime(e);
        }
    }

    public abstract byte[] ask(byte[] msg);
    public void terminate(){
        proc.destroy();
        pool.shutdown();
    }

    public void reportWithCleanupTime(Exception e){
        try {
            TimeUnit.SECONDS.sleep(1);
            throw new RuntimeException(e);
        } catch (InterruptedException e1) {
            throw new RuntimeException(e1);
        }
    }
}
