import sbt.Keys.testFrameworks

ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "dev.ligature"
ThisBuild / organizationName := "Ligature"

val dottyVersion = "3.0.0-M1"

lazy val root = project
  .in(file("."))
  .settings(
    name := "ligature",
    scalaVersion := dottyVersion,
    libraryDependencies += ("io.monix" %% "monix" % "3.3.0").withDottyCompat(scalaVersion.value),
    libraryDependencies += "org.scalameta" %% "munit" % "0.7.16" % Test,
    testFrameworks += new TestFramework("munit.Framework")
  )
