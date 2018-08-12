/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.loaders

import org.almibe.ligature.parser.ntriples.NTriplesLexer
import org.almibe.ligature.parser.ntriples.NTriplesParser
import org.almibe.ligature.parser.turtle.ModalTurtleLexer
import org.almibe.ligature.parser.turtle.Turtle
import org.antlr.v4.gui.TreeViewer
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import java.awt.BorderLayout
import javax.swing.*

fun main(args: Array<String>) {
    val app = ParseTreeDebugView()
    app.show()
}

class ParseTreeDebugView {
    val frame = JFrame()
    val defaultDocument = "<http://one.example/subject1> <http://one.example/predicate1> <http://one.example/object1> ."
    val topControls = JPanel(BorderLayout())
    val textArea = JTextArea(defaultDocument)
    val swingNode = JPanel()
    val bottomScrollPane = JScrollPane(swingNode)
    val splitPane = JSplitPane()
    val radioGroup = ButtonGroup()
    val ntriplesButton = JRadioButton("N-Triples")
    val turtleButton = JRadioButton("Turtle")
    val buttonBox = JPanel()

    fun show() {
        topControls.add(textArea, BorderLayout.CENTER)
        topControls.add(buttonBox, BorderLayout.SOUTH)
        radioGroup.add(ntriplesButton)
        radioGroup.add(turtleButton)
        ntriplesButton.isSelected = true

        buttonBox.add(ntriplesButton)
        buttonBox.add(turtleButton)

        splitPane.add(topControls)
        splitPane.add(bottomScrollPane)

        splitPane.orientation = JSplitPane.HORIZONTAL_SPLIT
        splitPane.setDividerLocation(0.7)

        frame.setSize(1200, 800)
        frame.add(splitPane)
        frame.title = "Ligature Parser Debugger"
        frame.defaultCloseOperation = JFrame.EXIT_ON_CLOSE
        frame.isVisible = true
        checkDisplay() //TODO should be called on a timer?
    }

    fun checkDisplay() {
        SwingUtilities.invokeLater {
//            val component = ntriplesButton.selected ? createNTriplesTreeView(textArea.text) :
//            createTurtleTreeView(textArea.text)
//            updateSwingNode(component)
        }
    }

    fun updateSwingNode(component: TreeViewer) {
        SwingUtilities.invokeLater {
            component.scale = 1.5
        }
    }

    fun createTurtleTreeView(text: String): TreeViewer {
        val stream = CharStreams.fromString(text)
        val lexer = ModalTurtleLexer(stream)
        val tokenStream = CommonTokenStream(lexer)
        val parser = Turtle(tokenStream)
        val tree = parser.turtleDoc()
        return TreeViewer(parser.ruleNames.toList(), tree)
    }

     fun createNTriplesTreeView(text: String): TreeViewer {
        val stream = CharStreams.fromString(text)
        val lexer = NTriplesLexer(stream)
        val tokenStream = CommonTokenStream(lexer)
        val parser = NTriplesParser(tokenStream)
        val tree = parser.ntriplesDoc()
        return TreeViewer(parser.ruleNames.toList(), tree)
    }
}
