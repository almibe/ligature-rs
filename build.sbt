import sbt.Keys.testFrameworks

ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "dev.ligature"
ThisBuild / organizationName := "Ligature"
ThisBuild / name             := "ligature"
ThisBuild / scalaVersion     := "0.26.0"

lazy val root = project
  .in(file("."))
  .settings(
    libraryDependencies += "org.scalameta" %% "munit" % "0.7.12" % Test,
    testFrameworks += new TestFramework("munit.Framework")
  )
