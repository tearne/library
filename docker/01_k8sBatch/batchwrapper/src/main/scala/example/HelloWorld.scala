package example

import java.nio.file.{Files, Paths}

import org.apache.logging.log4j.LogManager

/**
 * Pretend user application, to be invoked once per job, with arguments to configure the job
 */
object HelloWorld extends App {
  val log = LogManager.getLogger(this.getClass)

  log.info(s"Scala application args: ${args.toSeq}")

  val config =
    if (args.size == 1) {
      val arg = args(0)
      if (Files.exists(Paths.get(arg))) Files.readString(Paths.get(arg))
      else s"Command line arg: $arg"
    }
   else if(args.size == 0) "No config supplied"
   else s"Multiple command line args: ${args.toSeq}"

  val outDir = Paths.get("output")
  if(Files.notExists(outDir))
    Files.createDirectories(outDir)

  Files.writeString(outDir.resolve("inputConfig"), config)
}