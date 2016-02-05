package org.libraryweasel.stinkpot.ntriples

interface Callback<T> {
    fun callback(t: T)
}