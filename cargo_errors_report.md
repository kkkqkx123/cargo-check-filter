# Cargo Check Error Analysis Report

## Filter Settings

- **No filters applied**

## Summary

- **Total Errors**: 344
- **Total Warnings**: 75
- **Total Issues**: 419
- **Unique Error Patterns**: 109
- **Unique Warning Patterns**: 24
- **Files with Issues**: 93

## Error Statistics

**Total Errors**: 344

### Error Type Breakdown

- **error[E0308]**: 146 errors
- **error[E0599]**: 122 errors
- **error[E0277]**: 29 errors
- **error[E0507]**: 23 errors
- **error[E0614]**: 18 errors
- **error[E0515]**: 4 errors
- **error[E0382]**: 2 errors

### Files with Errors (Top 10)

- `src\query\optimizer\elimination_rules.rs`: 28 errors
- `src\query\optimizer\limit_pushdown.rs`: 27 errors
- `src\query\optimizer\index_optimization.rs`: 22 errors
- `src\query\optimizer\operation_merge.rs`: 20 errors
- `src\query\optimizer\predicate_pushdown.rs`: 14 errors
- `src\query\planner\ngql\go_planner.rs`: 13 errors
- `src\query\planner\ngql\subgraph_planner.rs`: 12 errors
- `src\query\visitor\extract_filter_expr_visitor.rs`: 12 errors
- `src\query\planner\match_planning\utils\connection_strategy.rs`: 12 errors
- `src\query\planner\ngql\lookup_planner.rs`: 10 errors

## Warning Statistics

**Total Warnings**: 75

### Warning Type Breakdown

- **warning**: 75 warnings

### Files with Warnings (Top 10)

- `src\query\optimizer\elimination_rules.rs`: 28 warnings
- `src\query\planner\plan\core\nodes\factory.rs`: 15 warnings
- `src\query\visitor\extract_filter_expr_visitor.rs`: 12 warnings
- `src\query\visitor\find_visitor.rs`: 10 warnings
- `src\query\optimizer\join_optimization.rs`: 8 warnings
- `src\query\visitor\evaluable_expr_visitor.rs`: 8 warnings
- `src\core\evaluator\expression_evaluator.rs`: 8 warnings
- `src\query\executor\result_processing\aggregation.rs`: 6 warnings
- `src\query\optimizer\optimizer.rs`: 5 warnings
- `src\query\planner\plan\management\ddl\edge_ops.rs`: 5 warnings

## Detailed Error Categorization

### error[E0599]: no method named [identifier] found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in [identifier]

**Total Occurrences**: 85  
**Unique Files**: 7

#### `src\query\optimizer\limit_pushdown.rs`: 25 occurrences

- Line 74: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 81: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 104: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- ... 22 more occurrences in this file

#### `src\query\optimizer\index_optimization.rs`: 20 occurrences

- Line 29: no method named `is_index_scan` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 40: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 49: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- ... 17 more occurrences in this file

#### `src\query\optimizer\elimination_rules.rs`: 18 occurrences

- Line 56: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 120: no method named `is_dedup` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 142: no method named `is_dedup` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- ... 15 more occurrences in this file

#### `src\query\optimizer\operation_merge.rs`: 11 occurrences

- Line 40: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 109: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 110: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- ... 8 more occurrences in this file

#### `src\query\optimizer\predicate_pushdown.rs`: 6 occurrences

- Line 45: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 307: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 432: no method named `as_any` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- ... 3 more occurrences in this file

#### `src\query\optimizer\join_optimization.rs`: 3 occurrences

- Line 25: no method named `is_hash_inner_join` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 26: no method named `is_hash_left_join` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 27: no method named `is_inner_join` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`

#### `src\query\optimizer\scan_optimization.rs`: 2 occurrences

- Line 26: no method named `is_index_scan` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`
- Line 67: no method named `is_scan_edges` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: method not found in `PlanNodeEnum`


### error[E0308]: mismatched types: expected [identifier], found `Arc<dyn PlanNode>`

**Total Occurrences**: 18  
**Unique Files**: 8

#### `src\query\planner\ngql\lookup_planner.rs`: 4 occurrences

- Line 71: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 96: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 113: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- ... 1 more occurrences in this file

#### `src\query\planner\ngql\subgraph_planner.rs`: 4 occurrences

- Line 78: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 91: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 102: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- ... 1 more occurrences in this file

#### `src\query\planner\ngql\go_planner.rs`: 3 occurrences

- Line 117: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 144: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 161: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`

#### `src\query\planner\match_planning\clauses\unwind_planner.rs`: 2 occurrences

- Line 132: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 132: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`

#### `src\query\planner\ngql\path_planner.rs`: 2 occurrences

- Line 120: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`
- Line 139: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`

#### `src\query\planner\match_planning\paths\shortest_path_planner.rs`: 1 occurrences

- Line 61: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`

#### `src\query\planner\ngql\fetch_vertices_planner.rs`: 1 occurrences

- Line 144: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`

#### `src\query\planner\ngql\maintain_planner.rs`: 1 occurrences

- Line 92: mismatched types: expected `PlanNodeEnum`, found `Arc<dyn PlanNode>`


### error[E0614]: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced

**Total Occurrences**: 18  
**Unique Files**: 3

#### `src\query\visitor\evaluable_expr_visitor.rs`: 6 occurrences

- Line 133: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- Line 139: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- Line 140: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- ... 3 more occurrences in this file

#### `src\query\visitor\extract_filter_expr_visitor.rs`: 6 occurrences

- Line 191: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- Line 197: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- Line 198: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- ... 3 more occurrences in this file

#### `src\query\visitor\find_visitor.rs`: 6 occurrences

- Line 511: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- Line 521: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- Line 522: type `core::types::expression::Expression` cannot be dereferenced: can't be dereferenced
- ... 3 more occurrences in this file


### error[E0308]: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

**Total Occurrences**: 13  
**Unique Files**: 12

#### `src\query\executor\result_processing\aggregation.rs`: 2 occurrences

- Line 557: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`
- Line 825: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\data_processing\graph_traversal\expand.rs`: 1 occurrences

- Line 172: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\data_processing\graph_traversal\expand_all.rs`: 1 occurrences

- Line 241: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\data_processing\graph_traversal\shortest_path.rs`: 1 occurrences

- Line 349: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\data_processing\graph_traversal\traverse.rs`: 1 occurrences

- Line 294: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\result_processing\projection.rs`: 1 occurrences

- Line 204: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\result_processing\sort.rs`: 1 occurrences

- Line 286: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\result_processing\limit.rs`: 1 occurrences

- Line 290: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\result_processing\dedup.rs`: 1 occurrences

- Line 488: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\result_processing\filter.rs`: 1 occurrences

