import org.apache.spark.sql.{Dataset, SparkSession}

object SparkWordCount {
  def main(args: Array[String]) {
    val session: SparkSession = SparkSession.builder
        .appName("WordCount")
        .getOrCreate()

    import session.implicits._

    // get threshold
    val threshold = args(1).toInt

    // read in text file and split each document into words
    val tokenized: Dataset[String] = session.read.text(args(0)).as[String].flatMap(_.split(" "))

    // count the occurrence of each word
    val wordCounts: Dataset[(String, Int)] = tokenized.groupByKey(identity).mapGroups{case (key, valuesIt) => key -> valuesIt.size}

    System.out.println("Counted: "+wordCounts.count+" words")

    wordCounts.write.csv("s3://some-bucket/counts.csv")
  }
}