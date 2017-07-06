/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.parsers

import org.almibe.ligature.parser.TurtleLexer
import org.almibe.ligature.parser.TurtleParser
import org.antlr.v4.gui.TreeViewer
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

import javax.swing.*

class ParseTreeDebugView {
    static void main(String[] args) {
        def stream = CharStreams.fromString("<http://example.org/#spiderman> <http://xmlns.com/foaf/0.1/name> \"test\"@ru .")
        def lexer = new TurtleLexer(stream)
//        def lexer = new NTriplesLexer(stream)
        def tokenStream = new CommonTokenStream(lexer)
        def parser = new TurtleParser(tokenStream)
//        def parser = new NTriplesParser(tokenStream)
        def tree = parser.turtleDoc()
//        def tree = parser.ntriplesDoc()

        def frame = new JFrame("ANTLR TreeViewer")
        def panel = new JPanel()
        def scrollBox = new JScrollPane(panel)
        def treeViewer = new TreeViewer(parser.ruleNames.toList(), tree)

        treeViewer.scale = 1.5
        panel.add(treeViewer)
        frame.add(scrollBox)
        frame.setSize(800, 1200)
        frame.defaultCloseOperation = JFrame.EXIT_ON_CLOSE
        frame.visible = true
    }
}