- Line 303: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\result_processing\sample.rs`: 1 occurrences

- Line 497: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`

#### `src\query\executor\result_processing\topn.rs`: 1 occurrences

- Line 411: mismatched types: expected `Option<&Box<dyn Executor<S>>>`, found `Option<Box<dyn Executor<S>>>`


### error[E0599]: no method named [identifier] found for reference `&plan_node_enum::PlanNodeEnum` in the current scope: method not found in `&PlanNodeEnum`

**Total Occurrences**: 11  
**Unique Files**: 2

#### `src\query\optimizer\predicate_pushdown.rs`: 8 occurrences

- Line 60: no method named `as_any` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope: method not found in `&PlanNodeEnum`
- Line 117: no method named `as_any` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope: method not found in `&PlanNodeEnum`
- Line 175: no method named `as_any` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope: method not found in `&PlanNodeEnum`
- ... 5 more occurrences in this file

#### `src\query\optimizer\operation_merge.rs`: 3 occurrences

- Line 41: no method named `as_any` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope: method not found in `&PlanNodeEnum`
- Line 300: no method named `is_dedup` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope: method not found in `&PlanNodeEnum`
- Line 363: no method named `is_dedup` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope: method not found in `&PlanNodeEnum`


### error[E0308]: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`

**Total Occurrences**: 8  
**Unique Files**: 7

#### `src\query\executor\result_processing\aggregation.rs`: 2 occurrences

- Line 471: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`
- Line 739: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`

#### `src\query\executor\result_processing\sort.rs`: 1 occurrences

- Line 200: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`

#### `src\query\executor\result_processing\limit.rs`: 1 occurrences

- Line 204: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`

#### `src\query\executor\result_processing\dedup.rs`: 1 occurrences

- Line 396: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`

#### `src\query\executor\result_processing\filter.rs`: 1 occurrences

- Line 217: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`

#### `src\query\executor\result_processing\sample.rs`: 1 occurrences

- Line 411: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`

#### `src\query\executor\result_processing\topn.rs`: 1 occurrences

- Line 427: mismatched types: expected `Option<&ExecutionResult>`, found `Option<ExecutionResult>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ArgumentNode>`

**Total Occurrences**: 8  
**Unique Files**: 6

#### `src\query\planner\ngql\fetch_vertices_planner.rs`: 2 occurrences

- Line 77: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`
- Line 145: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`

#### `src\query\planner\ngql\maintain_planner.rs`: 2 occurrences

- Line 70: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`
- Line 93: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`

#### `src\query\planner\ngql\fetch_edges_planner.rs`: 1 occurrences

- Line 92: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`

#### `src\query\planner\ngql\go_planner.rs`: 1 occurrences

- Line 162: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`

#### `src\query\planner\ngql\path_planner.rs`: 1 occurrences

- Line 140: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`

#### `src\query\planner\ngql\subgraph_planner.rs`: 1 occurrences

- Line 117: mismatched types: expected `PlanNodeEnum`, found `Arc<ArgumentNode>`


### error[E0277]: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`

**Total Occurrences**: 8  
**Unique Files**: 6

#### `src\query\planner\ngql\maintain_planner.rs`: 3 occurrences

- Line 78: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`
- Line 81: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`
- Line 84: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`

#### `src\query\planner\ngql\fetch_vertices_planner.rs`: 1 occurrences

- Line 139: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`

#### `src\query\planner\ngql\go_planner.rs`: 1 occurrences

- Line 156: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`

#### `src\query\planner\ngql\lookup_planner.rs`: 1 occurrences

- Line 108: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`

#### `src\query\planner\ngql\path_planner.rs`: 1 occurrences

- Line 134: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`

#### `src\query\planner\ngql\subgraph_planner.rs`: 1 occurrences

- Line 104: the trait bound `project_node::ProjectNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `project_node::ProjectNode`


### error[E0599]: no method named [identifier] found for enum `plan_node_enum::PlanNodeEnum` in the current scope

**Total Occurrences**: 6  
**Unique Files**: 3

#### `src\query\optimizer\join_optimization.rs`: 4 occurrences

- Line 50: no method named `type_name` found for enum `plan_node_enum::PlanNodeEnum` in the current scope
- Line 51: no method named `type_name` found for enum `plan_node_enum::PlanNodeEnum` in the current scope
- Line 97: no method named `type_name` found for enum `plan_node_enum::PlanNodeEnum` in the current scope
- ... 1 more occurrences in this file

#### `src\query\optimizer\limit_pushdown.rs`: 1 occurrences

- Line 505: no method named `is_scan_vertices` found for enum `plan_node_enum::PlanNodeEnum` in the current scope

#### `src\query\optimizer\scan_optimization.rs`: 1 occurrences

- Line 66: no method named `is_scan_vertices` found for enum `plan_node_enum::PlanNodeEnum` in the current scope


### error[E0599]: no method named [identifier] found for reference `&OptGroupNode` in the current scope: field, not a method

**Total Occurrences**: 6  
**Unique Files**: 1

#### `src\query\optimizer\operation_merge.rs`: 6 occurrences

- Line 99: no method named `plan_node` found for reference `&OptGroupNode` in the current scope: field, not a method
- Line 198: no method named `plan_node` found for reference `&OptGroupNode` in the current scope: field, not a method
- Line 261: no method named `plan_node` found for reference `&OptGroupNode` in the current scope: field, not a method
- ... 3 more occurrences in this file


### error[E0277]: the trait bound `filter_node::FilterNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `filter_node::FilterNode`

**Total Occurrences**: 6  
**Unique Files**: 4

#### `src\query\planner\ngql\subgraph_planner.rs`: 3 occurrences

- Line 68: the trait bound `filter_node::FilterNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `filter_node::FilterNode`
- Line 82: the trait bound `filter_node::FilterNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `filter_node::FilterNode`
- Line 95: the trait bound `filter_node::FilterNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `filter_node::FilterNode`

#### `src\query\planner\ngql\go_planner.rs`: 1 occurrences

- Line 136: the trait bound `filter_node::FilterNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `filter_node::FilterNode`

#### `src\query\planner\ngql\lookup_planner.rs`: 1 occurrences

- Line 74: the trait bound `filter_node::FilterNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `filter_node::FilterNode`

#### `src\query\planner\ngql\path_planner.rs`: 1 occurrences

- Line 105: the trait bound `filter_node::FilterNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `filter_node::FilterNode`


### error[E0507]: cannot move out of `left.root` which is behind a shared reference

**Total Occurrences**: 6  
**Unique Files**: 1

