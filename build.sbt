import Dependencies._

ThisBuild / scalaVersion     := "2.13.2"
ThisBuild / version          := "0.1.0-SNAPSHOT"
ThisBuild / organization     := "dev.ligature"
ThisBuild / organizationName := "Ligature"

scalacOptions ++= Seq("-deprecation", "-feature")

lazy val root = (project in file("."))
  .settings(
    name := "ligature",
    libraryDependencies += "io.monix" %% "monix" % "3.2.2",
    libraryDependencies += scalaTest % Test
  )
