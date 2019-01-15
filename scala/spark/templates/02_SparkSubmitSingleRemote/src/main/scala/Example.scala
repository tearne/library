import java.io.File
import java.nio.charset.Charset

import com.amazonaws.services.s3.AmazonS3ClientBuilder
import org.apache.commons.io.FileUtils
import org.apache.spark.sql.{Dataset, SparkSession}
import org.slf4j.LoggerFactory
import play.api.libs.json._

object Example extends App {

  val log = LoggerFactory.getLogger(this.getClass)

  val session: SparkSession = SparkSession.builder
    .master("local[*]")
    .appName("SparkSubmitExample")
    //.config("key", "value")
    .getOrCreate()

  import session.implicits._

  val inputFile = args(1)
  val bucketName = args(0)

  val root = s"s3a://$bucketName/"
  val outputFile = inputFile+".counts.json"

  val lines: Dataset[String] = session.read.text(root+inputFile).as[String]

  val wordCounts = lines
      .flatMap(_.split(' '))
      .groupByKey(identity _)
      .count
      .collect
      .toMap

  val json: String = Json.prettyPrint(Json.toJson(wordCounts))

  val file = File.createTempFile("sparkSubmitExample", "jsonDump")
  file.deleteOnExit()
  FileUtils.writeStringToFile(file, json, Charset.defaultCharset)
  log.info("Wrote temp results to: "+file)

  AmazonS3ClientBuilder.standard.build.putObject(bucketName, outputFile, file)
  log.info("Result uploaded to: "+root+outputFile)
  log.info("--== DONE ==--")
}