#### `src\query\planner\match_planning\utils\connection_strategy.rs`: 6 occurrences

- Line 74: cannot move out of `left.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `left.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait
- Line 119: cannot move out of `left.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `left.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait
- Line 167: cannot move out of `left.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `left.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait
- ... 3 more occurrences in this file


### error[E0308]: mismatched types: expected [identifier], found `Box<PlanNodeEnum>`

**Total Occurrences**: 5  
**Unique Files**: 2

#### `src\query\optimizer\elimination_rules.rs`: 4 occurrences

- Line 882: mismatched types: expected `PlanNodeEnum`, found `Box<PlanNodeEnum>`
- Line 906: mismatched types: expected `PlanNodeEnum`, found `Box<PlanNodeEnum>`
- Line 918: mismatched types: expected `PlanNodeEnum`, found `Box<PlanNodeEnum>`
- ... 1 more occurrences in this file

#### `src\query\optimizer\transformation_rules.rs`: 1 occurrences

- Line 46: mismatched types: expected `PlanNodeEnum`, found `Box<PlanNodeEnum>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ProjectNode>`

**Total Occurrences**: 5  
**Unique Files**: 4

#### `src\query\planner\ngql\fetch_vertices_planner.rs`: 2 occurrences

- Line 125: mismatched types: expected `PlanNodeEnum`, found `Arc<ProjectNode>`
- Line 131: mismatched types: expected `PlanNodeEnum`, found `Arc<ProjectNode>`

#### `src\query\planner\ngql\go_planner.rs`: 1 occurrences

- Line 151: mismatched types: expected `PlanNodeEnum`, found `Arc<ProjectNode>`

#### `src\query\planner\ngql\lookup_planner.rs`: 1 occurrences

- Line 103: mismatched types: expected `PlanNodeEnum`, found `Arc<ProjectNode>`

#### `src\query\planner\ngql\path_planner.rs`: 1 occurrences

- Line 129: mismatched types: expected `PlanNodeEnum`, found `Arc<ProjectNode>`


### error[E0277]: the trait bound `ExpandAllNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for [identifier]

**Total Occurrences**: 5  
**Unique Files**: 3

#### `src\query\planner\ngql\go_planner.rs`: 2 occurrences

- Line 114: the trait bound `ExpandAllNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `ExpandAllNode`
- Line 140: the trait bound `ExpandAllNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `ExpandAllNode`

#### `src\query\planner\ngql\subgraph_planner.rs`: 2 occurrences

- Line 69: the trait bound `ExpandAllNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `ExpandAllNode`
- Line 72: the trait bound `ExpandAllNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `ExpandAllNode`

#### `src\query\planner\ngql\path_planner.rs`: 1 occurrences

- Line 107: the trait bound `ExpandAllNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `ExpandAllNode`


### error[E0507]: cannot move out of `right.root` which is behind a shared reference

**Total Occurrences**: 5  
**Unique Files**: 1

#### `src\query\planner\match_planning\utils\connection_strategy.rs`: 5 occurrences

- Line 77: cannot move out of `right.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `right.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait
- Line 122: cannot move out of `right.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `right.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait
- Line 170: cannot move out of `right.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `right.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait
- ... 2 more occurrences in this file


### error[E0599]: no method named [identifier] found for mutable reference `&mut V` in the current scope

**Total Occurrences**: 4  
**Unique Files**: 2

#### `src\query\planner\plan\algorithms\path_algorithms.rs`: 3 occurrences

- Line 267: no method named `visit_bfs_shortest` found for mutable reference `&mut V` in the current scope
- Line 401: no method named `visit_all_paths` found for mutable reference `&mut V` in the current scope
- Line 528: no method named `visit_shortest_path` found for mutable reference `&mut V` in the current scope

#### `src\query\planner\plan\algorithms\index_scan.rs`: 1 occurrences

- Line 135: no method named `visit_index_scan` found for mutable reference `&mut V` in the current scope


### error[E0308]: mismatched types: expected `Option<&String>`, found `Option<String>`

**Total Occurrences**: 4  
**Unique Files**: 3

#### `src\query\planner\plan\core\nodes\traversal_node.rs`: 2 occurrences

- Line 349: mismatched types: expected `Option<&String>`, found `Option<String>`
- Line 484: mismatched types: expected `Option<&String>`, found `Option<String>`

#### `src\query\planner\plan\core\nodes\graph_scan_node.rs`: 1 occurrences

- Line 668: mismatched types: expected `Option<&String>`, found `Option<String>`

#### `src\query\visitor\evaluable_expr_visitor.rs`: 1 occurrences

- Line 73: mismatched types: expected `Option<&String>`, found `Option<String>`


### error[E0308]: mismatched types: expected `Arc<dyn PlanNode>`, found [identifier]

**Total Occurrences**: 4  
**Unique Files**: 3

#### `src\query\planner\match_planning\paths\match_path_planner.rs`: 2 occurrences

- Line 220: mismatched types: expected `Arc<dyn PlanNode>`, found `PlanNodeEnum`
- Line 274: mismatched types: expected `Arc<dyn PlanNode>`, found `PlanNodeEnum`

#### `src\query\planner\match_planning\clauses\unwind_planner.rs`: 1 occurrences

- Line 239: mismatched types: expected `Arc<dyn PlanNode>`, found `PlanNodeEnum`

#### `src\query\planner\match_planning\paths\shortest_path_planner.rs`: 1 occurrences

- Line 58: mismatched types: expected `Arc<dyn PlanNode>`, found `PlanNodeEnum`


### error[E0277]: the trait bound `data_processing_node::DedupNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `data_processing_node::DedupNode`

**Total Occurrences**: 4  
**Unique Files**: 4

#### `src\query\planner\ngql\fetch_vertices_planner.rs`: 1 occurrences

- Line 137: the trait bound `data_processing_node::DedupNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `data_processing_node::DedupNode`

#### `src\query\planner\ngql\go_planner.rs`: 1 occurrences

- Line 154: the trait bound `data_processing_node::DedupNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `data_processing_node::DedupNode`

#### `src\query\planner\ngql\lookup_planner.rs`: 1 occurrences

- Line 106: the trait bound `data_processing_node::DedupNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `data_processing_node::DedupNode`

#### `src\query\planner\ngql\path_planner.rs`: 1 occurrences

- Line 132: the trait bound `data_processing_node::DedupNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `data_processing_node::DedupNode`


### error[E0308]: mismatched types: expected `&Expression`, found `Box<Expression>`

**Total Occurrences**: 4  
**Unique Files**: 1

#### `src\query\validator\strategies\aggregate_strategy.rs`: 4 occurrences

