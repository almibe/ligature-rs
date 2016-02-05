package org.libraryweasel.stinkpot

interface TokenType

data class Token<T : TokenType> (val tokenType: T, val text:String)
