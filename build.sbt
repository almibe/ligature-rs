import sbt.Keys.testFrameworks

ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "dev.ligature"
ThisBuild / organizationName := "Ligature"
ThisBuild / scalaVersion     := "2.13.3"

lazy val root = project
  .in(file("."))
  .settings(
    name := "ligature",
    libraryDependencies += "io.monix" %% "monix" % "3.3.0",///.withDottyCompat(scalaVersion.value),
    libraryDependencies += "org.scalameta" %% "munit" % "0.7.12" % Test,
    testFrameworks += new TestFramework("munit.Framework")
  )