- Line 42: mismatched types: expected `&Expression`, found `Box<Expression>`
- Line 51: mismatched types: expected `&Expression`, found `Box<Expression>`
- Line 162: mismatched types: expected `&Expression`, found `Box<Expression>`
- ... 1 more occurrences in this file


### error[E0599]: no variant or associated item named [identifier] found for enum `plan_node_enum::PlanNodeEnum` in the current scope: variant or associated item not found in [identifier]

**Total Occurrences**: 3  
**Unique Files**: 1

#### `src\query\executor\factory.rs`: 3 occurrences

- Line 80: no variant or associated item named `HashInnerJoin` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: variant or associated item not found in `PlanNodeEnum`
- Line 81: no variant or associated item named `HashLeftJoin` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: variant or associated item not found in `PlanNodeEnum`
- Line 82: no variant or associated item named `CartesianProduct` found for enum `plan_node_enum::PlanNodeEnum` in the current scope: variant or associated item not found in `PlanNodeEnum`


### error[E0599]: no method named [identifier] found for reference `&plan_node_enum::PlanNodeEnum` in the current scope

**Total Occurrences**: 3  
**Unique Files**: 2

#### `src\query\optimizer\optimizer.rs`: 2 occurrences

- Line 322: no method named `type_name` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope
- Line 323: no method named `type_name` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope

#### `src\query\executor\factory.rs`: 1 occurrences

- Line 95: no method named `type_name` found for reference `&plan_node_enum::PlanNodeEnum` in the current scope


### error[E0308]: mismatched types: expected [identifier], found `Arc<PlanNodeEnum>`

**Total Occurrences**: 3  
**Unique Files**: 2

#### `src\query\optimizer\rule_traits.rs`: 2 occurrences

- Line 427: mismatched types: expected `PlanNodeEnum`, found `Arc<PlanNodeEnum>`
- Line 436: mismatched types: expected `PlanNodeEnum`, found `Arc<PlanNodeEnum>`

#### `src\query\optimizer\transformation_rules.rs`: 1 occurrences

- Line 56: mismatched types: expected `PlanNodeEnum`, found `Arc<PlanNodeEnum>`


### error[E0308]: arguments to this function are incorrect

**Total Occurrences**: 3  
**Unique Files**: 2

#### `src\query\optimizer\elimination_rules.rs`: 2 occurrences

- Line 983: arguments to this function are incorrect
- Line 998: arguments to this function are incorrect

#### `src\query\planner\ngql\go_planner.rs`: 1 occurrences

- Line 93: arguments to this function are incorrect


### error[E0515]: cannot return value referencing local variable [identifier]: returns a value referencing data owned by the current function

**Total Occurrences**: 3  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\plan_node_traits.rs`: 3 occurrences

- Line 42: cannot return value referencing local variable `deps`: returns a value referencing data owned by the current function
- Line 52: cannot return value referencing local variable `deps`: returns a value referencing data owned by the current function
- Line 62: cannot return value referencing local variable `deps`: returns a value referencing data owned by the current function


### error[E0308]: mismatched types: expected [identifier], found `Box<Expression>`

**Total Occurrences**: 2  
**Unique Files**: 1

#### `src\core\visitor.rs`: 2 occurrences

- Line 403: mismatched types: expected `Expression`, found `Box<Expression>`
- Line 410: mismatched types: expected `Expression`, found `Box<Expression>`


### error[E0308]: mismatched types: expected `Option<&Value>`, found `Option<Value>`

**Total Occurrences**: 2  
**Unique Files**: 1

#### `src\query\executor\cypher\context.rs`: 2 occurrences

- Line 151: mismatched types: expected `Option<&Value>`, found `Option<Value>`
- Line 331: mismatched types: expected `Option<&Value>`, found `Option<Value>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<IndexScan>`

**Total Occurrences**: 2  
**Unique Files**: 1

#### `src\query\optimizer\index_optimization.rs`: 2 occurrences

- Line 735: mismatched types: expected `PlanNodeEnum`, found `Arc<IndexScan>`
- Line 901: mismatched types: expected `PlanNodeEnum`, found `Arc<IndexScan>`


### error[E0308]: mismatched types: expected `&Expr`, found `Box<Expr>`

**Total Occurrences**: 2  
**Unique Files**: 1

#### `src\query\parser\ast\expr.rs`: 2 occurrences

- Line 489: mismatched types: expected `&Expr`, found `Box<Expr>`
- Line 497: mismatched types: expected `&Expr`, found `Box<Expr>`


### error[E0599]: no method named [identifier] found for mutable reference `&mut V` in the current scope: method not found in `&mut V`

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\planner\plan\algorithms\index_scan.rs`: 1 occurrences

- Line 244: no method named `visit_fulltext_index_scan` found for mutable reference `&mut V` in the current scope: method not found in `&mut V`

#### `src\query\planner\plan\algorithms\path_algorithms.rs`: 1 occurrences

- Line 144: no method named `visit_multi_shortest_path` found for mutable reference `&mut V` in the current scope: method not found in `&mut V`


### error[E0308]: mismatched types: expected [identifier], found `Arc<GetVerticesNode>`

**Total Occurrences**: 2  
**Unique Files**: 1

#### `src\query\planner\ngql\fetch_vertices_planner.rs`: 2 occurrences

- Line 95: mismatched types: expected `PlanNodeEnum`, found `Arc<GetVerticesNode>`
- Line 103: mismatched types: expected `PlanNodeEnum`, found `Arc<GetVerticesNode>`


### error[E0277]: the trait bound `join_node::InnerJoinNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `join_node::InnerJoinNode`

**Total Occurrences**: 2  
**Unique Files**: 1

#### `src\query\planner\ngql\go_planner.rs`: 2 occurrences

- Line 112: the trait bound `join_node::InnerJoinNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `join_node::InnerJoinNode`
- Line 138: the trait bound `join_node::InnerJoinNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `join_node::InnerJoinNode`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ExpandAllNode>`

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\planner\ngql\path_planner.rs`: 1 occurrences

- Line 102: mismatched types: expected `PlanNodeEnum`, found `Arc<ExpandAllNode>`

#### `src\query\planner\ngql\subgraph_planner.rs`: 1 occurrences

- Line 65: mismatched types: expected `PlanNodeEnum`, found `Arc<ExpandAllNode>`


