import sbt.Keys.testFrameworks

ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "dev.ligature"
ThisBuild / organizationName := "Ligature"

lazy val root = project
  .in(file("."))
  .settings(
    name := "ligature",
    scalaVersion := "3.0.0-M2",
    libraryDependencies += "dev.ligature" %% "iris" % "0.1.0-SNAPSHOT",
    libraryDependencies += ("io.monix" %% "monix" % "3.3.0").withDottyCompat(scalaVersion.value),
    libraryDependencies += "org.scalameta" %% "munit" % "0.7.19" % Test,
    testFrameworks += new TestFramework("munit.Framework")
  )
