package tmp

import org.apache.flink.api.java.utils.ParameterTool
import org.apache.flink.api.scala.{AggregateDataSet, ExecutionEnvironment}
import org.apache.flink.api.scala._

/**
  * Implements the "WordCount" program that computes a simple word occurrence histogram
  * over text files.
  *
  * The input is a plain text file with lines separated by newline characters.
  *
  * Usage:
  * {{{
  *   WordCount --input <path> --output <path>
  * }}}
  *
  * If no parameters are provided, the program is run with default data from
  * [[org.apache.flink.examples.java.wordcount.util.WordCountData]]
  *
  * This example shows how to:
  *
  *   - write a simple Flink program.
  *   - use Tuple data types.
  *   - write and use user-defined functions.
  *
  */
object WordCount {

  case class Word(str: String){
    def firstCharacter() = str.charAt(0)
  }

  def main(args: Array[String]) {

    val params: ParameterTool = ParameterTool.fromArgs(args)

    // set up execution environment
    val env = ExecutionEnvironment.getExecutionEnvironment

    // make parameters available in the web interface
    env.getConfig.setGlobalJobParameters(params)
    val text =
        env.readTextFile("/usr/share/dict/words")

    val counts = text.flatMap { _.toLowerCase.split("\\W+") filter { _.nonEmpty } }
        .map(str => Word(str))
        .groupBy(_.firstCharacter())
          .reduceGroup { items =>
            val itemsSeq = items.toSeq
            val count = itemsSeq.size
            val letter = itemsSeq.head.firstCharacter

            s"Words beginning with the letter $letter: $count"
          }

    if (params.has("output")) {
      counts.writeAsCsv(params.get("output"), "\n", " ")
      env.execute("Scala WordCount Example")
    } else {
      println("Printing result to stdout. Use --output to specify output path.")
      counts.print()
    }

  }
}