### error[E0507]: cannot move out of `input_plan.root` which is behind a shared reference

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\planner\match_planning\clauses\order_by_planner.rs`: 1 occurrences

- Line 60: cannot move out of `input_plan.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `input_plan.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait

#### `src\query\planner\match_planning\clauses\pagination_planner.rs`: 1 occurrences

- Line 60: cannot move out of `input_plan.root` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `input_plan.root` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait


### error[E0382]: borrow of moved value: `plan.root`: value borrowed here after move

**Total Occurrences**: 2  
**Unique Files**: 1

#### `src\query\planner\match_planning\clauses\yield_planner.rs`: 2 occurrences

- Line 86: borrow of moved value: `plan.root`: value borrowed here after move
- Line 117: borrow of moved value: `plan.root`: value borrowed here after move


### error[E0507]: cannot move out of `*default` which is behind a shared reference

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\visitor\extract_filter_expr_visitor.rs`: 1 occurrences

- Line 379: cannot move out of `*default` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `*default` has type `std::option::Option<core::types::expression::Expression>`, which does not implement the `Copy` trait

#### `src\query\visitor\find_visitor.rs`: 1 occurrences

- Line 710: cannot move out of `*default` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `*default` has type `std::option::Option<core::types::expression::Expression>`, which does not implement the `Copy` trait


### error[E0507]: cannot move out of `*start` which is behind a shared reference

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\visitor\extract_filter_expr_visitor.rs`: 1 occurrences

- Line 434: cannot move out of `*start` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `*start` has type `std::option::Option<core::types::expression::Expression>`, which does not implement the `Copy` trait

#### `src\query\visitor\find_visitor.rs`: 1 occurrences

- Line 752: cannot move out of `*start` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `*start` has type `std::option::Option<core::types::expression::Expression>`, which does not implement the `Copy` trait


### error[E0507]: cannot move out of `*end` which is behind a shared reference

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\visitor\extract_filter_expr_visitor.rs`: 1 occurrences

- Line 435: cannot move out of `*end` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `*end` has type `std::option::Option<core::types::expression::Expression>`, which does not implement the `Copy` trait

#### `src\query\visitor\find_visitor.rs`: 1 occurrences

- Line 753: cannot move out of `*end` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `*end` has type `std::option::Option<core::types::expression::Expression>`, which does not implement the `Copy` trait


### error[E0308]: mismatched types: expected `&Option<Expression>`, found `&Option<&Expression>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\core\visitor.rs`: 1 occurrences

- Line 326: mismatched types: expected `&Option<Expression>`, found `&Option<&Expression>`


### error[E0308]: arguments to this method are incorrect

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\core\visitor.rs`: 1 occurrences

- Line 423: arguments to this method are incorrect


### error[E0308]: mismatched types: expected `Vec<Vertex>`, found `Vec<Box<Vertex>>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\executor\data_processing\graph_traversal\traverse.rs`: 1 occurrences

- Line 283: mismatched types: expected `Vec<Vertex>`, found `Vec<Box<Vertex>>`


### error[E0308]: mismatched types: expected `Option<&Vertex>`, found `Option<Vertex>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\executor\cypher\context.rs`: 1 occurrences

- Line 255: mismatched types: expected `Option<&Vertex>`, found `Option<Vertex>`


### error[E0308]: mismatched types: expected `Option<&Edge>`, found `Option<Edge>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\executor\cypher\context.rs`: 1 occurrences

- Line 265: mismatched types: expected `Option<&Edge>`, found `Option<Edge>`


### error[E0599]: no method named [identifier] found for reference `&project_node::ProjectNode` in the current scope: method not found in `&ProjectNode`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\optimizer\elimination_rules.rs`: 1 occurrences

- Line 889: no method named `dependencies` found for reference `&project_node::ProjectNode` in the current scope: method not found in `&ProjectNode`


### error[E0599]: no method named [identifier] found for struct [identifier] in the current scope: field, not a method

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\optimizer\limit_pushdown.rs`: 1 occurrences

- Line 41: no method named `plan_node` found for struct `OptGroupNode` in the current scope: field, not a method


### error[E0308]: mismatched types: expected `&PlanNodeEnum`, found [identifier]

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\optimizer\optimizer.rs`: 1 occurrences

- Line 294: mismatched types: expected `&PlanNodeEnum`, found `PlanNodeEnum`


### error[E0277]: the trait bound `Box<dyn OptRule>: OptRule` is not satisfied: the trait [identifier] is not implemented for `Box<dyn OptRule>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\optimizer\optimizer.rs`: 1 occurrences

- Line 573: the trait bound `Box<dyn OptRule>: OptRule` is not satisfied: the trait `OptRule` is not implemented for `Box<dyn OptRule>`


### error[E0308]: mismatched types: expected `Option<&Variable>`, found `Option<Variable>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\filter_node.rs`: 1 occurrences

- Line 57: mismatched types: expected `Option<&Variable>`, found `Option<Variable>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowConfigs>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\config_ops.rs`: 1 occurrences

- Line 51: mismatched types: expected `ShowConfigs`, found `Arc<ShowConfigs>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<SetConfig>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\config_ops.rs`: 1 occurrences

- Line 87: mismatched types: expected `SetConfig`, found `Arc<SetConfig>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<GetConfig>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\config_ops.rs`: 1 occurrences

- Line 117: mismatched types: expected `GetConfig`, found `Arc<GetConfig>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<AddHosts>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\host_ops.rs`: 1 occurrences

- Line 35: mismatched types: expected `AddHosts`, found `Arc<AddHosts>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DropHosts>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\host_ops.rs`: 1 occurrences

- Line 57: mismatched types: expected `DropHosts`, found `Arc<DropHosts>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowHosts>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\host_ops.rs`: 1 occurrences

- Line 73: mismatched types: expected `ShowHosts`, found `Arc<ShowHosts>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowHostsStatus>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\host_ops.rs`: 1 occurrences

- Line 89: mismatched types: expected `ShowHostsStatus`, found `Arc<ShowHostsStatus>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<CreateIndex>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\index_ops.rs`: 1 occurrences

- Line 50: mismatched types: expected `CreateIndex`, found `Arc<CreateIndex>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DropIndex>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\index_ops.rs`: 1 occurrences

- Line 80: mismatched types: expected `DropIndex`, found `Arc<DropIndex>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowIndexes>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\index_ops.rs`: 1 occurrences

- Line 102: mismatched types: expected `ShowIndexes`, found `Arc<ShowIndexes>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DescIndex>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\index_ops.rs`: 1 occurrences

- Line 126: mismatched types: expected `DescIndex`, found `Arc<DescIndex>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<SubmitJob>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\system_ops.rs`: 1 occurrences

- Line 40: mismatched types: expected `SubmitJob`, found `Arc<SubmitJob>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<CreateSnapshot>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\system_ops.rs`: 1 occurrences

- Line 70: mismatched types: expected `CreateSnapshot`, found `Arc<CreateSnapshot>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DropSnapshot>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\system_ops.rs`: 1 occurrences

