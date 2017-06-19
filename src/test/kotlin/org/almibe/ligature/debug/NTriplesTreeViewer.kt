/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature.debug

import org.almibe.ligature.parser.NTriplesLexer
import org.almibe.ligature.parser.NTriplesParser
import org.antlr.v4.gui.TreeViewer
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream
import org.antlr.v4.runtime.tree.Tree
import javax.swing.JFrame
import javax.swing.JPanel
import javax.swing.JScrollPane

fun main(args : Array<String>) {
    val stream = CharStreams.fromString("<http://example.org/#spiderman> <http://www.perceive.net/schemas/relationship/enemyOf> <http://example.org/#green-goblin> .")
    val lexer = NTriplesLexer(stream)
    val tokenStream = CommonTokenStream(lexer)
    val parser = NTriplesParser(tokenStream)
    val tree: Tree = parser.document()

    val frame = JFrame("NTriples TreeViewer")
    val panel = JPanel()
    val scrollBox = JScrollPane(panel)
    val treeViewer = TreeViewer(parser.ruleNames.asList(), tree)

    treeViewer.scale = 1.5
    panel.add(treeViewer)
    frame.add(scrollBox)
    frame.setSize(800, 1200)
    frame.defaultCloseOperation = JFrame.EXIT_ON_CLOSE
    frame.isVisible = true
}
