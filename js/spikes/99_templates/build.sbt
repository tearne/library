name := "VisTemplates"

version := "0.1.0-SNAPSHOT"

scalaVersion := "2.12.3"

resolvers ++= Seq(
  Resolver.mavenLocal,
  Resolver.bintrayRepo("tearne", "maven")
)

libraryDependencies ++= Seq(
  "commons-io" % "commons-io" % "2.4",
  "org.tearne" %% "sampler-core" % "0.3.11",
  "com.typesafe.play" %% "play" % "2.6.6"
)