- Line 94: mismatched types: expected `DropSnapshot`, found `Arc<DropSnapshot>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowSnapshots>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\admin\system_ops.rs`: 1 occurrences

- Line 110: mismatched types: expected `ShowSnapshots`, found `Arc<ShowSnapshots>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<CreateEdge>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\edge_ops.rs`: 1 occurrences

- Line 41: mismatched types: expected `CreateEdge`, found `Arc<CreateEdge>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DropEdge>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\edge_ops.rs`: 1 occurrences

- Line 71: mismatched types: expected `DropEdge`, found `Arc<DropEdge>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowEdges>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\edge_ops.rs`: 1 occurrences

- Line 87: mismatched types: expected `ShowEdges`, found `Arc<ShowEdges>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowCreateEdge>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\edge_ops.rs`: 1 occurrences

- Line 111: mismatched types: expected `ShowCreateEdge`, found `Arc<ShowCreateEdge>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<CreateSpace>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 65: mismatched types: expected `CreateSpace`, found `Arc<CreateSpace>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DescSpace>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 89: mismatched types: expected `DescSpace`, found `Arc<DescSpace>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowCreateSpace>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 113: mismatched types: expected `ShowCreateSpace`, found `Arc<ShowCreateSpace>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowSpaces>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 129: mismatched types: expected `ShowSpaces`, found `Arc<ShowSpaces>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<SwitchSpace>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 153: mismatched types: expected `SwitchSpace`, found `Arc<SwitchSpace>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DropSpace>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 183: mismatched types: expected `DropSpace`, found `Arc<DropSpace>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ClearSpace>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 213: mismatched types: expected `ClearSpace`, found `Arc<ClearSpace>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<AlterSpace>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\space_ops.rs`: 1 occurrences

- Line 252: mismatched types: expected `AlterSpace`, found `Arc<AlterSpace>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<CreateTag>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\tag_ops.rs`: 1 occurrences

- Line 40: mismatched types: expected `CreateTag`, found `Arc<CreateTag>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DescTag>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\tag_ops.rs`: 1 occurrences

- Line 64: mismatched types: expected `DescTag`, found `Arc<DescTag>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DropTag>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\tag_ops.rs`: 1 occurrences

- Line 94: mismatched types: expected `DropTag`, found `Arc<DropTag>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowTags>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\tag_ops.rs`: 1 occurrences

- Line 110: mismatched types: expected `ShowTags`, found `Arc<ShowTags>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowCreateTag>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\ddl\tag_ops.rs`: 1 occurrences

- Line 134: mismatched types: expected `ShowCreateTag`, found `Arc<ShowCreateTag>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<NewVertex>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\data_constructors.rs`: 1 occurrences

- Line 30: mismatched types: expected `NewVertex`, found `Arc<NewVertex>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<NewTag>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\data_constructors.rs`: 1 occurrences

- Line 57: mismatched types: expected `NewTag`, found `Arc<NewTag>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<NewProp>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\data_constructors.rs`: 1 occurrences

- Line 87: mismatched types: expected `NewProp`, found `Arc<NewProp>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<NewEdge>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\data_constructors.rs`: 1 occurrences

- Line 114: mismatched types: expected `NewEdge`, found `Arc<NewEdge>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DeleteVertices>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\delete_ops.rs`: 1 occurrences

- Line 25: mismatched types: expected `DeleteVertices`, found `Arc<DeleteVertices>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DeleteTags>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\delete_ops.rs`: 1 occurrences

- Line 52: mismatched types: expected `DeleteTags`, found `Arc<DeleteTags>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DeleteEdges>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\delete_ops.rs`: 1 occurrences

- Line 74: mismatched types: expected `DeleteEdges`, found `Arc<DeleteEdges>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<InsertVertices>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\insert_ops.rs`: 1 occurrences

- Line 25: mismatched types: expected `InsertVertices`, found `Arc<InsertVertices>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<InsertEdges>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\insert_ops.rs`: 1 occurrences

- Line 47: mismatched types: expected `InsertEdges`, found `Arc<InsertEdges>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<UpdateVertex>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\update_ops.rs`: 1 occurrences

- Line 33: mismatched types: expected `UpdateVertex`, found `Arc<UpdateVertex>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<UpdateEdge>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\update_ops.rs`: 1 occurrences

- Line 63: mismatched types: expected `UpdateEdge`, found `Arc<UpdateEdge>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<CreateRole>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\security\role_ops.rs`: 1 occurrences

- Line 33: mismatched types: expected `CreateRole`, found `Arc<CreateRole>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<DropRole>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\security\role_ops.rs`: 1 occurrences

- Line 63: mismatched types: expected `DropRole`, found `Arc<DropRole>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<GrantRole>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\security\role_ops.rs`: 1 occurrences

- Line 93: mismatched types: expected `GrantRole`, found `Arc<GrantRole>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<RevokeRole>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\security\role_ops.rs`: 1 occurrences

- Line 123: mismatched types: expected `RevokeRole`, found `Arc<RevokeRole>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<ShowRoles>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\security\role_ops.rs`: 1 occurrences

- Line 139: mismatched types: expected `ShowRoles`, found `Arc<ShowRoles>`


### error[E0277]: a value of type `Vec<&dyn CypherClausePlanner>` cannot be built from an iterator over elements of type `&Box<dyn CypherClausePlanner>`: value of type `Vec<&dyn CypherClausePlanner>` cannot be built from `std::iter::Iterator<Item=&Box<dyn CypherClausePlanner>>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\match_planning\match_planner.rs`: 1 occurrences

- Line 123: a value of type `Vec<&dyn CypherClausePlanner>` cannot be built from an iterator over elements of type `&Box<dyn CypherClausePlanner>`: value of type `Vec<&dyn CypherClausePlanner>` cannot be built from `std::iter::Iterator<Item=&Box<dyn CypherClausePlanner>>`


### error[E0308]: mismatched types: expected `Option<&SubPlan>`, found `Option<SubPlan>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\match_planning\match_planner.rs`: 1 occurrences

- Line 138: mismatched types: expected `Option<&SubPlan>`, found `Option<SubPlan>`


### error[E0308]: mismatched types: expected [identifier], found `Arc<GetEdgesNode>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\ngql\fetch_edges_planner.rs`: 1 occurrences

- Line 65: mismatched types: expected `PlanNodeEnum`, found `Arc<GetEdgesNode>`


### error[E0308]: [identifier] arms have incompatible types: expected `Arc<FilterNode>`, found `Arc<GetEdgesNode>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\ngql\fetch_edges_planner.rs`: 1 occurrences

- Line 72: `match` arms have incompatible types: expected `Arc<FilterNode>`, found `Arc<GetEdgesNode>`


### error[E0277]: the trait bound `graph_scan_node::GetEdgesNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `graph_scan_node::GetEdgesNode`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\ngql\lookup_planner.rs`: 1 occurrences

- Line 56: the trait bound `graph_scan_node::GetEdgesNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `graph_scan_node::GetEdgesNode`


