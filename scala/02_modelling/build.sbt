enablePlugins(JavaAppPackaging)

val commonsMath3 = "org.apache.commons" % "commons-math3" % "3.2"

val scalameter = "com.storm-enroute" %% "scalameter" % "0.17" % Test
//val scalaTest = "org.scalatest" %% "scalatest" % "3.0.4" % "test"
//val playJson = "com.typesafe.play" %% "play-json" % "2.6.7"
val samplerCore = "org.tearne" %% "sampler-core" % "0.3.16"
//val samplerAbc = "org.tearne" %% "sampler-abc" % "0.3.16"
//val logbackClassic = "ch.qos.logback" % "logback-classic" % "1.1.1"
val commonsIo = "commons-io" % "commons-io" % "2.4"

lazy val root = (project in file(".")).
    settings(
      inThisBuild(List(
        scalaVersion := "2.12.8",
        version := "0.1"
      )),
      name := "examples",
//      resolvers += Resolver.bintrayRepo("tearne", "maven"),
      libraryDependencies ++= Seq(
        commonsMath3,
        samplerCore,
        commonsIo
        commonsMath3,
        scalameter
      )
    )