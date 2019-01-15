import org.apache.spark.SparkConf
import org.apache.spark.SparkContext
import org.apache.spark.rdd.RDD

object Join {
  def main(args: Array[String]){
    val sc = new SparkContext(new SparkConf().setAppName("Test").setMaster("local"))
    
    val words: RDD[String] = sc.textFile("/usr/share/dict/words")
    
    val indexedWords = words.zipWithIndex.map{ case (a,b) => (b,a) }
    
    val extras = sc.parallelize(Seq(999999999999l,88888888888l))
    val subsetIndexes = indexedWords.filter(_._1 % 1000 == 0).map(_._1) ++ extras
    println(subsetIndexes.count)
    
    val join = subsetIndexes.map(i => (i,i)).join(indexedWords)
    val leftOuterJoin = subsetIndexes.map(i => (i,i)).leftOuterJoin(indexedWords)
    println(s"join size = ${join.count}, leftOuterJoin size = ${leftOuterJoin.count}")
    println(leftOuterJoin.lookup(999999999999l))
  }
}