### error[E0277]: the trait bound `graph_scan_node::GetVerticesNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `graph_scan_node::GetVerticesNode`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\ngql\lookup_planner.rs`: 1 occurrences

- Line 60: the trait bound `graph_scan_node::GetVerticesNode: plan_node_traits::PlanNode` is not satisfied: the trait `plan_node_traits::PlanNode` is not implemented for `graph_scan_node::GetVerticesNode`


### error[E0308]: mismatched types: expected `Option<&TypeDeductionError>`, found `Option<TypeDeductionError>`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\visitor\deduce_type_visitor.rs`: 1 occurrences

- Line 88: mismatched types: expected `Option<&TypeDeductionError>`, found `Option<TypeDeductionError>`


### error[E0507]: cannot move out of `case_expr.default_alternative` which is behind a shared reference

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\expression\cypher\expression_optimizer.rs`: 1 occurrences

- Line 122: cannot move out of `case_expr.default_alternative` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `case_expr.default_alternative` has type `std::option::Option<Box<query::parser::cypher::ast::expressions::Expression>>`, which does not implement the `Copy` trait


### error[E0507]: cannot move out of `e.match_expr` which is behind a shared reference

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\parser\ast\expr.rs`: 1 occurrences

- Line 51: cannot move out of `e.match_expr` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `e.match_expr` has type `std::option::Option<Box<expr::Expr>>`, which does not implement the `Copy` trait


### error[E0507]: cannot move out of `e.default` which is behind a shared reference

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\parser\ast\expr.rs`: 1 occurrences

- Line 59: cannot move out of `e.default` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `e.default` has type `std::option::Option<Box<expr::Expr>>`, which does not implement the `Copy` trait


### error[E0515]: cannot return value referencing temporary value: returns a value referencing data owned by the current function

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\plan_node_traits.rs`: 1 occurrences

- Line 75: cannot return value referencing temporary value: returns a value referencing data owned by the current function


### error[E0507]: cannot move out of `right.tail` which is behind a shared reference

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\match_planning\utils\connection_strategy.rs`: 1 occurrences

- Line 225: cannot move out of `right.tail` which is behind a shared reference: help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents, move occurs because `right.tail` has type `std::option::Option<plan_node_enum::PlanNodeEnum>`, which does not implement the `Copy` trait



## Detailed Warning Categorization

### warning: unused variable: [identifier]

**Total Occurrences**: 29  
**Unique Files**: 10

#### `src\query\planner\plan\core\nodes\factory.rs`: 12 occurrences

- Line 38: unused variable: `expr`: help: if this is intentional, prefix it with an underscore: `_expr`
- Line 34: unused variable: `input`: help: if this is intentional, prefix it with an underscore: `_input`
- Line 49: unused variable: `input`: help: if this is intentional, prefix it with an underscore: `_input`
- ... 9 more occurrences in this file

#### `src\core\evaluator\expression_evaluator.rs`: 6 occurrences

- Line 311: unused variable: `expr`: help: if this is intentional, prefix it with an underscore: `_expr`
- Line 311: unused variable: `context`: help: if this is intentional, prefix it with an underscore: `_context`
- Line 904: unused variable: `distinct`: help: if this is intentional, prefix it with an underscore: `_distinct`
- ... 3 more occurrences in this file

#### `src\query\executor\result_processing\aggregation.rs`: 2 occurrences

- Line 283: unused variable: `i`: help: if this is intentional, prefix it with an underscore: `_i`
- Line 283: unused variable: `col_name`: help: if this is intentional, prefix it with an underscore: `_col_name`

#### `src\core\evaluator\traits.rs`: 2 occurrences

- Line 30: unused variable: `expr`: help: if this is intentional, prefix it with an underscore: `_expr`
- Line 30: unused variable: `context`: help: if this is intentional, prefix it with an underscore: `_context`

#### `src\query\visitor\extract_filter_expr_visitor.rs`: 2 occurrences

- Line 343: unused variable: `func`: help: if this is intentional, prefix it with an underscore: `_func`
- Line 467: unused variable: `name`: help: if this is intentional, prefix it with an underscore: `_name`

#### `src\query\executor\data_processing\transformations\append_vertices.rs`: 1 occurrences

- Line 314: unused variable: `expr_context`: help: if this is intentional, prefix it with an underscore: `_expr_context`

#### `src\core\context\base.rs`: 1 occurrences

- Line 102: unused variable: `event`: help: if this is intentional, prefix it with an underscore: `_event`

#### `src\core\query_pipeline_manager.rs`: 1 occurrences

- Line 117: unused variable: `query_context`: help: if this is intentional, prefix it with an underscore: `_query_context`

#### `src\query\executor\cypher\clauses\match_path\expression_evaluator.rs`: 1 occurrences

- Line 103: unused variable: `path`: help: if this is intentional, prefix it with an underscore: `_path`

#### `src\query\visitor\deduce_props_visitor.rs`: 1 occurrences

- Line 402: unused variable: `property`: help: if this is intentional, prefix it with an underscore: `_property`


### warning: unused import: [identifier]

**Total Occurrences**: 12  
**Unique Files**: 12

#### `src\cache\factory.rs`: 1 occurrences

- Line 7: unused import: `StatsMode`

#### `src\cache\global_manager.rs`: 1 occurrences

- Line 5: unused import: `CacheStrategy`

#### `src\core\context\query.rs`: 1 occurrences

- Line 7: unused import: `QueryResult`

#### `src\core\context\request.rs`: 1 occurrences

- Line 10: unused import: `HierarchicalContext`

#### `src\core\context\validation.rs`: 1 occurrences

- Line 9: unused import: `HierarchicalContext`

#### `src\query\planner\plan\core\nodes\control_flow_node.rs`: 1 occurrences

- Line 6: unused import: `Arc`

#### `src\query\planner\plan\core\nodes\graph_scan_node.rs`: 1 occurrences

- Line 9: unused import: `Arc`

#### `src\query\planner\plan\core\nodes\traversal_node.rs`: 1 occurrences

- Line 8: unused import: `Arc`

#### `src\query\visitor\deduce_props_visitor.rs`: 1 occurrences

- Line 5: unused import: `VisitorResult`

#### `src\query\visitor\evaluable_expr_visitor.rs`: 1 occurrences

- Line 4: unused import: `VisitorResult`

#### `src\query\visitor\extract_filter_expr_visitor.rs`: 1 occurrences

- Line 4: unused import: `VisitorResult`

#### `src\query\visitor\find_visitor.rs`: 1 occurrences

- Line 4: unused import: `VisitorResult`


### warning: unused import: `crate::core::Value`

**Total Occurrences**: 3  
**Unique Files**: 3

#### `src\core\context\execution.rs`: 1 occurrences

- Line 9: unused import: `crate::core::Value`

#### `src\core\context\query.rs`: 1 occurrences

- Line 8: unused import: `crate::core::Value`

#### `src\core\context\session.rs`: 1 occurrences

- Line 8: unused import: `crate::core::Value`


### warning: unused imports: [identifier] and [identifier]

**Total Occurrences**: 3  
**Unique Files**: 3

#### `src\core\context\runtime.rs`: 1 occurrences

- Line 17: unused imports: `MemoryIndexManager` and `MemorySchemaManager`

#### `src\expression\cypher\expression_converter.rs`: 1 occurrences

- Line 3: unused imports: `BinaryOperator` and `UnaryOperator`

#### `src\query\visitor\mod.rs`: 1 occurrences

- Line 6: unused imports: `ExpressionVisitor` and `Expression`


### warning: ambiguous glob re-exports: the name [identifier] in the type namespace is first re-exported here

**Total Occurrences**: 3  
**Unique Files**: 2

#### `src\core\context\mod.rs`: 2 occurrences

- Line 20: ambiguous glob re-exports: the name `SessionVariable` in the type namespace is first re-exported here
- Line 22: ambiguous glob re-exports: the name `SessionInfo` in the type namespace is first re-exported here

#### `src\core\mod.rs`: 1 occurrences

- Line 47: ambiguous glob re-exports: the name `SymbolType` in the type namespace is first re-exported here


### warning: unused import: `std::sync::Arc`

**Total Occurrences**: 3  
**Unique Files**: 3

#### `src\query\optimizer\elimination_rules.rs`: 1 occurrences

- Line 4: unused import: `std::sync::Arc`

#### `src\query\optimizer\optimizer.rs`: 1 occurrences

- Line 7: unused import: `std::sync::Arc`

#### `src\query\planner\plan\core\nodes\sort_node.rs`: 1 occurrences

- Line 6: unused import: `std::sync::Arc`


### warning: variable does not need to be mutable

**Total Occurrences**: 3  
**Unique Files**: 2

#### `src\core\evaluator\expression_evaluator.rs`: 2 occurrences

- Line 243: variable does not need to be mutable
- Line 434: variable does not need to be mutable

#### `src\query\executor\data_processing\transformations\append_vertices.rs`: 1 occurrences

- Line 314: variable does not need to be mutable


### warning: unused import: `crate::query::planner::plan::core::nodes::plan_node_enum::PlanNodeEnum`

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\optimizer\join_optimization.rs`: 1 occurrences

