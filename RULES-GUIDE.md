# Rules guide

Polonius.next rules are defined in [a Soufflé rules file in the source directory](src/polonius.dl). You need to have [Soufflé](https://souffle-lang.github.io/install.html) installed to use them.

## Concepts

Before explaining the rules themselves, here are the core concepts of Polonius.next:

- **`Node`**: Node symbols in the Soufflé rules correspond to a single MIR statement in the input program. Those nodes are linked together by marking their possible control-flow successors.
- **Loan**: A loan is a construct representing an access constraint that must hold.
    + Loans are not concrete objects in the Soufflé rules but need to be considered when computing inputs for the ruleset.
    + Loans can either describe that a place can no longer be accessed mutably, or that a place can no longer be accessed at all.
    + Each loan is associated with a path representing the memory location it enforces restrictions about. For example, if a loan restricts access to the value `*(some_ref.0)`, this path will be associated to the loan.
    + Loans are created on borrowing sites, when a reference is created, and are live through a part of the program (similarly to how variables are live).
    + In tests, loans are often named after the path they restrict, for example `L*(some_ref.0)`.
- **`Origin`**: An origin symbol in the Soufflé rules corresponds to a lifetime symbol in a MIR program.
    + They can either be part of a type (like in a reference type) or be part of a borrowing operation (at a loan origin, the origin being the creation point of a loan).
    + Origins are named by the syntactic lifetime associated with their defining site.
    + Origins "contain" loans that must hold while the associated reference is live.
    + In tests, origins are often named as the loan they introduce (when they do), for example `'L*(some_ref.0)`. This is because those origins contain only one loan (the one they just created).

For example, consider the following MIR snippet:

```rs
let mut _x: &'a i32;

bb0: {
    _x = &'L_y _y;
    foo(_x) -> bb1;
}
```

- The contextual declaration of `_x` defines the origin `'a` as part of its type.
- The statement `_x = &'L_y y` is a `Node` that also defines the origin `'L_y`. This origin contains a single loan `L_y` describing that `_y` may only be used immutably.
- `foo(_x)` is also a `Node`, and it uses the origin `'a`.

Both nodes are linked, meaning that it is specified that the `_x = &'a y` node has only one successor, `foo(_x)`. Additionally, `foo(_x)` has the first statement of `bb1` as a successor node.

### Relationship between origins

Origins are partially ordered in a subset relationship. For two origins `'a` and `'b`, the subset relationship `'a <: 'b` holds if and only if `'a` contains all the loans in `'b`.

To help you build an intuition of this, keep in mind that increasing the amount of loans in an origin *restricts the origin further*. When `'a <: 'b` holds, that means that when `'b` is live, all the constraints of `'a` are also satisfied and thus `'a` can be used as well. Thus, this formulation is equivalent in terms of lifetimes to `'a` outlives `'b` (in Rust, `'a: 'b`).

In the previous example, the node `_x = &'L_y _y` enforces the subset relationship `'L_y <: 'a` (because `_x` is of type `&'a i32`). Indeed, if it is possible to put a reference with lifetime `'L_y` in a variable with lifetime `'a`, then `'a` must be used only when `'L_y` can be used, and thus `'a` must contain at least the loans in `'L_y`.

## Inputs

The caller must provide to the ruleset a few pre-computed facts about the program.

- `mark_as_loan_origin(Origin)`: This input differentiates origins that generate a new loan from origins that come from a type. If this input is set, the origin is assumed to be defined as part of a loan creation.
- `access_origin(Origin, Node)`: This input specifies that the provided node will use the reference associated with the provided origin. The reference is used in a way that does not conflict with the loan kind.
- `invalidate_origin(Origin, Node)`: This input specifies that the provided node will break the expectations of the loan defined at the provided origin. This can simply be computed by tracking which kind of loan each origin created and setting `invalidate_origin` for each node that invalidates it.
- `clear_origin(Origin, Node)`: This input specifies that the given origin symbol has been emptied of its meaning. This happens at loan creation or when the path attached to a loan is no longer reachable because one of its elements were overwritten. This does not mean the origin is invalidated, it just means that the origin in question does not point to the same memory anymore, and thus needs not care about constraints meant for the value it used to point to. Note that this is also used to mark the definition of the origin. That way, both definition and re-definition trigger a clear, which is useful to analyze the liveness of an origin.
- `introduce_subset(Origin1, Origin2, Node)`: This input specifies that at the specific node, there is a constraint that `Origin1` must be a subset of `Origin2`. This happens when a node links two origins together, for example when assigning a newly defined reference in a reference variable. Informally, this allows the rules to understand the lifetime inference constraints.
- `cfg_edge(Node1, Node2)`: Specifies `Node2` is a CFG successor of `Node1`.

### Input order

The effect of some of the previously mentionned inputs are ordered within a node. The node first does all of its accesses (`access_origin` and `invalidate_origin`), then clears the origins it shadows (`clear_origin`), then introduces potential new subset facts (`introduce_subset`). This ordering is important for the correctness of some of the rules.

## Rules

With this context, the details of the rules are not super tricky.

The core principle of Polonius.next is to find statements that access an origin that has previously been invalidated. Invalidating an origin is fine if that origin is never used afterwards (this is the point of Non-Lexical Lifetimes), and accessing an origin that has not previously been invalidated is also obviously fine, both at the same time however is invalid. This is computed by the `invalidated_origin_accessed` rule.

To do so, Polonius.next needs to compute which nodes have seen a specific origin been invalidated before. This is computed by `origin_invalidated`, which can be generated in two ways:

- If a predecessor invalidated the origin (and did not clear it), then it is still invalidated for its successors.
- If a predecessor invalidated a subset `O1` of the origin `O2`, this also invalidates `O2` and `O2` is still invalidated for its successors (no matter if `O1` gets cleared or not in the predecessor). This is where the subset relationship comes into play: this rule will also mark as invalidated all of the supersets of origins that get invalidated.

The rules `subset_on_entry` and `subset_on_exit` then handle tracking the subset relationship itself. Note that it does so only on origins that are live, that is origins that are used by a successor node without having been cleared in-between.

There are more details to the rules than presented in this guide, but hopefully now they should be clear to understand from the source code.
