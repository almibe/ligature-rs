/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.almibe.ligature

sealed class SparqlResult
sealed class SparqlCommand

data class Query(val queryType: QueryType, val valuesClause ValuesClause): SparqlCommand
data class Update(TODO): SparqlCommand

enum SelectModifier { DISTINCT, REDUCED }
data class Var(name: String, as: String?)

sealed class QueryType
data class SelectQuery(val selectClause: SelectClause, val datasetClauses: List<DatasetClause>, val whereClause: WhereClause, val solutionModifier: SolutionModifier): QueryType
data class ConstructQuery(TODO): QueryType
data class DescribeQuery(TODO): QueryType
data class AskQuery(TODO): QueryType

data class SelectClause(val modifier: SelectModifier?, val vars: List<Var>)

sealed class DatasetClause
data class DefaultGraphClause(val sourceSelector: IRI): DatasetClause
data class NamedGraphClause(val sourceSelector: IRI): DatasetClause

data class WhereClause(val groupGraphPattern: GroupGraphPattern)

sealed class GroupGraphPattern
data class SubSelect(val selectClause: SelectClause, val whereClause: WhereClause, val solutionModifier: SolutionModifier, val valuesClause: ValuesClause): GroupGraphPattern
data class GroupGraphPatternSub(TODO): GroupGraphPattern

data class SolutionModifier(val groupClause: GroupClause?, val havingClause: HavingClause?, val orderClause: OrderClause?, val limitOffsetClauses: LimitOffsetClauses?)

data class GroupClause(val groupConditions: List<GroupCondition>)
data class HavingClause(val constraint: Constraint)
data class OrderClause(val orderConditions: List<OrderCondition>)

sealed class LimitOffsetClauses
data class LimitOffsetClause(val limit: Int, val offset: Int?): LimitOffsetClauses
data class OffsetLimitClause(val offset: Int, val limit: Int?): LimitOffsetClauses

sealed class Constraint
data class BrackettedExpression(TODO): Constraint
data class BuiltInCall(TODO): Constraint
data class FunctionCall(TODO): Constraint

data class ValuesClause(val dataBlock: DataBlock?)

sealed class DataBlock
data class InlineDataOneVar(TODO): DataBlock
data class InlineDataFull(TODO): DataBlock
