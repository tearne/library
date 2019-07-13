name := "flink"

version := "0.0.1-SNAPSHOT"

scalaVersion := "2.12.8"

libraryDependencies ++= Seq(
  "org.apache.flink" %% "flink-scala" % "1.7.2",
  "org.apache.flink" %% "flink-clients" % "1.7.2"
)