val dottyVersion = "0.24.0-RC1"

lazy val root = project
  .in(file("."))
  .settings(
    name := "ligature",
    version := "0.1.0-SNAPSHOT",
    organization := "dev.ligature",

    scalaVersion := dottyVersion,

    resolvers += DefaultMavenRepository,

    libraryDependencies += "com.novocode" % "junit-interface" % "0.11" % "test",
    libraryDependencies += "dev.zio" % "zio-streams_2.13" % "1.0.0-RC20",
    libraryDependencies += "dev.zio" % "zio_2.13" % "1.0.0-RC20"
  )
