
organization := "org.tearne"
name := "spark-submit"
version := "0.0.1"
scalaVersion := "2.11.12"

enablePlugins(JavaAppPackaging)

val jacksonMess = Seq(
  "com.fasterxml.jackson.core" % "jackson-core" % "2.8.9",
  "com.fasterxml.jackson.module" %% "jackson-module-scala" % "2.8.9"
)

libraryDependencies ++= Seq(
  "commons-io" % "commons-io" % "2.4",
  "org.apache.spark" %% "spark-core" % "2.3.0",
  "org.apache.spark" %% "spark-sql" % "2.3.0",
  "com.typesafe.play" %% "play-json" % "2.6.9",
  "org.apache.hadoop" % "hadoop-aws" % "2.8.3",
  "org.apache.hadoop" % "hadoop-client" % "2.8.3",
  "com.amazonaws" % "aws-java-sdk" % "1.11.289",
  "org.slf4j" % "slf4j-api" % "1.7.25"
) ++ jacksonMess