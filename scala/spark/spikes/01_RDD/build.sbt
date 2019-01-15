name := "spark-test"

version := "0.0.1"

scalaVersion := "2.10.6"

libraryDependencies ++= Seq(
    "org.apache.spark" %% "spark-core" % "1.6.0",
    "org.slf4j" % "slf4j-api" % "1.7.12",
    "org.apache.commons" % "commons-lang3" % "3.4"
)