- Line 8: unused import: `crate::query::planner::plan::core::nodes::plan_node_enum::PlanNodeEnum`

#### `src\query\optimizer\scan_optimization.rs`: 1 occurrences

- Line 8: unused import: `crate::query::planner::plan::core::nodes::plan_node_enum::PlanNodeEnum`


### warning: unused import: `crate::query::context::validate::types::Variable`

**Total Occurrences**: 2  
**Unique Files**: 2

#### `src\query\planner\plan\core\nodes\plan_node_enum.rs`: 1 occurrences

- Line 5: unused import: `crate::query::context::validate::types::Variable`

#### `src\query\planner\plan\management\ddl\edge_ops.rs`: 1 occurrences

- Line 5: unused import: `crate::query::context::validate::types::Variable`


### warning: unused import: `super::traits::StatsCache`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\cache\manager.rs`: 1 occurrences

- Line 12: unused import: `super::traits::StatsCache`


### warning: unused import: `std::time::Duration`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\cache\parser_cache.rs`: 1 occurrences

- Line 13: unused import: `std::time::Duration`


### warning: unused import: `crate::storage::native_storage::NativeStorage`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\core\context\runtime.rs`: 1 occurrences

- Line 18: unused import: `crate::storage::native_storage::NativeStorage`


### warning: unused imports: [identifier], [identifier], [identifier], [identifier], [identifier], [identifier], [identifier], [identifier], [identifier], [identifier], and [identifier]

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\optimizer\elimination_rules.rs`: 1 occurrences

- Line 11: unused imports: `AppendVerticesNode`, `DedupNode`, `GetEdgesNode`, `GetVerticesNode`, `InnerJoinNode`, `LeftJoinNode`, `LimitNode`, `ScanEdgesNode`, `ScanVerticesNode`, `SortNode`, and `StartNode`


### warning: unused import: `crate::query::planner::plan::algorithms::IndexScan`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\optimizer\elimination_rules.rs`: 1 occurrences

- Line 869: unused import: `crate::query::planner::plan::algorithms::IndexScan`


### warning: unused import: `super::filter_node::FilterNode`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\factory.rs`: 1 occurrences

- Line 10: unused import: `super::filter_node::FilterNode`


### warning: unused import: `super::join_node::InnerJoinNode`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\factory.rs`: 1 occurrences

- Line 14: unused import: `super::join_node::InnerJoinNode`


### warning: unused import: `super::project_node::ProjectNode`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\factory.rs`: 1 occurrences

- Line 16: unused import: `super::project_node::ProjectNode`


### warning: unused import: `super::plan_node_operations::*`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\plan_node_enum.rs`: 1 occurrences

- Line 359: unused import: `super::plan_node_operations::*`


### warning: unused import: `plan_node_operations::*`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\core\nodes\mod.rs`: 1 occurrences

- Line 33: unused import: `plan_node_operations::*`


### warning: unused import: `delete_ops::*`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\mod.rs`: 1 occurrences

- Line 10: unused import: `delete_ops::*`


### warning: unused import: `update_ops::*`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\plan\management\dml\mod.rs`: 1 occurrences

- Line 12: unused import: `update_ops::*`


### warning: unused import: `crate::core::ValueTypeDef`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\planner\match_planning\paths\shortest_path_planner.rs`: 1 occurrences

- Line 5: unused import: `crate::core::ValueTypeDef`


### warning: unused imports: [identifier], [identifier], and [identifier]

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\visitor\mod.rs`: 1 occurrences

- Line 5: unused imports: `VisitorContext`, `VisitorCore`, and `VisitorResult`


### warning: unused import: `crate::core::expressions::ExpressionContext`

**Total Occurrences**: 1  
**Unique Files**: 1

#### `src\query\context\validate\context.rs`: 1 occurrences

- Line 8: unused import: `crate::core::expressions::ExpressionContext`

