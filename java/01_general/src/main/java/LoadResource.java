import org.apache.commons.io.FileUtils;

import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.charset.Charset;
import java.nio.file.Path;
import java.nio.file.Paths;

/**
 * Loading a configs.json from resources.  Tested to work within both
 *
 * 1) IntelliJ IDEA
 *
 * AND
 *
 * 2) On the command line with jars and configs.json in a lib dir: java -cp "lib/*:lib" LoadResource
 * where lib contains jars and resources
 *
 *
 * To build far jar just "clean" and "package" and jar will appear in target
 */

public class LoadResource {
    public static void main(String[] args) throws URISyntaxException, IOException {

        String resourceName = "my.resource";

        System.out.println("Attempting to load resource: " + resourceName);
        String configPathStr = ClassLoader.getSystemResource(resourceName).toURI().getPath();
        System.out.println("Found path: " + configPathStr);

        Path configPath = Paths.get(configPathStr);

        System.out.println("----- resource ----");
        FileUtils.readLines(configPath.toFile(), Charset.defaultCharset())
                .forEach(System.out::println);
    }
}