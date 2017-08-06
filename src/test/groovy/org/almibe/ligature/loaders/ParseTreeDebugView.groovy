/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import javafx.animation.Animation
import javafx.animation.KeyFrame
import javafx.animation.Timeline
import javafx.application.Application
import javafx.application.Platform
import javafx.embed.swing.SwingNode
import javafx.event.ActionEvent
import javafx.geometry.Orientation
import javafx.scene.Scene
import javafx.scene.control.*
import javafx.scene.layout.BorderPane
import javafx.scene.layout.HBox
import javafx.stage.Stage
import javafx.util.Duration
import org.almibe.ligature.parser.ntriples.NTriplesLexer
import org.almibe.ligature.parser.ntriples.NTriplesParser
import org.almibe.ligature.parser.turtle.ModalTurtleLexer
import org.almibe.ligature.parser.turtle.Turtle
import org.antlr.v4.gui.TreeViewer
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

import javax.swing.*

class ParseTreeDebugView extends Application {
    final String defaultDocument = "<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> ."
    Stage stage
    Scene scene
    final BorderPane topControls = new BorderPane()
    final TextArea textArea = new TextArea(defaultDocument)
    final SwingNode swingNode = new SwingNode()
    final ScrollPane bottomScrollPane = new ScrollPane(swingNode)
    final SplitPane splitPane = new SplitPane(topControls, bottomScrollPane)
    final ToggleGroup radioGroup = new ToggleGroup()
    final RadioButton ntriplesButton = new RadioButton("N-Triples")
    final RadioButton turtleButton = new RadioButton("Turtle")
    final HBox buttonBox = new HBox(ntriplesButton, turtleButton)

    @Override
    void start(Stage primaryStage) throws Exception {
        this.stage = stage
        topControls.center = textArea
        topControls.bottom = buttonBox
        ntriplesButton.toggleGroup = radioGroup
        turtleButton.toggleGroup = radioGroup
        ntriplesButton.selected = true
        splitPane.orientation = Orientation.VERTICAL
        splitPane.setDividerPositions(0.2, 0.8)
        scene = new Scene(splitPane, 1200, 800)
        primaryStage.title = "Ligature Parser Debugger"
        primaryStage.scene = scene
        primaryStage.show()
        checkDisplay()

        Timeline timeline = new Timeline(new KeyFrame(
                Duration.seconds(1.0),
                { ActionEvent event ->
                    checkDisplay()
                }
        ))
        timeline.cycleCount = Animation.INDEFINITE
        timeline.play()
    }

    void checkDisplay() {
        Platform.runLater {
            TreeViewer component = ntriplesButton.selected ? createNTriplesTreeView(textArea.text) :
                createTurtleTreeView(textArea.text)
            updateSwingNode(component)
        }
    }

    void updateSwingNode(TreeViewer component) {
        SwingUtilities.invokeLater {
            component.scale = 1.5
            swingNode.content = component
        }
    }

    TreeViewer createTurtleTreeView(String text) {
        def stream = CharStreams.fromString(text)
        def lexer = new ModalTurtleLexer(stream)
        def tokenStream = new CommonTokenStream(lexer)
        def parser = new Turtle(tokenStream)
        def tree = parser.turtleDoc()
        return new TreeViewer(parser.ruleNames.toList(), tree)
    }

    TreeViewer createNTriplesTreeView(String text) {
        def stream = CharStreams.fromString(text)
        def lexer = new NTriplesLexer(stream)
        def tokenStream = new CommonTokenStream(lexer)
        def parser = new NTriplesParser(tokenStream)
        def tree = parser.ntriplesDoc()
        return new TreeViewer(parser.ruleNames.toList(), tree)
    }

    static void main(String[] args) {
        launch(ParseTreeDebugView.class, args)
    }
}
