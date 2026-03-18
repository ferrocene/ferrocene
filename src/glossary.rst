.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers
   SPDX-FileCopyrightText: The Rust Project Contributors

.. default-domain:: spec

.. informational-page::

Glossary
========

ABI
^^^

For :dt:`ABI`, see :t:`Application Binary Interface`.

ABI clobber
^^^^^^^^^^^

An :dt:`ABI clobber` is an argument to :t:`macro` :std:`core::arch::asm` which
indicates that the :t:`[value]s` of selected :t:`[register]s` might be
overwritten during the :t:`execution` of an :t:`assembly code block`.

See :s:`AbiClobber`.

ABI kind
^^^^^^^^

The :dt:`ABI kind` indicates the :t:`ABI` of a :t:`construct`.

See :s:`AbiKind`.

abort
^^^^^

:dt:`Abort` is the immediate termination of a program.

abstract data type
^^^^^^^^^^^^^^^^^^

An :dt:`abstract data type` is a collection of other :t:`[type]s`.

active attribute
^^^^^^^^^^^^^^^^

An :dt:`active attribute` is an :t:`attribute` that is removed from the
:t:`item` it decorates.

addition assignment
^^^^^^^^^^^^^^^^^^^

For :dt:`addition assignment`, see :t:`addition assignment expression`.

addition assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`addition assignment expression` is a
:t:`compound assignment expression` that uses addition.

See :s:`AdditionAssignmentExpression`.

addition expression
^^^^^^^^^^^^^^^^^^^

An :dt:`addition expression` is an :t:`arithmetic expression` that uses
addition.

See :s:`AdditionExpression`.

adjusted call operand
^^^^^^^^^^^^^^^^^^^^^

An :dt:`adjusted call operand` is a :t:`call operand` adjusted with inserted :t:`[borrow expression]s` and :t:`[dereference expression]s`.

alignment
^^^^^^^^^

The :dt:`alignment` of a :t:`value` specifies which addresses are valid for
storing the value.

all configuration predicate
^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`all configuration predicate` is a :t:`configuration predicate` that
models existential quantifier ALL.

See :s:`ConfigurationPredicateAll`.

anonymous loop expression
^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`anonymous loop expression` is a :t:`loop expression` without a
:t:`label`.

anonymous return type
^^^^^^^^^^^^^^^^^^^^^

An :dt:`anonymous return type` is an :t:`impl trait type` ascribed to a
:t:`function` return type.

anonymous type parameter
^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`anonymous type parameter` is an :t:`impl trait type` ascribed to a
:t:`function parameter`.

any configuration predicate
^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`any configuration predicate` is a :t:`configuration predicate` that
models existential quantifier ANY.

See :s:`ConfigurationPredicateAny`.

Application Binary Interface
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Application Binary Interface` is a set of conventions that dictate how
data and computation cross language boundaries.

See :s:`AbiSpecification`.

argument operand
^^^^^^^^^^^^^^^^

An :dt:`argument operand` is an :t:`operand` which is used as an argument in a
:t:`call expression` or a :t:`method call expression`.

arithmetic expression
^^^^^^^^^^^^^^^^^^^^^

An :dt:`arithmetic expression` is an :t:`expression` that computes a :t:`value`
from two :t:`[operand]s` using arithmetic.

See :s:`ArithmeticExpression`.

arithmetic operator
^^^^^^^^^^^^^^^^^^^

An :dt:`arithmetic operator` is the operator of an :t:`arithmetic expression`.

arithmetic overflow
^^^^^^^^^^^^^^^^^^^

An :dt:`arithmetic overflow` occurs if an :t:`arithmetic expression` or a
:t:`negation expression` computes a :t:`value` of a :t:`scalar type` that lies
outside of the range of valid :t:`[value]s` for the :t:`scalar type`.

arity
^^^^^

An :dt:`arity` is the number of :t:`[tuple field]s` in a :t:`tuple type`.

array
^^^^^

An :dt:`array` is a :t:`value` of an :t:`array type`.

array element constructor
^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`array element constructor` is an :t:`array expression` that lists all
elements of the :t:`array` being constructed.

See :s:`ArrayElementConstructor`.

array expression
^^^^^^^^^^^^^^^^

An :dt:`array expression` is an :t:`expression` that constructs an :t:`array`.

See :s:`ArrayExpression`.

array repetition constructor
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`array repetition constructor` is an :t:`array expression` that
specifies how many times an element is repeated in the :t:`array` being
constructed.

See :s:`ArrayRepetitionConstructor`.

array type
^^^^^^^^^^

An :dt:`array type` is a :t:`sequence type` that represents a fixed sequence of
elements.

See :s:`ArrayTypeSpecification`.

assembly code block
^^^^^^^^^^^^^^^^^^^

An :dt:`assembly code block` is a sequence of :t:`[assembly instruction]s`.

See :s:`AssemblyCodeBlock`.

assembly directive
^^^^^^^^^^^^^^^^^^

An :dt:`assembly directive` is a request to the assembler to perform a
particular action or change a setting.

assembly instruction
^^^^^^^^^^^^^^^^^^^^

An :dt:`assembly instruction` is a :t:`string literal` that represents a
low-level assembly operation or an :t:`assembly directive`.

See :s:`AssemblyInstruction`.

assembly option
^^^^^^^^^^^^^^^

An :dt:`assembly option` is used to specify a characteristic of or a restriction
on the related :t:`assembly code block`.

See :s:`AssemblyOption`.

assigned operand
^^^^^^^^^^^^^^^^

An :dt:`assigned operand` is the target :t:`operand` of a
:t:`compound assignment expression`.

See :s:`AssignedOperand`.

assignee expression
^^^^^^^^^^^^^^^^^^^

An :dt:`assignee expression` is an :t:`expression` that appears as the
:t:`left operand` of an :t:`assignment expression`.

assignee operand
^^^^^^^^^^^^^^^^

An :dt:`assignee operand` is the target :t:`operand` of an
:t:`assignment expression`.

See :s:`AssigneeOperand`.

assignment
^^^^^^^^^^

See :t:`assignment expression`.

assignment expression
^^^^^^^^^^^^^^^^^^^^^

An :dt:`assignment expression` is an :t:`expression` that assigns the
:t:`value` of a :t:`value operand` to an :t:`assignee operand`.

See :s:`AssignmentExpression`.

associated constant
^^^^^^^^^^^^^^^^^^^

An :dt:`associated constant` is a :t:`constant` that appears as an
:t:`associated item`.

associated function
^^^^^^^^^^^^^^^^^^^

An :dt:`associated function` is a :t:`function` that appears as an
:t:`associated item`.

associated implementation constant
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated implementation constant` is an :t:`associated constant` that
appears within an :t:`implementation`.

associated implementation function
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated implementation function` is an :t:`associated function` that
appears within an :t:`implementation`.

associated implementation type
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated implementation type` is an :t:`associated type` that appears
within an :t:`implementation`.

associated item
^^^^^^^^^^^^^^^

An :dt:`associated item` is an :t:`item` that appears within an
:t:`implementation` or a :t:`trait`.

See :s:`AssociatedItem`.

associated trait constant
^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated trait constant` is an :t:`associated constant` that appears
within a :t:`trait`.

associated trait function
^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated trait function` is an :t:`associated function` that appears
within a :t:`trait`.

associated trait implementation function
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated trait implementation function` is an :t:`associated function`
that appears within a :t:`trait implementation`.

associated trait implementation item
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated trait implementation item` is an :t:`associated item` that
appears within a :t:`trait implementation`.

associated trait item
^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated trait item` is an :t:`associated item` that appears
within a :t:`trait`.

associated trait type
^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated trait type` is an :t:`associated type` that appears within
a :t:`trait`.

associated type
^^^^^^^^^^^^^^^

An :dt:`associated type` is a :t:`type alias` that appears as an
:t:`associated item`.

associated type projection
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`associated type projection` is a :t:`qualified type path` of the form
``<type as trait>::associated_type``, where ``type`` is a :t:`type`, ``trait``
is a :t:`qualifying trait`, and ``associated type`` is an :t:`associated type`.

associativity
^^^^^^^^^^^^^

:dt:`Associativity` is the order by which :t:`[operand]s` are evaluated within
a single :t:`expression`.

async block
^^^^^^^^^^^

For :dt:`async block`, see :t:`async block expression`.

async block expression
^^^^^^^^^^^^^^^^^^^^^^

An :dt:`async block expression` is a :t:`block expression` that is specified
with :t:`keyword` ``async`` and encapsulates behavior which is executed in
an asynchronous manner.

See :s:`AsyncBlockExpression`.

async closure expression
^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`async closure expression` is a :t:`closure expression` subject to keyword ``async`` that defines an :t:`async closure type` and constructs a value of that :t:`type`.

See :s:`ClosureExpression`.

async closure type
^^^^^^^^^^^^^^^^^^

An :dt:`async closure type` is a unique anonymous :t:`function type` that encapsulates
all :t:`[capture target]s` of a :t:`closure expression` producing a :std:`core::future::Future`.


async control flow boundary
^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`async control flow boundary` is a :t:`control flow boundary` that
additionally allows the suspension of execution via :t:`[await expression]s`.

async function
^^^^^^^^^^^^^^

An :dt:`async function` is a :t:`function` subject to :t:`keyword` ``async``.

atomic
^^^^^^

See :t:`atomic type`.

atomic type
^^^^^^^^^^^

An :dt:`atomic type` is a :t:`type` defined in :t:`module`
:std:`core::sync::atomic`.

attribute
^^^^^^^^^

An :dt:`attribute` is a general, free-form metadatum that is interpreted based
on its name, convention, language, and tool.

attribute content
^^^^^^^^^^^^^^^^^

An :dt:`attribute content` is a :t:`construct` that provides the content of
an :t:`attribute`.

See :s:`AttributeContent`.

attribute macro
^^^^^^^^^^^^^^^

An :dt:`attribute macro` is a :t:`procedural macro` that consumes two streams
of :t:`[token]s` to produce a stream of tokens, and defines a new
:t:`outer attribute` that can be attached to :t:`[item]s`.

auto trait
^^^^^^^^^^

An :dt:`auto trait` is a :t:`trait` that is implicitly and automatically
implemented by a :t:`type` when the types of its constituent :t:`[field]s`
implement the :t:`trait`.

await expression
^^^^^^^^^^^^^^^^

An :dt:`await expression` is an :t:`expression` that polls a :t:`future`,
suspending the execution of the future until the future is ready.

See :s:`AwaitExpression`.

base initializer
^^^^^^^^^^^^^^^^

A :dt:`base initializer` is a :t:`construct` that specifies an :t:`enum value`,
a :t:`struct value`, or a :t:`union value` to be used as a base for
construction in a :t:`struct expression`.

See :s:`BaseInitializer`.

basic assignment
^^^^^^^^^^^^^^^^

A :dt:`basic assignment` is an :t:`assignment expression` that is not a
:t:`destructuring assignment`.

binary crate
^^^^^^^^^^^^

A :dt:`binary crate` is a :t:`crate` whose :t:`crate type` is ``bin``.

binary literal
^^^^^^^^^^^^^^

A :dt:`binary literal` is an :t:`integer literal` in base 2.

See :s:`BinaryLiteral`.

binary operator
^^^^^^^^^^^^^^^

A :dt:`binary operator` is an operator that operates on two :t:`[operand]s`.

binding
^^^^^^^

A :dt:`binding` of a :t:`binding pattern` binds a matched :t:`value` to a
:t:`name`.

See :s:`Binding`.

binding argument
^^^^^^^^^^^^^^^^

A :dt:`binding argument` is a :t:`generic argument` that supplies the :t:`type`
of an :t:`associated trait type`.

binding bound argument
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`binding bound argument` is a :t:`generic argument` that further imposes
:t:`[bound]s` on an :t:`associated trait type`.

binding mode
^^^^^^^^^^^^

:dt:`Binding mode` is the mechanism by which a matched :t:`value` is bound to a
:t:`binding` of a :t:`pattern`.

binding pattern
^^^^^^^^^^^^^^^

A :dt:`binding pattern` is either an :t:`identifier pattern` or a
:t:`shorthand deconstructor`.

binding scope
^^^^^^^^^^^^^

A :dt:`binding scope` is a :t:`scope` for :t:`[binding]s`.

bit and assignment
^^^^^^^^^^^^^^^^^^

For :dt:`bit and assignment`, see :t:`bit and assignment expression`.

bit and assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`bit and assignment expression` is a :t:`compound assignment expression`
that uses bit and arithmetic.

See :s:`BitAndAssignmentExpression`.

bit and expression
^^^^^^^^^^^^^^^^^^

A :dt:`bit and expression` is a :t:`bit expression` that uses bit and
arithmetic.

See :s:`BitAndExpression`.

bit expression
^^^^^^^^^^^^^^

A :dt:`bit expression` is an :t:`expression` that computes a :t:`value` from
two :t:`[operand]s` using bit arithmetic.

See :s:`BitExpression`.

bit or assignment
^^^^^^^^^^^^^^^^^

For :dt:`bit or assignment`, see :t:`bit or assignment expression`.

bit or assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`bit or assignment expression` is a :t:`compound assignment expression`
that uses bit or arithmetic.

See :s:`BitOrAssignmentExpression`.

bit or expression
^^^^^^^^^^^^^^^^^

A :dt:`bit or expression` is a :t:`bit expression` that uses bit or arithmetic.

See :s:`BitOrExpression`.

bit xor assignment
^^^^^^^^^^^^^^^^^^

For :dt:`bit xor assignment`, see :t:`bit xor assignment expression`.

bit xor assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`bit xor assignment expression` is a :t:`compound assignment expression`
that uses bit exclusive or arithmetic.

See :s:`BitXorAssignmentExpression`.

bit xor expression
^^^^^^^^^^^^^^^^^^

A :dt:`bit xor expression` is a :t:`bit expression` that uses bit exclusive or
arithmetic.

See :s:`BitXorExpression`.

block comment
^^^^^^^^^^^^^

A :dt:`block comment` is a :t:`comment` that spans one or more :t:`[line]s`.

See :s:`BlockComment`.

block expression
^^^^^^^^^^^^^^^^

A :dt:`block expression` is an :t:`expression` that sequences expressions and
:t:`[statement]s`.

See :s:`BlockExpression`.

bool
^^^^

:dc:`bool` is a :t:`type` whose :t:`[value]s` denote the truth values of logic
and Boolean algebra.

boolean literal
^^^^^^^^^^^^^^^

A :dt:`boolean literal` is a :t:`literal` that denotes the truth :t:`[value]s`
of logic and Boolean algebra.

See :s:`BooleanLiteral`.

borrow
^^^^^^

A :dt:`borrow` is a :t:`reference` produced by :t:`borrowing`.

borrow expression
^^^^^^^^^^^^^^^^^

A :dt:`borrow expression` is an :t:`expression` that borrows the :t:`value`
of its :t:`operand` and creates a :t:`reference` to the memory location of its
operand.

See :s:`BorrowExpression`.

borrowed
^^^^^^^^

A memory location is :dt:`borrowed` when a :t:`reference` pointing to it is
:t:`active`.

borrowing
^^^^^^^^^

:dt:`Borrowing` is the process of temporarily associating a :t:`reference` with
a :t:`value` without transferring :t:`ownership` permanently.

bound
^^^^^

A :dt:`bound` imposes a constraint on a :t:`generic parameter` by limiting the
set of possible :t:`[generic substitution]s`.

See :s:`TypeBound`.

bound pattern
^^^^^^^^^^^^^

A :dt:`bound pattern` is a :t:`pattern` that imposes a constraint on a related
:t:`identifier pattern`.

See :s:`BoundPattern`.

break expression
^^^^^^^^^^^^^^^^

A :dt:`break expression` is an :t:`expression` that terminates a
:t:`loop expression` or a :t:`named block expression`.

See :s:`BreakExpression`.

break type
^^^^^^^^^^

:dt:`Break type` is the :t:`type` of the :t:`operand` of a
:t:`break expression`.

break value
^^^^^^^^^^^

:dt:`Break value` is the :t:`value` of the :t:`operand` of a
:t:`break expression`.

built-in attribute
^^^^^^^^^^^^^^^^^^

A :dt:`built-in attribute` is a language-defined :t:`attribute`.

See :s:`InnerBuiltinAttribute`, :s:`OuterBuiltinAttribute`.

built-in trait
^^^^^^^^^^^^^^

A :dt:`built-in trait` is a language-defined :t:`trait`.

byte literal
^^^^^^^^^^^^

A :dt:`byte literal` is a :t:`literal` that denotes a fixed byte :t:`value`.

See :s:`ByteLiteral`.

byte string literal
^^^^^^^^^^^^^^^^^^^

A :dt:`byte string literal` is a :t:`literal` that consists of multiple
:s:`[AsciiCharacter]s`.

See :s:`ByteStringLiteral`.

C
^

:dt:`C` is the programming language described in the ISO/IEC 9899:2018
International Standard.

C representation
^^^^^^^^^^^^^^^^

:dt:`C representation` is a :t:`type representation` that lays out :t:`[type]s`
such that they are interoperable with the :t:`C` language.

C signed int type
^^^^^^^^^^^^^^^^^

:dt:`C signed int type` is the `signed int` :t:`type` of the :t:`C` language.

c string literal
^^^^^^^^^^^^^^^^

A :dt:`c string literal` is a :t:`literal` that consists of multiple characters
with an implicit 0x00 byte appended to it.

See :s:`CStringLiteral`.

Call conformance
^^^^^^^^^^^^^^^^

:dt:`Call conformance` measures the compatibility between a set of
:t:`[argument operand]s` and a set if :t:`[function parameter]s` or
:t:`[field]s`.

call expression
^^^^^^^^^^^^^^^

A :dt:`call expression` is an :t:`expression` that invokes a :t:`function` or
constructs a :t:`tuple struct value` or :t:`tuple enum variant value`.

See :s:`CallExpression`.

call operand
^^^^^^^^^^^^

A :dt:`call operand` is the :t:`function` being invoked or the
:t:`tuple struct value` or :t:`tuple enum variant value` being constructed by a
:t:`call expression`.

See :s:`CallOperand`.

call resolution
^^^^^^^^^^^^^^^

:dt:`Call resolution` is a kind of :t:`resolution` that applies to a
:t:`call expression`.


call site hygiene
^^^^^^^^^^^^^^^^^

:dt:`Call site hygiene` is a type of :t:`hygiene` which resolves to the
:s:`MacroInvocation` site. :t:`[Identifier]s` with :t:`call site hygiene` can
reference the environment of the :s:`MacroRulesDeclaration`, can reference the
environment of the :s:`MacroInvocation`, and are considered :t:`unhygienic`.

callee type
^^^^^^^^^^^

A :dt:`callee type` is either a :t:`function item type`, a
:t:`function pointer type`, a :t:`tuple struct type`, a :t:`tuple enum variant`
or a :t:`type` that implements any of the :std:`core::ops::Fn`,
:std:`core::ops::FnMut`, or :std:`core::ops::FnOnce` :t:`[trait]s`.

capture mode
^^^^^^^^^^^^

:dt:`Capture mode` is the mechanism by which a :t:`capture target` is captured.

capture target
^^^^^^^^^^^^^^

A :dt:`capture target` is either a :t:`binding` or a :t:`field` of a
:t:`binding`.

capturing
^^^^^^^^^

:dt:`Capturing` is the process of saving the :t:`[capture target]s` of a
:t:`[capturing expression]'s` :t:`capturing environment`.

capturing environment
^^^^^^^^^^^^^^^^^^^^^

The :dt:`capturing environment` of a :t:`capturing expression` consists of all
:t:`[capture target]s` that are defined outside the :t:`capturing expression`.

capturing expression
^^^^^^^^^^^^^^^^^^^^

A :dt:`capturing expression` is either an :t:`async block expression` or a
:t:`closure expression`.

cast
^^^^

:dt:`Cast` or :dt:`casting` is the process of changing the :t:`type` of an
:t:`expression`.

char
^^^^

:dc:`char` is a :t:`type` whose :t:`[value]s` denote :t:`Unicode` characters.

character literal
^^^^^^^^^^^^^^^^^

A :dt:`character literal` is a :t:`literal` that denotes a fixed :t:`Unicode`
character.

See :s:`CharacterLiteral`.

closure body
^^^^^^^^^^^^

A :dt:`closure body` is a :t:`construct` that represents the executable portion
of a :t:`closure expression`.

See :s:`ClosureBody`, :s:`ClosureBodyWithReturnType`.

closure expression
^^^^^^^^^^^^^^^^^^

A :dt:`closure expression` is an :t:`expression` that defines a
:t:`closure type` and constructs a value of that :t:`type`.

See :s:`ClosureExpression`.

closure parameter
^^^^^^^^^^^^^^^^^

A :dt:`closure parameter` is a :t:`construct` that yields a set of
:t:`[binding]s` that bind matched input :t:`[value]s` to :t:`[name]s` at the
site of a :t:`call expression` or a :t:`method call expression`.

See :s:`ClosureParameter`.

closure type
^^^^^^^^^^^^

A :dt:`closure type` is a unique anonymous :t:`function type` that encapsulates
all :t:`[capture target]s` of a :t:`closure expression`.

code point
^^^^^^^^^^

In :t:`Unicode`, a :dt:`code point` is a numeric :t:`value` that maps to a
character.

comment
^^^^^^^

A :dt:`comment` is a :t:`lexical element` that acts as an annotation or an
explanation in program text.

See :s:`Comment`.

comparison expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`comparison expression` is an :t:`expression` that compares the
:t:`[value]s` of two :t:`[operand]s`.

See :s:`ComparisonExpression`.

compilation root
^^^^^^^^^^^^^^^^

A :dt:`compilation root` is an input to a compilation performed by a tool.

compound assignment
^^^^^^^^^^^^^^^^^^^

For :dt:`compound assignment`, see :t:`compound assignment expression`.

compound assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`compound assignment expression` is an expression that first computes
a :t:`value` from two :t:`[operand]s` and then assigns the value to an
:t:`assigned operand`.

See :s:`CompoundAssignmentExpression`.

concrete type
^^^^^^^^^^^^^

A :dt:`concrete type` is a :t:`type` described by a :t:`type specification`.

conditional compilation
^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Conditional compilation` is the process of compiling
:t:`conditionally-compiled source code`.

conditionally-compiled source code
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Conditionally-compiled source code` is source code that may or may not be
considered a part of a Rust program depending on certain conditions.

configuration predicate
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`configuration predicate` is a :t:`construct` that evaluates statically
to either ``true`` or ``false``, and controls :t:`conditional compilation`.

See :s:`ConfigurationPredicate`.

const block expression
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`const block expression` is a :t:`block expression` that is specified
with :t:`keyword` ``const`` and encapsulates behavior which is evaluated
statically.

constant
^^^^^^^^

A :dt:`constant` is an immutable :t:`value` whose uses are substituted by the
:t:`value`.

See :s:`ConstantDeclaration`.

constant argument
^^^^^^^^^^^^^^^^^

A :dt:`constant argument` is a :t:`generic argument` that supplies the
:t:`value` of a :t:`constant parameter`.

See :s:`ConstantArgument`.

constant context
^^^^^^^^^^^^^^^^

A :dt:`constant context` is a :t:`construct` that requires a
:t:`constant expression`.

constant expression
^^^^^^^^^^^^^^^^^^^

A :dt:`constant expression` is an :t:`expression` that can be evaluated
statically.

constant function
^^^^^^^^^^^^^^^^^

A :dt:`constant function` is a :t:`function` subject to :t:`keyword` ``const``.

constant initializer
^^^^^^^^^^^^^^^^^^^^

A :dt:`constant initializer` is a :t:`construct` that provides the :t:`value`
of its related :t:`constant`.

See :s:`ConstantInitializer`.

constant parameter
^^^^^^^^^^^^^^^^^^

A :dt:`constant parameter` is a :t:`generic parameter` for a :t:`constant`.

See :s:`ConstantParameter`.

constant parameter initializer
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`constant parameter initializer` is a :t:`construct` that provides the
default `:t:`value` of its related :t:`constant parameter`.

See :s:`ConstantParameterInitializer`.

constant promotion
^^^^^^^^^^^^^^^^^^

:dt:`Constant promotion` is the process of converting a :t:`value expression`
into a :t:`constant`.

constrain
^^^^^^^^^

A :t:`generic parameter` is said to :dt:`constrain` an :t:`implementation` if
it makes the :t:`[implementation]'s` applicability more narrow.

construct
^^^^^^^^^

A :dt:`construct` is a piece of program text that is an instance of a
:t:`syntactic category`.

constructee
^^^^^^^^^^^

A :dt:`constructee` indicates the :t:`enum variant`, :t:`struct` or :t:`union`
whose value is being constructed by a :t:`struct expression`.

container operand
^^^^^^^^^^^^^^^^^

A :dt:`container operand` is an :t:`operand` that indicates the :t:`value`
whose :t:`field` is selected in a :t:`field access expression`.

See :s:`ContainerOperand`.

continue expression
^^^^^^^^^^^^^^^^^^^

A :dt:`continue expression` is an :t:`expression` that first terminates and
then restarts a :t:`loop expression`.

See :s:`ContinueExpression`.

control flow boundary
^^^^^^^^^^^^^^^^^^^^^

A :dt:`control flow boundary` is a :t:`construct` that limits control flow from
returning beyond the :t:`construct`, and acts as the target of control flow
returning operations.

copy type
^^^^^^^^^

A :dt:`copy type` is a :t:`type` that implements the
:std:`core::marker::Copy` :t:`trait`.

crate
^^^^^

A :dt:`crate` is a unit of compilation and linking that contains a tree of
nested :t:`[module]s`.

crate import
^^^^^^^^^^^^

A :dt:`crate import` specifies a dependency on an external :t:`crate`.

See :s:`ExternalCrateImport`.

crate indication
^^^^^^^^^^^^^^^^

A :dt:`crate indication` is a :t:`construct` that indicates a :t:`crate`.

See :s:`CrateIndication`.

crate public modifier
^^^^^^^^^^^^^^^^^^^^^

A :dt:`crate public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility` within the current :t:`crate` only.

See :s:`CratePublicModifier`.

crate root
^^^^^^^^^^

A :dt:`crate root` is an entry point into a :t:`crate`.

crate root module
^^^^^^^^^^^^^^^^^

A :dt:`crate root module` is the root of the nested :t:`module` tree of a
:t:`crate`.

crate type
^^^^^^^^^^

The :dt:`crate type` of a :t:`crate` is the value of the :t:`attribute`
``crate_type`` of a :t:`crate` or the value of ``--crate-type`` flag passed to
the tool compiling the :t:`crate`.

dangling
^^^^^^^^

A :t:`value` of an :t:`indirection type` is :dt:`dangling` if it is either
:c:`null` or not all of the bytes at the referred memory location are part of
the same allocation.

data race
^^^^^^^^^

A :dt:`data race` is a scenario where two or more threads access a shared
memory location concurrently.

decimal literal
^^^^^^^^^^^^^^^

A :dt:`decimal literal` is an :t:`integer literal` in base 10.

See :s:`DecimalLiteral`.

declaration
^^^^^^^^^^^

A :dt:`declaration` is a :t:`construct` that introduces a :t:`name` for an
:t:`entity`.

declarative macro
^^^^^^^^^^^^^^^^^

A :dt:`declarative macro` is a :t:`macro` that associates a :t:`name` with a
set of syntactic transformation rules.

See :s:`MacroRulesDeclaration`.

deconstructee
^^^^^^^^^^^^^

A :dt:`deconstructee` indicates the :t:`enum variant` or :t:`type` that is
being deconstructed by a :t:`struct pattern`.

See :s:`Deconstructee`.

default representation
^^^^^^^^^^^^^^^^^^^^^^

:dt:`Default representation` is a :t:`type representation` that does not make
any guarantees about :t:`layout`.

definition site hygiene
^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Definition site hygiene` is a type of :t:`hygiene` which resolves to the
:s:`MacroRulesDeclaration` site. :t:`[Identifier]s` with
:t:`definition site hygiene` cannot reference the environment of the
:s:`MacroRulesDeclaration`, cannot be referenced by the environment of a
:s:`MacroInvocation`, and are considered :t:`hygienic`.

dereference
^^^^^^^^^^^

A :dt:`dereference` is the memory location produced by evaluating a
:t:`dereference expression`.

dereference expression
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`dereference expression` is an :t:`expression` that obtains the
pointed-to memory location of its :t:`operand`.

See :s:`DereferenceExpression`.

dereference type
^^^^^^^^^^^^^^^^

A :dt:`dereference type` is either a :t:`reference type` or a :t:`type` that
implements the :std:`core::ops::Deref` :t:`trait`.

dereference type chain
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`dereference type chain` is a sequence of :t:`[dereference type]s`.

derive macro
^^^^^^^^^^^^

A :dt:`derive macro` is a :t:`procedural macro` that consumes a stream of
:t:`[token]s` and produces a stream of tokens, and is invoked via attribute
:c:`derive`.

destruction
^^^^^^^^^^^

:dt:`Destruction` is the process of recovering resources associated with a
:t:`value` as it goes out of scope.

destructor
^^^^^^^^^^

A :dt:`destructor` is a :t:`function` that is invoked immediately before the
:t:`destruction` of a :t:`value` of a :t:`drop type`.

destructuring assignment
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`destructuring assignment` is an :t:`assignment expression` where
the :t:`assignee operand` is either an :t:`array expression`, a
:t:`struct expression`, or a :t:`tuple expression`.

direction modifier
^^^^^^^^^^^^^^^^^^

A :dt:`direction modifier` is a :t:`construct` that indicates whether a
:t:`register argument` initializes a :t:`register`, assigns the :t:`value` of a
:t:`register` to an :t:`expression`, or both.

See :s:`DirectionModifier`.

discriminant
^^^^^^^^^^^^

A :dt:`discriminant` is an opaque integer that identifies an :t:`enum variant`.

discriminant initializer
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`discriminant initializer` provides the :t:`value` of a :t:`discriminant`.

See :s:`DiscriminantInitializer`.

discriminant type
^^^^^^^^^^^^^^^^^

A :dt:`discriminant type` is the :t:`type` of a :t:`discriminant`.

diverging expression
^^^^^^^^^^^^^^^^^^^^

A :dt:`diverging expression` is an :t:`expression` whose :t:`evaluation` causes
program flow to diverge from the normal :t:`evaluation` order.

diverging type variable
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`diverging type variable` is a :t:`type variable` that can refer to any
:t:`type` and originates from a :t:`diverging expression`.

division assignment
^^^^^^^^^^^^^^^^^^^

For :dt:`division assignment`, see :t:`division assignment expression`.

division assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`division assignment expression` is a :t:`compound assignment expression`
that uses division.

See :s:`DivisionAssignmentExpression`.

division expression
^^^^^^^^^^^^^^^^^^^

A :dt:`division expression` is an :t:`arithmetic expression` that uses division.

See :s:`DivisionExpression`.

doc comment
^^^^^^^^^^^

A :dt:`doc comment` is a :t:`comment` class that includes
:t:`[inner block doc]s`, :t:`[inner line doc]s`, :t:`[outer block doc]s`,
and :t:`[outer line doc]s`.

drop construct
^^^^^^^^^^^^^^

A :dt:`drop construct` is a :t:`construct` that employs a :t:`drop scope`.

drop order
^^^^^^^^^^

:dt:`Drop order` is the order by which :t:`[value]s` are :t:`dropped` when a
:t:`drop scope` is left.

drop scope
^^^^^^^^^^

A :dt:`drop scope` is a region of program text that governs the :t:`dropping`
of :t:`[value]s`.

drop scope extension
^^^^^^^^^^^^^^^^^^^^

:dt:`Drop scope extension` is the process of extending a :t:`drop scope`
associated with a :t:`temporary` to prevent the premature :t:`dropping` of the
:t:`temporary`.

drop type
^^^^^^^^^

A :dt:`drop type` is a :t:`type` that implements the :std:`core::ops::Drop`
:t:`trait` or contains a :t:`field` that has a :t:`destructor`.

dropping
^^^^^^^^

:dt:`Dropping` a :t:`value` is the act of invoking the :t:`destructor` of the
related :t:`type`.

dynamically sized type
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`dynamically sized type` is a :t:`type` that does not implement the
:std:`core::marker::Sized` :t:`trait`.

elaboration
^^^^^^^^^^^

:dt:`Elaboration` is the process by which a :t:`declaration` achieves its
runtime effects.

element type
^^^^^^^^^^^^

An :dt:`element type` is the :t:`type` of the elements of an :t:`array type` or
a :t:`slice type`.

See :s:`ElementType`.

elided
^^^^^^

For :dt:`elided`, see :t:`elided lifetime`.

elided lifetime
^^^^^^^^^^^^^^^

An :dt:`elided lifetime` is either an :t:`unnamed lifetime` or a :t:`lifetime`
that has been explicitly omitted from a :t:`function signature` or an
:t:`implementation`.

else expression
^^^^^^^^^^^^^^^

An :dt:`else expression` is an :t:`expression` that represents either a
:t:`block expression`, an :t:`if expression`, or an :t:`if let expression`.

See :s:`ElseExpression`.

empty statement
^^^^^^^^^^^^^^^

An :dt:`empty statement` is a :t:`statement` expressed as character 0x3B
(semicolon).

entity
^^^^^^

An :dt:`entity` is a :t:`construct` that can be referred to within program
text, usually via a :t:`field access expression` or a :t:`path`.

enum
^^^^

An :dt:`enum` is an :t:`item` that declares an :t:`enum type`.

enum field
^^^^^^^^^^

An :dt:`enum field` is a :t:`field` of an :t:`enum variant`.

enum type
^^^^^^^^^

An :dt:`enum type` is an :t:`abstract data type` that contains
:t:`[enum variant]s`.

See :s:`EnumDeclaration`.

enum value
^^^^^^^^^^

An :dt:`enum value` is a :t:`value` of an :t:`enum type`.

enum variant
^^^^^^^^^^^^

An :dt:`enum variant` is a :t:`construct` that declares one of the
possible variations of an :t:`enum`.

See :s:`EnumVariant`.

enum variant value
^^^^^^^^^^^^^^^^^^

An :dt:`enum variant value` is the :t:`enum value` of the corresponding
:t:`enum` of the :t:`enum variant`.

equals expression
^^^^^^^^^^^^^^^^^

An :dt:`equals expression` is a :t:`comparison expression` that tests equality.

See :s:`EqualsExpression`.

error propagation expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`error propagation expression` is an :t:`expression` that either
evaluates to a :t:`value` of its :t:`operand` or returns a value to the next
control flow boundary.

See :s:`ErrorPropagationExpression`.

escaped character
^^^^^^^^^^^^^^^^^

An :dt:`escaped character` is the textual representation for a character with
special meaning. An escaped character consists of character 0x5C (reverse
solidus), followed by the single character encoding of the special meaning
character. For example, ``\t`` is the escaped character for 0x09 (horizontal
tabulation).

evaluated
^^^^^^^^^

See :t:`evaluation`.

evaluation
^^^^^^^^^^

:dt:`Evaluation` is the process by which an :t:`expression` achieves its
runtime effects.

exclusive range pattern
^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`exclusive range pattern` is a :t:`range pattern` with both a
:t:`range pattern low bound` and a :t:`range pattern high bound`.

See :s:`ExclusiveRangePattern`.

executed
^^^^^^^^

See :t:`execution`.

execution
^^^^^^^^^

:dt:`Execution` is the process by which a :t:`statement` achieves its runtime
effects.

explicit register argument
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`explicit register argument` is a :t:`register argument` that uses an
:t:`explicit register name`.

explicit register name
^^^^^^^^^^^^^^^^^^^^^^

An :dt:`explicit register name` is a target-specific string that identifies
a :t:`register`.

See :s:`ExplicitRegisterName`.

explicitly declared entity
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`explicitly declared entity` is an :t:`entity` that has a
:t:`declaration`.

exported function
^^^^^^^^^^^^^^^^^

An :dt:`exported function` is an export of a :t:`function`.

exported static
^^^^^^^^^^^^^^^^^

An :dt:`exported static` is an export of a :t:`static`.

expression
^^^^^^^^^^

An :dt:`expression` is a :t:`construct` that produces a :t:`value`, and may
have side effects at run-time.

See :s:`Expression`.

expression statement
^^^^^^^^^^^^^^^^^^^^

An :dt:`expression statement` is an :t:`expression` whose result is ignored.

See :s:`ExpressionStatement`.

expression-with-block
^^^^^^^^^^^^^^^^^^^^^

An :dt:`expression-with-block` is an :t:`expression` whose structure involves a
:t:`block expression`.

See :s:`ExpressionWithBlock`.

expression-without-block
^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`expression-without-block` is an :t:`expression` whose structure does
not involve a :t:`block expression`.

See :s:`ExpressionWithoutBlock`.

external block
^^^^^^^^^^^^^^

An :dt:`external block` is a :t:`construct` that provides the declarations of
foreign :t:`[function]s` as unchecked imports.

See :s:`ExternalBlock`.

external function
^^^^^^^^^^^^^^^^^

An :dt:`external function` is an unchecked import of a foreign :t:`function`.

external function item type
^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`external function item type` is a :t:`function item type` where the
related :t:`function` is an :t:`external function`.

external static
^^^^^^^^^^^^^^^

An :dt:`external static` is an import of a foreign :t:`variable`.

f32
^^^

:dc:`f32` is a :t:`floating-point type` equivalent to the IEEE 754-2008
binary32 :t:`type`.

f64
^^^

:dc:`f64` is a :t:`floating-point type` equivalent to the IEEE 754-2008
binary64 :t:`type`.

fat pointer
^^^^^^^^^^^

A :dt:`fat pointer` is a :t:`value` of a :t:`fat pointer type`.

fat pointer type
^^^^^^^^^^^^^^^^

A :dt:`fat pointer type` is an :t:`indirection type` whose contained :t:`type specification` is a :t:`dynamically sized type`.

FFI
^^^

For :dt:`FFI`, see :t:`Foreign Function Interface`.

field
^^^^^

A :dt:`field` is an element of an :t:`abstract data type`.

field access expression
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`field access expression` is an :t:`expression` that accesses a
:t:`field` of a :t:`value`.

See :s:`FieldAccessExpression`.

field index
^^^^^^^^^^^

A :dt:`field index` is the position of a :t:`field` within a
:t:`tuple struct type` or :t:`tuple enum variant`. The first :t:`field` has a
:t:`field index` of zero, the Nth :t:`field` has a :t:`field index` of N-1.

See :s:`FieldIndex`.

field list
^^^^^^^^^^

A :dt:`field list` is a :s:`RecordStructFieldList` or :s:`TupleStructFieldList`.

field resolution
^^^^^^^^^^^^^^^^

:dt:`Field resolution` is a form of :t:`resolution` that applies to a
:t:`field access expression`.

field selector
^^^^^^^^^^^^^^

A :dt:`field selector` is a :t:`construct` that selects the :t:`field` to be
accessed in a :t:`field access expression`.

See :s:`FieldSelector`.

final match arm
^^^^^^^^^^^^^^^

A :dt:`final match arm` is the last :t:`match arm` of a :t:`match expression`.

See :s:`FinalMatchArm`.

fixed sized type
^^^^^^^^^^^^^^^^

A :dt:`fixed sized type` is a :t:`type` that implements the
:std:`core::marker::Sized` :t:`trait`.

float literal
^^^^^^^^^^^^^

A :dt:`float literal` is a :t:`numeric literal` that denotes a fractional
number.

See :s:`FloatLiteral`.

float suffix
^^^^^^^^^^^^

A :dt:`float suffix` is a component of a :t:`float literal` that specifies an
explicit :t:`floating-point type`.

See :s:`FloatSuffix`.

floating-point type
^^^^^^^^^^^^^^^^^^^

A :dt:`floating-point type` is a :t:`numeric type` whose :t:`[value]s` denote
fractional numbers.

floating-point type variable
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`floating-point type variable` is a :t:`type variable` that can refer
only to :t:`[floating-point type]s`.

floating-point value
^^^^^^^^^^^^^^^^^^^^

A :dt:`floating-point value` is a :t:`value` of a :t:`floating-point type`.

for loop
^^^^^^^^

For :dt:`for loop`, see :t:`for loop expression`.

for loop expression
^^^^^^^^^^^^^^^^^^^

A :dt:`for loop expression` is a :t:`loop expression` that continues to
evaluate its :t:`loop body` as long as its :t:`subject expression` yields a
:t:`value`.

See :s:`ForLoopExpression`.

Foreign Function Interface
^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Foreign Function Interface` employs :t:`ABI`, :t:`[attribute]s`,
:t:`external block`, :t:`[external function]s`, linkage, and :t:`type`
:t:`layout` to interface a Rust program with foreign code.

fragment specifier
^^^^^^^^^^^^^^^^^^

A :dt:`fragment specifier` is a :t:`construct` that indicates the :t:`type` of
a :t:`metavariable`.

See ``MacroFragmentSpecifier``.

full range expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`full range expression` is a :t:`range expression` that covers the full
range of a :t:`type`.

function
^^^^^^^^

A :dt:`function` is a :t:`value` of a :t:`function type` that models a behavior.

See :s:`FunctionDeclaration`.

function body
^^^^^^^^^^^^^

A :dt:`function body` is the :t:`block expression` of a :t:`function`.

See :s:`FunctionBody`.

function item type
^^^^^^^^^^^^^^^^^^

A :dt:`function item type` is a unique anonymous :t:`function type` that
identifies a :t:`function`.

function lifetime elision
^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Function lifetime elision` is a form of :t:`lifetime elision` that applies
to :t:`[function]s`, :t:`[function pointer type parameter]s` and :t:`[path]s`
resolving to one of the :std:`core::ops::Fn`, :std:`core::ops::FnMut`, and
:std:`core::ops::FnOnce` :t:`[trait]s`.

function parameter
^^^^^^^^^^^^^^^^^^

A :dt:`function parameter` is a :t:`construct` that yields a set of
:t:`[binding]s` that bind matched input :t:`[value]s` to :t:`[name]s` at the
site of a :t:`call expression` or a :t:`method call expression`.

See :s:`FunctionParameter`.

function pointer type
^^^^^^^^^^^^^^^^^^^^^

A :dt:`function pointer type` is an :t:`indirection type` that refers to a
:t:`function`.

See :s:`FunctionPointerTypeSpecification`.

function pointer type parameter
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`function pointer type parameter` is a :t:`function parameter` of a
:t:`function pointer type`.

See :s:`FunctionPointerTypeParameter`.

function qualifier
^^^^^^^^^^^^^^^^^^

A :dt:`function qualifier` is a :t:`construct` that determines the role of
a :t:`function`.

See :s:`FunctionQualifierList`.

function signature
^^^^^^^^^^^^^^^^^^

A :dt:`function signature` is a unique identification of a :t:`function`
that encompasses of its :t:`[function qualifier]s`, :t:`name`,
:t:`[generic parameter]s`, :t:`[function parameter]s`, :t:`return type`, and
:t:`where clause`.

function type
^^^^^^^^^^^^^

A :dt:`function type` is either a :t:`closure type` or a
:t:`function item type`.

function-like macro
^^^^^^^^^^^^^^^^^^^

A :dt:`function-like macro` is a :t:`procedural macro` that consumes a stream
of :t:`[token]s` and produces a stream of tokens, and is invoked directly.

fundamental
^^^^^^^^^^^

A :t:`trait` or :t:`type` is :dt:`fundamental` when its
:t:`implementation coherence` rules are relaxed and the :t:`trait` or :t:`type`
is always treated as if it was a :t:`local trait` or a :t:`local type`.

future
^^^^^^

A :dt:`future` represents a :t:`value` of a :t:`type` that implements the
:std:`core::future::Future` :t:`trait` which may not have finished computing
yet.

future operand
^^^^^^^^^^^^^^

A :dt:`future operand` is an :t:`operand` whose :t:`future` is being awaited by
an :t:`await expression`.

See :s:`FutureOperand`.

generic argument
^^^^^^^^^^^^^^^^

A :dt:`generic argument` supplies a static input for an
:t:`associated trait type` or a :t:`generic parameter`.

See :s:`GenericArgumentList`.

generic associated type
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`generic associated type` is an :t:`associated type` with
:t:`[generic parameter]s`.

generic conformance
^^^^^^^^^^^^^^^^^^^

:dt:`Generic conformance` measures the compatibility between a set of
:t:`[generic parameter]s` and a set of :t:`[generic argument]s`.

generic enum
^^^^^^^^^^^^

A :dt:`generic enum` is an :t:`enum` with :t:`[generic parameter]s`.

generic function
^^^^^^^^^^^^^^^^

A :dt:`generic function` is a :t:`function` with :t:`[generic parameter]s`.

generic implementation
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`generic implementation` is an :t:`implementation` with
:t:`[generic parameter]s`.

generic parameter
^^^^^^^^^^^^^^^^^

A :dt:`generic parameter` is a placeholder for a :t:`constant`, a
:t:`lifetime`, or a :t:`type` whose :t:`value` is supplied statically by a
:t:`generic argument`.

See :s:`GenericParameterList`.

generic parameter scope
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`generic parameter scope` is a :t:`scope` for :t:`[generic parameter]s`.

generic struct
^^^^^^^^^^^^^^

A :dt:`generic struct` is a :t:`struct` with :t:`[generic parameter]s`.

generic substitution
^^^^^^^^^^^^^^^^^^^^

A :dt:`generic substitution` is the replacement of a :t:`generic parameter`
with a :t:`generic argument`.

generic trait
^^^^^^^^^^^^^

A :dt:`generic trait` is a :t:`trait` with :t:`[generic parameter]s`.

generic type
^^^^^^^^^^^^

A :dt:`generic type` is a :t:`type` with a :t:`generic parameter`.

generic type alias
^^^^^^^^^^^^^^^^^^

A :dt:`generic type alias` is a :t:`type alias` with :t:`[generic parameter]s`.

generic union
^^^^^^^^^^^^^

A :dt:`generic union` is a :t:`union` with :t:`[generic parameter]s`.

glob import
^^^^^^^^^^^

A :dt:`glob import` is a :t:`use import` that brings all :t:`[name]s` with
:t:`public visibility` prefixed by its :t:`path` prefix into :t:`scope`.

See :s:`GlobImport`.

global path
^^^^^^^^^^^

A :dt:`global path` is a :t:`path` that starts with :t:`namespace qualifier`
``::``.

global type variable
^^^^^^^^^^^^^^^^^^^^

A :dt:`global type variable` is a :t:`type variable` that can refer to any
:t:`type`.

greater-than expression
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`greater-than expression` is a :t:`comparison expression` that tests for
a greater-than relationship.

See :s:`GreaterThanExpression`.

greater-than-or-equals expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`greater-than-or-equals expression` is a :t:`comparison expression` that
tests for a greater-than-or-equals relationship.

See :s:`GreaterThanOrEqualsExpression`.

half-open range pattern
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`half-open range pattern` is a :t:`range pattern` with only a
:t:`range pattern low bound`.

See :s:`HalfOpenRangePattern`.

hexadecimal literal
^^^^^^^^^^^^^^^^^^^

A :dt:`hexadecimal literal` is an :t:`integer literal` in base 16.

See :s:`HexadecimalLiteral`.

higher-ranked trait bound
^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`higher-ranked trait bound` is a :t:`bound` that specifies an infinite
list of :t:`[bound]s` for all possible :t:`[lifetime]s`.

See :s:`ForGenericParameterList`.

hygiene
^^^^^^^

:dt:`Hygiene` is a property of :t:`[macro]s` and :t:`[identifier]s`` that
appear within them, which aims to eliminate the syntactic interference between
a :t:`macro` and its environment.

hygienic
^^^^^^^^

An :t:`identifier` is :dt:`hygienic` when it has :t:`definition site hygiene`.

i8
^^

:dc:`i8` is a :t:`signed integer type` whose :t:`[value]s` range from - (2\
:sup:`7`) to 2\ :sup:`7` - 1, all inclusive.

i16
^^^

:dc:`i16` is a :t:`signed integer type` whose :t:`[value]s` range from - (2\
:sup:`15`) to 2\ :sup:`15` - 1, all inclusive.

i32
^^^

:dc:`i32` is a :t:`signed integer type` whose :t:`[value]s` range from - (2\
:sup:`31`) to 2\ :sup:`31` - 1, all inclusive.

i64
^^^

:dc:`i64` is a :t:`signed integer type` whose :t:`[value]s` range from - (2\
:sup:`63`) to 2\ :sup:`63` - 1, all inclusive.

i128
^^^^

:dc:`i128` is a :t:`signed integer type` whose :t:`[value]s` range from - (2\
:sup:`127`) to 2\ :sup:`127` - 1, all inclusive.

identifier
^^^^^^^^^^

An :dt:`identifier` is a :t:`lexical element` that refers to a :t:`name`.

See :s:`Identifier`.

identifier pattern
^^^^^^^^^^^^^^^^^^

An :dt:`identifier pattern` is a :t:`pattern` that binds the :t:`value` it
matches to a :t:`binding`.

See :s:`IdentifierPattern`.

if expression
^^^^^^^^^^^^^

An :dt:`if expression` is an :t:`expression` that evaluates either a
:t:`block expression` or an :t:`else expression` depending on the :t:`value`
of its :t:`subject expression`.

See :s:`IfExpression`.

if let expression
^^^^^^^^^^^^^^^^^

An :dt:`if let expression` is an :t:`expression` that evaluates either a
:t:`block expression` or an :t:`else expression` depending on whether its
:t:`pattern` can be matched against its :t:`subject let expression`.

See :s:`IfLetExpression`.

immutable
^^^^^^^^^

A :t:`value` is :dt:`immutable` when it cannot be modified.

immutable borrow
^^^^^^^^^^^^^^^^

An :dt:`immutable borrow` is an :t:`immutable reference` produced by
:t:`borrowing`.

immutable borrow expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`immutable borrow expression` is a :t:`borrow expression` that lacks
:t:`keyword` ``mut``.

immutable place expression
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`immutable place expression` is a :t:`place expression` whose memory
location cannot be modified.


immutable place expression context
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`immutable place expression context` is a :t:`place expression context`
whose memory location cannot be modified.

immutable raw pointer type
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`immutable raw pointer type` is a :t:`raw pointer type` subject to
:t:`keyword` ``const``.

immutable reference
^^^^^^^^^^^^^^^^^^^

An :dt:`immutable reference` is a :t:`value` of a :t:`shared reference type`,
and prevents the mutation of its :t:`referent`.

immutable static
^^^^^^^^^^^^^^^^

An :dt:`immutable static` is a :t:`static` whose :t:`value` cannot be modified.

immutable variable
^^^^^^^^^^^^^^^^^^

An :dt:`immutable variable` is a :t:`variable` whose :t:`value` cannot be
modified.

impl header lifetime elision
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Impl header lifetime elision` is a form of :t:`lifetime elision` that
applies to the :t:`implementing type` and :t:`implemented trait` (if any) of an
:t:`implementation`.

impl trait type
^^^^^^^^^^^^^^^

An :dt:`impl trait type` is a :t:`type` that implements a :t:`trait`, where the
:t:`type` is known at compile time.

See :s:`ImplTraitTypeSpecification`, :s:`ImplTraitTypeSpecificationOneBound`.

implementation
^^^^^^^^^^^^^^

An :dt:`implementation` is an :t:`item` that supplements an
:t:`implementing type` by extending its functionality.

See :s:`Implementation`.

implementation body
^^^^^^^^^^^^^^^^^^^

An :dt:`implementation body` is a :t:`construct` that encapsulates the
:t:`[associated item]s`, :t:`[inner attribute]s`, and
:t:`[inner doc comment]s` of an :t:`implementation`.

See :s:`ImplementationBody`.

implementation coherence
^^^^^^^^^^^^^^^^^^^^^^^^

A :t:`trait implementation` exhibits :dt:`implementation coherence` when it is
valid and does not overlap with another :t:`trait implementation`.

implementation conformance
^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Implementation conformance` measures the compatibility between a
:t:`trait implementation` and the :t:`implemented trait`.

implemented trait
^^^^^^^^^^^^^^^^^

An :dt:`implemented trait` is a :t:`trait` whose functionality has been
implemented by an :t:`implementing type`.

See :s:`ImplementedTrait`.

implementing type
^^^^^^^^^^^^^^^^^

An :dt:`implementing type` is the :t:`type` that the :t:`[associated item]s` of
an :t:`implementation` are associated with.

See :s:`ImplementingType`.

implicit borrow
^^^^^^^^^^^^^^^

An :dt:`implicit borrow` is a :t:`borrow` that is not present syntactically in
program text.

implicitly declared entity
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`implicitly declared entity` is an :t:`entity` that lacks an explicit
:t:`declaration`.

implied bound
^^^^^^^^^^^^^

An :dt:`implied bound` is a :t:`bound` that is not expressed in syntax, but is the byproduct of relations between :t:`[lifetime parameter]s` and :t:`[function parameter]s`, between :t:`[lifetime parameter]s` and a :t:`return type`, and between :t:`[lifetime parameter]s` and :t:`[field]s`.

in scope
^^^^^^^^

A :t:`name` is :dt:`in scope` when it can be referred to.

inclusive range pattern
^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`inclusive range pattern` is a :t:`range pattern` with both a
:t:`range pattern low bound` and a :t:`range pattern high bound`.

See :s:`InclusiveRangePattern`.

incomplete associated constant
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`incomplete associated constant` is an :t:`associated constant` without
a :t:`constant initializer`.

incomplete associated function
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`incomplete associated function` is an :t:`associated function` without
a :t:`function body`.

incomplete associated type
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`incomplete associated type` is an :t:`associated type` without an
:t:`initialization type`.

index expression
^^^^^^^^^^^^^^^^

An :dt:`index expression` is an :t:`expression` that indexes into a :t:`value`
of a :t:`type`.

See :s:`IndexExpression`.

indexable type
^^^^^^^^^^^^^^

A :dt:`indexable type` is a :t:`type` that implements the
:std:`core::ops::Index` :t:`trait`.

indexed deconstructor
^^^^^^^^^^^^^^^^^^^^^

An :dt:`indexed deconstructor` is a :t:`construct` that matches the position of
a :t:`tuple field`.

See :s:`IndexedDeconstructor`.

indexed field selector
^^^^^^^^^^^^^^^^^^^^^^

An :dt:`indexed field selector` is a :t:`field selector` where the selected
:t:`field` is indicated by an index.

See :s:`IndexedFieldSelector`.

indexed initializer
^^^^^^^^^^^^^^^^^^^

An :dt:`indexed initializer` is a :t:`construct` that specifies the index and
initial :t:`value` of a :t:`field` in a :t:`struct expression`.

See :s:`IndexedInitializer`.

indexed operand
^^^^^^^^^^^^^^^

An :dt:`indexed operand` is an :t:`operand` which indicates the :t:`value` of a
:t:`type` implementing :std:`core::ops::Index` being indexed into by an
:t:`index expression`.

See :s:`IndexedOperand`.

indexing operand
^^^^^^^^^^^^^^^^

An :dt:`indexing operand` is an :t:`operand` which specifies the index for the
:t:`indexed operand` being indexed into by an :t:`index expression`.

See :s:`IndexingOperand`.

indirection type
^^^^^^^^^^^^^^^^

An :dt:`indirection type` is a :t:`type` whose :t:`[value]s` refer to memory
locations.

inert attribute
^^^^^^^^^^^^^^^

An :dt:`inert attribute` is an :t:`attribute` that remains with the :t:`item`
it decorates.

inferred type
^^^^^^^^^^^^^

An :dt:`inferred type` is a placeholder for a :t:`type` deduced by
:t:`type inference`.

See :s:`InferredType`.

infinite loop
^^^^^^^^^^^^^

For :dt:`infinite loop`, see :t:`infinite loop expression`.

infinite loop expression
^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`infinite loop expression` is a :t:`loop expression` that continues to
evaluate its :t:`loop body` indefinitely.

See :s:`InfiniteLoopExpression`.

inherent implementation
^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`inherent implementation` is an :t:`implementation` that adds direct
functionality.

See :s:`InherentImplementation`.

initialization
^^^^^^^^^^^^^^

:dt:`Initialization` is the act of supplying an initial :t:`value` to a
:t:`constant`, a :t:`static`, or a :t:`variable`.

initialization expression
^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`initialization expression` is either a :t:`constant initializer` or a
:t:`static initializer`.

initialization type
^^^^^^^^^^^^^^^^^^^

An :dt:`initialization type` is the :t:`type` a :t:`type alias` defines a
:t:`name` for.

See :s:`InitializationType`.

inline assembly
^^^^^^^^^^^^^^^

:dt:`Inline assembly` is hand-written assembly code that is integrated into a
Rust program.

inline assembly argument
^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`inline assembly argument` is either an :t:`assembly instruction`, a :t:`register argument`, an :t:`ABI clobber`, or an :t:`assembly option`.

inline module
^^^^^^^^^^^^^

An :dt:`inline module` is a :t:`module` with an :s:`InlineModuleSpecification`.

See :s:`InlineModuleSpecification`.

inner attribute
^^^^^^^^^^^^^^^

An :dt:`inner attribute` is an :t:`attribute` that applies to an enclosing
:t:`item`.

See :s:`InnerAttribute`.

inner block doc
^^^^^^^^^^^^^^^

An :dt:`inner block doc` is a :t:`block comment` that applies to an enclosing
:t:`non-[comment]` :t:`construct`.

See :s:`InnerBlockDoc`.

inner doc comment
^^^^^^^^^^^^^^^^^

An :dt:`inner doc comment` is either an :t:`inner block doc` or an
:t:`inner line doc`.

inner line doc
^^^^^^^^^^^^^^

An :dt:`inner line doc` is a :t:`line comment` that applies to an enclosing
:t:`non-[comment]` :t:`construct`.

See :s:`InnerLineDoc`.

input register
^^^^^^^^^^^^^^

An :dt:`input register` is a :t:`register` whose :t:`register name` is used in
a :t:`register argument` subject to :t:`direction modifier` ``in``, ``inout``,
or ``inlateout``.

input register expression
^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`input register expression` is an :t:`expression` that provides the
initial :t:`value` of a :t:`register`.

See :s:`InputRegisterExpression`.

input-output register expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`input-output register expression` is a :t:`construct` that specifies
both an :t:`input register expression` and an :t:`output register expression`.

See :s:`InputOutputRegisterExpression`.

integer literal
^^^^^^^^^^^^^^^

An :dt:`integer literal` is a :t:`numeric literal` that denotes a whole number.

See :s:`IntegerLiteral`.

integer suffix
^^^^^^^^^^^^^^

An :dt:`integer suffix` is a component of an :t:`integer literal` that
specifies an explicit :t:`integer type`.

See ``IntegerSuffix.``

integer type
^^^^^^^^^^^^

An :dt:`integer type` is a :t:`numeric type` whose :t:`[value]s` denote whole
numbers.

integer type variable
^^^^^^^^^^^^^^^^^^^^^

An :dt:`integer type variable` is a :t:`type variable` that can refer only to
:t:`[integer type]s`.

interior mutability
^^^^^^^^^^^^^^^^^^^

:dt:`Interior mutability` is a property of :t:`[type]s` whose :t:`[value]s` can
be modified through :t:`[immutable reference]s`.

intermediate match arm
^^^^^^^^^^^^^^^^^^^^^^

An :dt:`intermediate match arm` is any :t:`non-[final match arm]` of a
:t:`match expression`.

See :s:`IntermediateMatchArm`.

irrefutable constant
^^^^^^^^^^^^^^^^^^^^

An :dt:`irrefutable constant` is a :t:`constant` of a :t:`type` that has at most
one :t:`value`.


irrefutable pattern
^^^^^^^^^^^^^^^^^^^

An :dt:`irrefutable pattern` is a :t:`pattern` that always matches the
:t:`value` it is being matched against.

isize
^^^^^

:dc:`isize` is a :t:`signed integer type` with the same number of bits as the
platform's :t:`pointer type`, and is at least 16-bits wide.

item
^^^^

An :dt:`item` is the most basic semantic element in program text. An item
defines the compile- and run-time semantics of a program.

See :s:`Item`.

item scope
^^^^^^^^^^

An :dt:`item scope` is a :t:`scope` for :t:`[item]s`.

item statement
^^^^^^^^^^^^^^

An :dt:`item statement` is a :t:`statement` that is expressed as an :t:`item`.

iteration expression
^^^^^^^^^^^^^^^^^^^^

An :dt:`iteration expression` is an :t:`expression` that provides the criterion
of a :t:`while loop expression`.

See :s:`IterationExpression`.

keyword
^^^^^^^

A :dt:`keyword` is a word in program text that has special meaning.

See :s:`Keyword`.

label
^^^^^

A :dt:`label` is the :t:`name` of a :t:`loop expression`.

See :s:`Label`.

label indication
^^^^^^^^^^^^^^^^

A :dt:`label indication` is a :t:`construct` that indicates a :t:`label`.

See :s:`LabelIndication`.

label scope
^^^^^^^^^^^

A :dt:`label scope` is a :t:`scope` for :t:`[label]s`.

layout
^^^^^^

:dt:`Layout` specifies the :t:`alignment`, :t:`size`, and the relative offset
of :t:`[field]s` in a :t:`type`.

lazy and expression
^^^^^^^^^^^^^^^^^^^

A :dt:`lazy and expression` is a :t:`lazy boolean expression` that uses short
circuit and arithmetic.

See :s:`LazyAndExpression`.

lazy boolean expression
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`lazy boolean expression` is an :t:`expression` that performs short
circuit Boolean arithmetic.

See :s:`LazyBooleanExpression`.

lazy or expression
^^^^^^^^^^^^^^^^^^

A :dt:`lazy or expression` is a :t:`lazy boolean expression` that uses short
circuit or arithmetic.

See :s:`LazyOrExpression`.

left operand
^^^^^^^^^^^^

A :dt:`left operand` is an :t:`operand` that appears on the left-hand side of a
:t:`binary operator`.

See :s:`LeftOperand`.

less-than expression
^^^^^^^^^^^^^^^^^^^^

A :dt:`less-than expression` is a :t:`comparison expression` that tests for a
less-than relationship.

See :s:`LessThanExpression`.

less-than-or-equals expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`less-than-or-equals expression` is a :t:`comparison expression` that
tests for a less-than-or-equals relationship.

See :s:`LessThanOrEqualsExpression`.


let binding
^^^^^^^^^^^

A :dt:`let binding` is the :t:`binding` introduced by a :t:`let statement`, an :t:`if let expression`, or a :t:`while let loop expression`.

let initializer
^^^^^^^^^^^^^^^

A :dt:`let initializer` is a :t:`construct` that provides the :t:`value` of
the :t:`[binding]s` of the :t:`let statement` using an :t:`expression`, or
alternatively executes a :t:`block expression`.

See :s:`LetInitializer`.

let statement
^^^^^^^^^^^^^

A :dt:`let statement` is a :t:`statement` that introduces new :t:`[variable]s`
given by the :t:`[binding]s` produced by its :t:`pattern-without-alternation`
that are optionally initialized to a :t:`value`.

See :s:`LetStatement`.

lexical element
^^^^^^^^^^^^^^^

A :dt:`lexical element` is the most basic syntactic element in program
text.

library crate
^^^^^^^^^^^^^

A :dt:`library crate` is a :t:`crate` whose :t:`crate type` is ``lib``, ``rlib``,
``staticlib``, ``dylib``, or ``cdylib``.

lifetime
^^^^^^^^

A :dt:`lifetime` specifies the expected longevity of a :t:`reference`.

See :s:`Lifetime`.

lifetime argument
^^^^^^^^^^^^^^^^^

A :dt:`lifetime argument` is a :t:`generic argument` that supplies the
:t:`value` of a :t:`lifetime parameter`.

See :s:`LifetimeArgument`.

lifetime bound
^^^^^^^^^^^^^^

A :dt:`lifetime bound` is a :t:`bound` that imposes a constraint on the
:t:`[lifetime]s` of :t:`[generic parameter]s`.

See :s:`LifetimeIndication`.

lifetime bound predicate
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`lifetime bound predicate` is a :t:`construct` that specifies
:t:`[lifetime bound]s` on a :t:`lifetime parameter`.

See :s:`LifetimeBoundPredicate`.

lifetime elision
^^^^^^^^^^^^^^^^

:dt:`Lifetime elision` is a set of rules that automatically insert
:t:`[lifetime parameter]s` and/or :t:`[lifetime argument]s` when they are
elided in the source code.

lifetime parameter
^^^^^^^^^^^^^^^^^^

A :dt:`lifetime parameter` is a :t:`generic parameter` for a :t:`lifetime`.

See :s:`LifetimeParameter`.

lifetime variable
^^^^^^^^^^^^^^^^^

A :dt:`lifetime variable` is a placeholder used during :t:`type inference` to
stand in for an undetermined :t:`lifetime` of a :t:`type`.


line
^^^^

A :dt:`line` is a sequence of zero or more characters followed by an end of
line.

line comment
^^^^^^^^^^^^

A :dt:`line comment` is a :t:`comment` that spans exactly one :t:`line`.

See :s:`LineComment`.

literal
^^^^^^^

A :dt:`literal` is a fixed :t:`value` in program text.

See :s:`Literal`.

literal expression
^^^^^^^^^^^^^^^^^^

A :dt:`literal expression` is an :t:`expression` that denotes a :t:`literal`.

See :s:`LiteralExpression`.

literal pattern
^^^^^^^^^^^^^^^

A :dt:`literal pattern` is a :t:`pattern` that matches a :t:`literal`.

See :s:`LiteralPattern`.

local trait
^^^^^^^^^^^

A :dt:`local trait` is a :t:`trait` that is defined in the current :t:`crate`.

local type
^^^^^^^^^^

A :dt:`local type` is a :t:`type` that is defined in the current :t:`crate`.

local variable
^^^^^^^^^^^^^^

For :dt:`local variable`, see :t:`variable`.

loop
^^^^

For :dt:`loop`, see :t:`loop expression`.

loop body
^^^^^^^^^

A :dt:`loop body` is the :t:`block expression` of a :t:`loop expression`.

See :s:`LoopBody`.

loop expression
^^^^^^^^^^^^^^^

A :dt:`loop expression` is an :t:`expression` that evaluates a
:t:`block expression` continuously as long as some criterion holds true.

See :s:`LoopExpression`.

macro
^^^^^

A :dt:`macro` is a custom definition that extends Rust by defining callable
syntactic transformations.

macro expansion
^^^^^^^^^^^^^^^

:dt:`Macro expansion` is the process of statically executing a
:t:`macro invocation` and replacing it with the produced output of the
:t:`macro invocation`.

macro implementation function
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`macro implementation function` is the :t:`function` that encapsulates
the syntactic transformations of a :t:`procedural macro`.

macro invocation
^^^^^^^^^^^^^^^^

A :dt:`macro invocation` is a call of a :t:`declarative macro` or
:t:`function-like macro` that is expanded statically and replaced with the
result of the :t:`macro`.

See :s:`MacroInvocation`.

macro match
^^^^^^^^^^^

A :dt:`macro match` is the most basic form of a satisfied :t:`macro matcher`.

See :s:`MacroMatch`.

macro matcher
^^^^^^^^^^^^^

A :dt:`macro matcher` is a :t:`construct` that describes a syntactic pattern
that a :t:`macro` must match.

See :s:`MacroMatcher`.

macro matching
^^^^^^^^^^^^^^

:dt:`Macro matching` is the process of performing :t:`rule matching` and
:t:`token matching`.

macro repetition
^^^^^^^^^^^^^^^^

A :dt:`macro repetition` is either a :t:`macro repetition in matching` or a
:t:`macro repetition in transcription`.

macro repetition in matching
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`macro repetition in matching` allows for a syntactic pattern to be
matched zero or multiple times during :t:`macro matching`.

See :s:`MacroRepetitionMatch`.

macro repetition in transcription
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`macro repetition in transcription` allows for a syntactic pattern to be
transcribed zero or multiple times during :t:`macro transcription`.

See :s:`MacroRepetitionTranscriber`.

macro rule
^^^^^^^^^^

A :dt:`macro rule` is a :t:`construct` that consists of a :t:`macro matcher`
and a :t:`macro transcriber`.

See :s:`MacroRule`.

macro statement
^^^^^^^^^^^^^^^

A :dt:`macro statement` is a :t:`statement` expressed as a
:t:`terminated macro invocation`.

macro transcriber
^^^^^^^^^^^^^^^^^

A :dt:`macro transcriber` is a :t:`construct` that describes the replacement
syntax of a :t:`macro`.

See :s:`MacroTranscriber`.

macro transcription
^^^^^^^^^^^^^^^^^^^

:dt:`Macro transcription` is the process of producing the expansion of a
:t:`declarative macro`.

main function signature
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`main function signature` is a :t:`function signature` subject to specific
restrictions.

match arm
^^^^^^^^^

A :dt:`match arm` is a :t:`construct` that consists of a :t:`match arm matcher`
and a :t:`match arm body`.

match arm body
^^^^^^^^^^^^^^

A :dt:`match arm body` is the :t:`operand` of a :t:`match arm`.

match arm guard
^^^^^^^^^^^^^^^

A :dt:`match arm guard` is a :t:`construct` that provides additional filtering
to a :t:`match arm matcher`.

See :s:`MatchArmGuard`.

match arm matcher
^^^^^^^^^^^^^^^^^

A :dt:`match arm matcher` is a :t:`construct` that consists of a :t:`pattern`
and a :t:`match arm guard`.

See :s:`MatchArmMatcher`.

match expression
^^^^^^^^^^^^^^^^

A :dt:`match expression` is an :t:`expression` that tries to match one of
its multiple :t:`[pattern]s` against its :t:`subject expression` and if it
succeeds, evaluates an :t:`operand`.

See :s:`MatchExpression`.

metavariable
^^^^^^^^^^^^

A :dt:`metavariable` is a :t:`macro match` that describes a :t:`variable`.

See :s:`MacroMetavariable`.

metavariable indication
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`metavariable indication` is a :t:`construct` that indicates a
:t:`metavariable`.

See :s:`MacroMetavariableIndication`.

method
^^^^^^

A :dt:`method` is an :t:`associated function` with a :t:`self parameter`.

method call expression
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`method call expression` is an :t:`expression` that invokes a :t:`method`
of a :t:`variable`.

See :s:`MethodCallExpression`.

method operand
^^^^^^^^^^^^^^

A :dt:`method operand` is an :t:`operand` that denotes the :t:`method` being
invoked by a :t:`method call expression`.

See :s:`MethodOperand`.

method resolution
^^^^^^^^^^^^^^^^^

:dt:`Method resolution` is a kind of :t:`resolution` that applies to a
:t:`method call expression`.

mixed site hygiene
^^^^^^^^^^^^^^^^^^

:dt:`Mixed site hygiene` is a type of :t:`hygiene` which resolves to the
:s:`MacroRulesDeclaration` site for :t:`[variable]s`, :t:`[label]s`, and the
``$crate`` :t:`metavariable`, and to the :s:`MacroInvocation` site otherwise,
and is considered :t:`partially hygienic`.

modifying operand
^^^^^^^^^^^^^^^^^

A :dt:`modifying operand` is an :t:`operand` that supplies the :t:`value` that
is used in the calculation of a :t:`compound assignment expression`.

See :s:`ModifyingOperand`.

module
^^^^^^

A :dt:`module` is a container for zero or more :t:`[item]s`.

See :s:`ModuleDeclaration`.

move type
^^^^^^^^^

A :dt:`move type` is a :t:`type` that implements the :std:`core::marker::Sized`
:t:`trait` and that is not a :t:`copy type`.

multi segment path
^^^^^^^^^^^^^^^^^^

A :dt:`multi segment path` is a :t:`path` consisting of more than one
:t:`path segment`.

multiplication assignment
^^^^^^^^^^^^^^^^^^^^^^^^^

For :dt:`multiplication assignment`, see
:t:`multiplication assignment expression`.

multiplication assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`multiplication assignment expression` is a
:t:`compound assignment expression` that uses multiplication.

See :s:`MultiplicationAssignmentExpression`.

multiplication expression
^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`multiplication expression` is an :t:`arithmetic expression` that uses
multiplication.

See :s:`MultiplicationExpression`.

mutability
^^^^^^^^^^

:dt:`Mutability` determines whether a :t:`construct` can modify a :t:`value`.

mutable
^^^^^^^

A :t:`value` is :dt:`mutable` when it can be modified.

mutable assignee expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`mutable assignee expression` is an :t:`assignee expression` whose
:t:`value` can be modified.

mutable binding
^^^^^^^^^^^^^^^

A :dt:`mutable binding` is a :t:`binding` whose :t:`value` can be modified.

mutable borrow
^^^^^^^^^^^^^^

A :dt:`mutable borrow` is a :t:`mutable reference` produced by :t:`borrowing`.

mutable borrow expression
^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`mutable borrow expression` is a :t:`borrow expression` that has
:t:`keyword` ``mut``.

mutable place expression
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`mutable place expression` is a :t:`place expression` whose memory
location can be modified.

mutable place expression context
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`mutable place expression context` is a :t:`place expression context`
that may evaluate its :t:`operand` as a mutable memory location.

mutable raw pointer type
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`mutable raw pointer type` is a :t:`raw pointer type` subject to
:t:`keyword` ``mut``.

mutable reference
^^^^^^^^^^^^^^^^^

A :dt:`mutable reference` is a :t:`value` of a :t:`mutable reference type`, and
allows the mutation of its :t:`referent`.

mutable reference type
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`mutable reference type` is a :t:`reference type` subject to :t:`keyword`
``mut``.

mutable static
^^^^^^^^^^^^^^

A :dt:`mutable static` is a :t:`static` whose :t:`value` can be modified.

mutable variable
^^^^^^^^^^^^^^^^

A :dt:`mutable variable` is a :t:`variable` whose :t:`value` can be modified.

name
^^^^

A :dt:`name` is an :t:`identifier` that refers to an :t:`entity`.

See :s:`Name`.

named block expression
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`named block expression` is a :t:`block expression` with a :t:`label`.

named deconstructor
^^^^^^^^^^^^^^^^^^^

A :dt:`named deconstructor` is a :t:`construct` that matches the :t:`name` of
a :t:`field`.

See :s:`NamedDeconstructor`.

named field selector
^^^^^^^^^^^^^^^^^^^^

A :dt:`named field selector` is a :t:`field selector` where the selected
:t:`field` is indicated by an :t:`identifier`.

See :s:`NamedFieldSelector`.

named initializer
^^^^^^^^^^^^^^^^^

A :dt:`named initializer` is a :t:`construct` that specifies the name and
initial :t:`value` of a :t:`field` in a :t:`struct expression`.

See :s:`NamedInitializer`.

named loop expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`named loop expression` is a :t:`loop expression` with a :t:`label`.

named register argument
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`named register argument` is a :t:`register argument` whose configuration
is bound to an :t:`identifier`.

namespace
^^^^^^^^^

A :dt:`namespace` is a logical grouping of :t:`[name]s` such that the
occurrence of a :t:`name` in one :t:`namespace` does not conflict with an
occurrence of the same :t:`name` in another :t:`namespace`.

NaN-boxing
^^^^^^^^^^

:dt:`NaN-boxing` is a technique for encoding :t:`[value]s` using the low order
bits of the mantissa of a 64-bit IEEE floating-point ``NaN``.

negation expression
^^^^^^^^^^^^^^^^^^^

A :dt:`negation expression` is an :t:`expression` that negates its :t:`operand`.

See :s:`NegationExpression`.

nesting import
^^^^^^^^^^^^^^

A :dt:`nesting import` is a :t:`use import` that provides a common :t:`path`
prefix for its nested :t:`[use import]s`.

See :s:`NestingImport`.

never type
^^^^^^^^^^

The :dt:`never type` is a :t:`type` that represents the result of a computation
that never completes.

See :s:`NeverType`.

non-reference pattern
^^^^^^^^^^^^^^^^^^^^^

A :dt:`non-reference pattern` is any :t:`pattern` except
:t:`non-[binding pattern]s`, :t:`[path pattern]s`, :t:`[reference pattern]s`,
and :t:`[underscore pattern]s`.

not configuration predicate
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`not configuration predicate` is a :t:`configuration predicate` that
negates the Boolean :t:`value` of its nested :t:`configuration predicate`.

See :s:`ConfigurationPredicateNot`.

not-equals expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`not-equals expression` is a :t:`comparison expression` that tests for
inequality.

See :s:`NotEqualsExpression`.

null
^^^^

A :dc:`null` :t:`value` denotes the address ``0``.

numeric literal
^^^^^^^^^^^^^^^

A :dt:`numeric literal` is a :t:`literal` that denotes a number.

See :s:`NumericLiteral`.

numeric literal pattern
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`numeric literal pattern` is a :t:`pattern` that matches a :t:`numeric
literal`.

See :s:`LiteralPattern`.

numeric type
^^^^^^^^^^^^

A :dt:`numeric type` is a :t:`type` whose :t:`[value]s` denote numbers.

object safe
^^^^^^^^^^^

A :t:`trait` is :dt:`object safe` when it can be used as a
:t:`trait object type`.

object safety
^^^^^^^^^^^^^

:dt:`Object safety` is the process of determining whether a :t:`trait` can be
used as a :t:`trait object type`.

obsolete range pattern
^^^^^^^^^^^^^^^^^^^^^^

An :dt:`obsolete range pattern` is a :t:`range pattern` that uses obsolete
syntax to express an :t:`inclusive range pattern`.

See :s:`ObsoleteRangePattern`.

octal literal
^^^^^^^^^^^^^

An :dt:`octal literal` is an :t:`integer literal` in base 8.

See ``OctalLiteral.``

operand
^^^^^^^

An :dt:`operand` is an :t:`expression` nested within an expression.

See :s:`Operand`.

operator expression
^^^^^^^^^^^^^^^^^^^

An :dt:`operator expression` is an :t:`expression` that involves an operator.

See :s:`OperatorExpression`.

opt-out trait bound
^^^^^^^^^^^^^^^^^^^

An :dt:`opt-out trait bound` is a :t:`trait bound` with :s:`Punctuation` ``?``
that nullifies an implicitly added :t:`trait bound`.


or-pattern
^^^^^^^^^^

An :dt:`or-pattern` is a :t:`pattern` that matches on one of two or more :t:`[pattern-without-alternation]s` and or-s them using character 0x7C (vertical line, i.e. ``|``).

See :s:`Pattern`.

outer attribute
^^^^^^^^^^^^^^^

An :dt:`outer attribute` is an :t:`attribute` that applies to a subsequent
:t:`item`.

See :s:`OuterAttribute`.

outer block doc
^^^^^^^^^^^^^^^

An :dt:`outer block doc` is a :t:`block comment` that applies to a subsequent
:t:`non-[comment]` :t:`construct`.

See :s:`OuterBlockDoc`.

outer doc comment
^^^^^^^^^^^^^^^^^

An :dt:`outer doc comment` is either an :t:`outer block doc` or an
:t:`outer line doc`.

outer line doc
^^^^^^^^^^^^^^

An :dt:`outer line doc` is a :t:`line comment` that applies to a subsequent
:t:`non-[comment]` :t:`construct`.

See :s:`OuterLineDoc`.

outline module
^^^^^^^^^^^^^^

An :dt:`outline module` is a :t:`module` with an
:s:`OutlineModuleSpecification`.

See :s:`OutlineModuleSpecification`.

outlives bound
^^^^^^^^^^^^^^

An :dt:`outlives bound` is a :t:`trait bound` which requires that a
:t:`generic parameter` outlives a :t:`lifetime parameter`.

output register
^^^^^^^^^^^^^^^

An :dt:`output register` is a :t:`register` whose :t:`register name` is
used in a :t:`register argument` subject to :t:`direction modifier` ``out``,
``lateout``, ``inout``, or ``inlateout``.

output register expression
^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`output register expression` is an :t:`expression` that is assigned the
:t:`value` of a :t:`register`.

See :s:`OutputRegisterExpression`.

overlap
^^^^^^^

Two :t:`[value]s` :dt:`overlap` when their memory locations overlap, or both
values are elements of the same :t:`array`.

owner
^^^^^

An :dt:`owner` is a :t:`variable` that holds a :t:`value`.

ownership
^^^^^^^^^

:dt:`Ownership` is a property of :t:`[value]s` that is central to the resource
management model of Rust.

panic
^^^^^

A :dt:`panic` is an abnormal program state caused by invoking :t:`macro`
:std:`core::panic`.

parenthesized expression
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`parenthesized expression` is an :t:`expression` that groups other
expressions.

See :s:`ParenthesizedExpression`.

parenthesized pattern
^^^^^^^^^^^^^^^^^^^^^

A :dt:`parenthesized pattern` is a :t:`pattern` that controls the precedence of
its :t:`[subpattern]s`.

See :s:`ParenthesizedPattern`.

parenthesized type
^^^^^^^^^^^^^^^^^^

A :dt:`parenthesized type` is a :t:`type` that disambiguates the interpretation
of :t:`[lexical element]s`.

See :s:`ParenthesizedTypeSpecification`.

partially hygienic
^^^^^^^^^^^^^^^^^^

An :t:`identifier` is :dt:`partially hygienic` when it has
:t:`mixed site hygiene`.

passing convention
^^^^^^^^^^^^^^^^^^

A :dt:`passing convention` is the mechanism that defines how a :t:`value` is
transferred between :t:`[place]s`.

path
^^^^

A :dt:`path` is a sequence of :t:`[path segment]s` logically separated by
:dt:`namespace qualifier` ``::`` that resolves to an :t:`entity`.

path expression
^^^^^^^^^^^^^^^

A :dt:`path expression` is a :t:`path` that acts as an :t:`expression`.

See :s:`PathExpression`.

path expression resolution
^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Path expression resolution` is a form of :t:`path resolution` that applies
to a :t:`path expression`.

path pattern
^^^^^^^^^^^^

A :dt:`path pattern` is a :t:`pattern` that matches a :t:`constant`, a
:t:`unit enum variant`, or a :t:`unit struct constant` indicated by a
:t:`path`.

See :s:`PathPattern`.

path resolution
^^^^^^^^^^^^^^^

:dt:`Path resolution` is a form of :t:`resolution` that applies to a :t:`path`.

path segment
^^^^^^^^^^^^

A :dt:`path segment` is a constituent of a :t:`path`.

See :s:`PathSegment`, :s:`SimplePathSegment`, :s:`TypePathSegment`.

pattern
^^^^^^^

A :dt:`pattern` is a :t:`construct` that matches a :t:`value` which satisfies
all the criteria of the pattern.

See :s:`Pattern`.

pattern matching
^^^^^^^^^^^^^^^^

:t:`Pattern matching` is the process of matching a :t:`pattern` against a :t:`value`.

pattern-without-alternation
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`pattern-without-alternation` is a :t:`pattern` that cannot be alternated.

See :s:`PatternWithoutAlternation`.

pattern-without-range
^^^^^^^^^^^^^^^^^^^^^

A :dt:`pattern-without-range` is a :t:`pattern-without-alternation` that
excludes :t:`[range pattern]s`.

See :s:`PatternWithoutRange`.

place
^^^^^

A :dt:`place` is a location where a :t:`value` resides.

place expression
^^^^^^^^^^^^^^^^

A :dt:`place expression` is an :t:`expression` that represents a memory
location.

place expression context
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`place expression context` is a :t:`construct` that may evaluate its
operand as a memory location.

plane
^^^^^

In :t:`Unicode`, a :dt:`plane` is a continuous group of 65,536
:t:`[code point]s`.

pointer
^^^^^^^

A :dt:`pointer` is a :t:`value` of a :t:`pointer type`.

pointer type
^^^^^^^^^^^^

A :dt:`pointer type` is a :t:`type` whose values indicate memory locations.

positional register argument
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`positional register argument` is a :t:`register argument` whose
configuration is not bound to an :t:`identifier`.

precedence
^^^^^^^^^^

:dt:`Precedence` is the order by which :t:`[expression]s` are evaluated in the
presence of other expressions.

prelude
^^^^^^^

A :dt:`prelude` is a collection of :t:`entities <entity>` that are
automatically brought :t:`in scope` of every :t:`module` in a :t:`crate`.

prelude entity
^^^^^^^^^^^^^^

A :dt:`prelude entity` is an :t:`entity` declared in a :t:`prelude`.

prelude name
^^^^^^^^^^^^

A :dt:`prelude name` is a :t:`name` of a :t:`prelude entity`.

primitive representation
^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Primitive representation` is the :t:`type representation` of
:t:`[integer type]s`.

principal trait
^^^^^^^^^^^^^^^

The :dt:`principal trait` of :t:`trait object type` is its first :t:`trait bound`.

private visibility
^^^^^^^^^^^^^^^^^^

:dt:`Private visibility` is a kind of :t:`visibility` that allows a :t:`name`
to be referred to only by the current :t:`module` of the :t:`entity`, and its
descendant :t:`[module]s`.

proc-macro crate
^^^^^^^^^^^^^^^^

A :dt:`proc-macro crate` is a :t:`crate` whose :t:`crate type` is ``proc-macro``.

procedural macro
^^^^^^^^^^^^^^^^

A :dt:`procedural macro` is a :t:`macro` that encapsulates syntactic
transformations in a :t:`function`.

program entry point
^^^^^^^^^^^^^^^^^^^

A :dt:`program entry point` is a :t:`function` that is invoked at the start of
a Rust program.

public visibility
^^^^^^^^^^^^^^^^^

:dt:`Public visibility` is a kind of :t:`visibility` that allows a :t:`name`
to be referred to from arbitrary :t:`module` ``M`` as long as the ancestor
:t:`[module]s` of the related :t:`entity` can be referred to from ``M``.

punctuator
^^^^^^^^^^

A :dt:`punctuator` is a character or a sequence of characters in category
:s:`Punctuation`.

pure identifier
^^^^^^^^^^^^^^^

A :dt:`pure identifier` is an :t:`identifier` that does not include
:t:`[weak keyword]s`.

qualified path expression
^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`qualified path expression` is a :t:`path expression` that resolves
through a :t:`qualified type`.

See :s:`QualifiedPathExpression`.

qualified type
^^^^^^^^^^^^^^

A :dt:`qualified type` is a :t:`type` that is restricted to a set of
:t:`[implementation]s` that exhibit :t:`implementation conformance` to a
:t:`qualifying trait`.

See :s:`QualifiedType`.

qualified type path
^^^^^^^^^^^^^^^^^^^

A :dt:`qualified type path` is a :t:`type path` that resolves through a
:t:`qualified type`.

See :s:`QualifiedTypePath`.

qualifying trait
^^^^^^^^^^^^^^^^

A :dt:`qualifying trait` is a :t:`trait` that imposes a restriction on a
:t:`qualified type`.

See :s:`QualifyingTrait`.

range expression
^^^^^^^^^^^^^^^^

A :dt:`range expression` is an :t:`expression` that constructs a range.

See :s:`RangeExpression`.

range expression high bound
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`range expression high bound` is an :t:`operand` that specifies the end
of a range.

See :s:`RangeExpressionHighBound`.

range expression low bound
^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`range expression low bound` is an :t:`operand` that specifies the start
of a range.

See :s:`RangeExpressionLowBound`.

range pattern
^^^^^^^^^^^^^

A :dt:`range pattern` is a :t:`pattern` that matches :t:`[value]s` which fall
within a range.

See ``RangePattern``.

range pattern bound
^^^^^^^^^^^^^^^^^^^

A :dt:`range pattern bound` is a constraint on the range of a
:t:`range pattern`.

See :s:`RangePatternBound`.

range pattern high bound
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`range pattern high bound` is a :t:`range pattern bound` that specifies
the end of a range.

See :s:`RangePatternHighBound`.

range pattern low bound
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`range pattern low bound` is a :t:`range pattern bound` that specifies
the start of a range.

See :s:`RangePatternLowBound`.

range-from expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`range-from expression` is a :t:`range expression` that specifies an
included :t:`range expression low bound`.

See :s:`RangeFromExpression`.

range-from-to expression
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`range-from-to expression` is a :t:`range expression` that specifies an
included :t:`range expression low bound` and an excluded
:t:`range expression high bound`.

See :s:`RangeFromToExpression`.

range-full expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`range-full expression` is a :t:`range expression` that covers the whole
range of a :t:`type`.

See :s:`RangeFullExpression`.

range-inclusive expression
^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`range-inclusive expression` is a :t:`range expression` that specifies an
included :t:`range expression low bound` and an included
:t:`range expression high bound`.

See :s:`RangeInclusiveExpression`.

range-to expression
^^^^^^^^^^^^^^^^^^^

A :dt:`range-to expression` is a :t:`range expression` that specifies an
excluded :t:`range expression high bound`.

See :s:`RangeToExpression`.

range-to-inclusive expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`range-to-inclusive expression` is a :t:`range expression` that specifies
an included :t:`range expression high bound`.

See :s:`RangeToInclusiveExpression`.

raw borrow expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`raw borrow expression` is an :t:`expression` that creates a :t:`raw pointer` to the memory location of its :t:`operand` without incurring a :t:`borrow`.

See :s:`RawBorrowExpression`.

raw byte string literal
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`raw byte string literal` is a :t:`simple byte string literal` that does
not recognize :t:`[escaped character]s`.

See :s:`RawByteStringLiteral`.

raw c string literal
^^^^^^^^^^^^^^^^^^^^

A :dt:`raw c string literal` is a :t:`simple c string literal` that does not
recognize :t:`[escaped character]s`.

See :s:`RawCStringLiteral`.

raw pointer
^^^^^^^^^^^

A :dt:`raw pointer` is a pointer of a :t:`raw pointer type`.

raw pointer type
^^^^^^^^^^^^^^^^

A :dt:`raw pointer type` is an :t:`indirection type` without safety and
liveness guarantees.

See :s:`RawPointerTypeSpecification`.

raw string literal
^^^^^^^^^^^^^^^^^^

A :dt:`raw string literal` is a :t:`simple string literal` that does not
recognize :t:`[escaped character]s`.

See :s:`RawStringLiteral`.

reachable control flow path
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`reachable control flow path` is a control flow path that can be
taken by the execution of a program between two given points in the program.

receiver operand
^^^^^^^^^^^^^^^^

A :dt:`receiver operand` is an :t:`operand` that denotes the :t:`value` whose
:t:`method` is being invoked by a :t:`method call expression`.

See :s:`ReceiverOperand`.

receiver type
^^^^^^^^^^^^^

A :dt:`receiver type` is the :t:`type` of a :t:`receiver operand`.

record enum variant
^^^^^^^^^^^^^^^^^^^

A :dt:`record enum variant` is an :t:`enum variant` with a
:s:`RecordStructFieldList`.

record struct
^^^^^^^^^^^^^

A :dt:`record struct` is a :t:`struct` with a :s:`RecordStructFieldList`.

See :s:`RecordStructDeclaration`.

record struct field
^^^^^^^^^^^^^^^^^^^

A :dt:`record struct field` is a :t:`field` of a :t:`record struct type`.

See :s:`RecordStructField`.

record struct pattern
^^^^^^^^^^^^^^^^^^^^^

A :dt:`record struct pattern` is a :t:`pattern` that matches a
:t:`enum variant value`, a :t:`struct value`, or a :t:`union value`.

See :s:`RecordStructPattern`.

record struct type
^^^^^^^^^^^^^^^^^^

A :dt:`record struct type` is the :t:`type` of a :t:`record struct`.

record struct value
^^^^^^^^^^^^^^^^^^^

A :dt:`record struct value` is a :t:`value` of a :t:`record struct type`.

recursive type
^^^^^^^^^^^^^^

A :dt:`recursive type` is a :t:`type` that may define other types within its
:t:`type specification`.

reference
^^^^^^^^^

A :dt:`reference` is a :t:`value` of a :t:`reference type`.

reference identifier pattern
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`reference identifier pattern` is an :t:`identifier pattern` with
:t:`keyword` ``ref``.

reference pattern
^^^^^^^^^^^^^^^^^

A :dt:`reference pattern` is a :t:`pattern` that dereferences a :t:`pointer`
that is being matched.

See :s:`ReferencePattern`.

reference type
^^^^^^^^^^^^^^

A :dt:`reference type` is an :t:`indirection type` with :t:`ownership`.

See :s:`ReferenceTypeSpecification`.

referent
^^^^^^^^

A :dt:`referent` is the :t:`value` pointed-to by a :t:`reference`.

refutability
^^^^^^^^^^^^

:dt:`Refutability` is a property of :t:`[pattern]s` that expresses the ability
to match all possible :t:`[value]s` of a :t:`type`.

refutable constant
^^^^^^^^^^^^^^^^^^

A :dt:`refutable constant` is a :t:`constant` of a :t:`refutable type`.

refutable pattern
^^^^^^^^^^^^^^^^^

A :dt:`refutable pattern` is a :t:`pattern` that has a possibility of not
matching the :t:`value` it is being matched against.

refutable type
^^^^^^^^^^^^^^

A :dt:`refutable type` is a :t:`type` that has more than one :t:`value`.

register
^^^^^^^^

A :dt:`register` is a hardware component capable of holding data that can be
read and written.

register argument
^^^^^^^^^^^^^^^^^

A :dt:`register argument` is a :t:`construct` that configures the input
and output of a :t:`register`, and optionally binds the configuration to an
:t:`identifier`.

See :s:`RegisterArgument`.

register class
^^^^^^^^^^^^^^

A :dt:`register class` represents a set of :t:`[register]s`.

register class argument
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`register class argument` is a :t:`register argument` that uses a
:t:`register class name`.

register class name
^^^^^^^^^^^^^^^^^^^

A :dt:`register class name` is a target-specific string that identifies a
:t:`register class`.

See :s:`RegisterClassName`.

register expression
^^^^^^^^^^^^^^^^^^^

A :dt:`register expression` is either an :t:`input-output register expression`
or a :t:`simple register expression`.

See :s:`RegisterExpression`.

register name
^^^^^^^^^^^^^

A :dt:`register name` is either the :t:`explicit register name` of a
:t:`register`, or the :t:`register class name` of the :t:`register class` a
:t:`register` belongs to.

See :s:`RegisterName`.

register parameter
^^^^^^^^^^^^^^^^^^

A :dt:`register parameter` is a substring delimited by characters 0x7B (left
curly bracket) and 0x7D (right curly bracket) that is substituted with a
:t:`register argument` in an :t:`assembly instruction`.

register parameter modifier
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`register parameter modifier` is a substring that starts with character
0x3A (colon), follows a :t:`register parameter`, and changes the formatting of
the related :t:`register parameter`.

remainder assignment
^^^^^^^^^^^^^^^^^^^^

For :dt:`remainder assignment`, see :t:`remainder assignment expression`.

remainder assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`remainder assignment expression` is a
:t:`compound assignment expression` that uses remainder division.

See :s:`RemainderAssignmentExpression`.

remainder expression
^^^^^^^^^^^^^^^^^^^^

A :dt:`remainder expression` is an :t:`arithmetic expression` that uses
remainder division.

See :s:`RemainderExpression`.

renaming
^^^^^^^^

A :dt:`renaming` provides an alternative :t:`name` for an existing name.

See :s:`Renaming`.

repeat operand
^^^^^^^^^^^^^^

A :dt:`repeat operand` is an :t:`operand` that specifies the element being
repeated in an :t:`array repetition constructor`.

See :s:`RepeatOperand`.

repetition operator
^^^^^^^^^^^^^^^^^^^

A :dt:`repetition operator` is a :t:`construct` that indicates the number
of times a :t:`macro repetition in matching` or a
:t:`macro repetition in transcription` can be repeated.

See :s:`MacroRepetitionOperator`.

representation
^^^^^^^^^^^^^^

See :t:`type representation`.

representation modifier
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`representation modifier` is a :t:`construct` that modifies the
:t:`alignment` of a :t:`type`.

See :s:`Alignment`.

reserved keyword
^^^^^^^^^^^^^^^^

A :dt:`reserved keyword` is a :t:`keyword` that is not yet in use.

See :s:`ReservedKeyword`.

resolution
^^^^^^^^^^

:dt:`Resolution` is the process of finding a unique interpretation for a
:t:`field access expression`, a :t:`method call expression`, or a :t:`path`.

rest pattern
^^^^^^^^^^^^

A :dt:`rest pattern` is a :t:`pattern` that matches zero or more elements that
have not already been matched.

See :s:`RestPattern`.

return expression
^^^^^^^^^^^^^^^^^

A :dt:`return expression` is an :t:`expression` that optionally yields a
:t:`value` and causes control flow to return to the caller.

See :s:`ReturnExpression`.

return type
^^^^^^^^^^^

A :dt:`return type` is the :t:`type` of the result a :t:`function` returns.

See :s:`ReturnType`.

right operand
^^^^^^^^^^^^^

A :dt:`right operand` is an :t:`operand` that appears on the right-hand side of
a :t:`binary operator`.

See :s:`RightOperand`.

rule matching
^^^^^^^^^^^^^

:dt:`Rule matching` is the process of consuming a :s:`TokenTree` in an attempt
to fully satisfy the :t:`macro matcher` of a :t:`macro rule` that belongs to a
resolved :t:`declarative macro`.

rustc
^^^^^

:dt:`rustc` is a compiler that implements the FLS.

safety invariant
^^^^^^^^^^^^^^^^

A :dt:`safety invariant` is an invariant that when violated may result in
:t:`undefined behavior`.

scalar type
^^^^^^^^^^^

A :dt:`scalar type` is either a :c:`bool` :t:`type`, a :c:`char` :t:`type`, or
a :t:`numeric type`.

scope
^^^^^

A :dt:`scope` is a region of program text where a :t:`name` can be referred to.

scope hierarchy
^^^^^^^^^^^^^^^

The :dt:`scope hierarchy` reflects the nesting of :t:`[scope]s` as introduced
by :t:`[scoping construct]s`.

selected field
^^^^^^^^^^^^^^

A :dt:`selected field` is a :t:`field` that is selected by a
:t:`field access expression`.

Self
^^^^

:dc:`Self` is either an implicit :t:`type parameter` in :t:`[trait]s` or an
implicit :t:`type alias` in :t:`[implementation]s`. :c:`Self` refers to the
:t:`type` that implements a :t:`trait`.

self parameter
^^^^^^^^^^^^^^

A :dt:`self parameter` is a :t:`function parameter` expressed by :t:`keyword`
``self``.

self public modifier
^^^^^^^^^^^^^^^^^^^^

A :dt:`self public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`private visibility`.

See :s:`SelfPublicModifier`.

Self scope
^^^^^^^^^^

A :dt:`Self scope` is a :t:`scope` for :c:`Self`.

send type
^^^^^^^^^

A :dt:`send type` is a :t:`type` that implements the :std:`core::marker::Send`
:t:`trait`.

separator
^^^^^^^^^

A :dt:`separator` is a character or a string that separates adjacent
:t:`[lexical element]s`.

sequence type
^^^^^^^^^^^^^

A :dt:`sequence type` represents a sequence of elements.

shadowing
^^^^^^^^^

:dt:`Shadowing` is a property of :t:`[name]s`. A :t:`name` is said to be
:dt:`shadowed` when another :t:`name` with the same characters is introduced
in the same :t:`scope` within the same :t:`namespace`, effectively hiding it.

shared borrow
^^^^^^^^^^^^^

A :dt:`shared borrow` is a :t:`borrow` produced by evaluating an
:t:`immutable borrow expression`.

shared reference
^^^^^^^^^^^^^^^^

A :dt:`shared reference` is a :t:`value` of a :t:`shared reference type`.

shared reference type
^^^^^^^^^^^^^^^^^^^^^

A :dt:`shared reference type` is a :t:`reference type` not subject to
:t:`keyword` ``mut``.

shift left assignment
^^^^^^^^^^^^^^^^^^^^^

For :dt:`shift left assignment`, see :t:`shift left assignment expression`.

shift left assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`shift left assignment expression` is a
:t:`compound assignment expression` that uses bit shift left arithmetic.

See :s:`ShiftLeftAssignmentExpression`.

shift left expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`shift left expression` is a :t:`bit expression` that uses bit shift left
arithmetic.

See :s:`ShiftLeftExpression`.

shift right assignment
^^^^^^^^^^^^^^^^^^^^^^

For :dt:`shift right assignment`, see :t:`shift right assignment expression`.

shift right assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`shift right assignment expression` is a
:t:`compound assignment expression` that uses bit shift right arithmetic.

See :s:`ShiftRightAssignmentExpression`.

shift right expression
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`shift right expression` is a :t:`bit expression` that uses bit shift
right arithmetic.

See :s:`ShiftRightExpression`.

shorthand deconstructor
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`shorthand deconstructor` is a :t:`construct` that matches the :t:`name`
of a :t:`field` and binds the :t:`value` of the matched :t:`field` to a
:t:`binding`.

See :s:`ShorthandDeconstructor`.

shorthand initializer
^^^^^^^^^^^^^^^^^^^^^

A :dt:`shorthand initializer` is a :t:`construct` that specifies the :t:`name`
of a :t:`field` in a :t:`struct expression`.

See :s:`ShorthandInitializer`.

signed integer type
^^^^^^^^^^^^^^^^^^^

A :dt:`signed integer type` is an :t:`integer type` whose :t:`[value]s` denote
negative whole numbers, zero, and positive whole numbers.

simple byte string literal
^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`simple byte string literal` is a :t:`byte string literal` that consists
of multiple :s:`[AsciiCharacter]s`.

See :s:`SimpleByteStringLiteral`.

simple c string literal
^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`simple c string literal` is any :t:`Unicode` character except characters
0x0D (carriage return), 0x22 (quotation mark), 0x5C (reverse solidus) and 0x00
(null byte).

See :s:`SimpleCStringLiteral`.

simple import
^^^^^^^^^^^^^

A :dt:`simple import` is a :t:`use import` that binds a :t:`simple path` to a
local :t:`name` by using an optional :t:`renaming`.

See :s:`SimpleImport`.

simple path
^^^^^^^^^^^

A :dt:`simple path` is a :t:`path` whose :t:`[path segment]s` consist of either
:t:`[identifier]s` or certain :t:`[keyword]s`.

See :s:`SimplePath`.

simple path prefix
^^^^^^^^^^^^^^^^^^

A :dt:`simple path prefix` is the leading :t:`simple path` of a
:t:`glob import` or a :t:`nesting import`.

See :s:`SimplePathPrefix`.

simple path public modifier
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`simple path public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility` within the provided :t:`simple path` only.

See :s:`SimplePathPublicModifier`.

simple path resolution
^^^^^^^^^^^^^^^^^^^^^^

:dt:`Simple path resolution` is a kind of :t:`path resolution` that applies to
a :t:`simple path`.

simple public modifier
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`simple public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility`.

See :s:`SelfPublicModifier`.

simple register expression
^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`simple register expression` is either an :t:`expression` or an
:t:`underscore expression`.

See :s:`SimpleRegisterExpression`.

simple string literal
^^^^^^^^^^^^^^^^^^^^^

A :dt:`simple string literal` is a :t:`string literal` where the characters are
:t:`Unicode` characters.

See :s:`SimpleStringLiteral`.

single segment path
^^^^^^^^^^^^^^^^^^^

A :dt:`single segment path` is a :t:`path` consisting of exactly one
:t:`path segment`.

size
^^^^

The :dt:`size` of a :t:`value` is the offset in bytes between successive
elements in an :t:`array type` with the same :t:`element type`, including any
padding for :t:`alignment`.

size operand
^^^^^^^^^^^^

A :dt:`size operand` is an :t:`operand` that specifies the size of an
:t:`array` or an :t:`array type`.

See :s:`SizeOperand`.

sized type
^^^^^^^^^^

A :dt:`sized type` is a :t:`type` with statically known size.

slice
^^^^^

A :dt:`slice` is a :t:`value` of a :t:`slice type`.

slice pattern
^^^^^^^^^^^^^

A :dt:`slice pattern` is a :t:`pattern` that matches :t:`[array]s` of fixed
size and :t:`[slice]s` of dynamic size.

See :s:`SlicePattern`.

slice type
^^^^^^^^^^

A :dt:`slice type` is a :t:`sequence type` that provides a view into a sequence
of elements.

See :s:`SliceTypeSpecification`.

source file
^^^^^^^^^^^

A :dt:`source file` contains the program text of :t:`[inner attribute]s`,
:t:`[inner doc comment]s`, and :t:`[item]s`.

See :s:`SourceFile`.

statement
^^^^^^^^^

A :dt:`statement` is a component of a block expression.

See :s:`Statement`.

static
^^^^^^

A :dt:`static` is a :t:`value` that is associated with a specific memory
location.

See :s:`StaticDeclaration`.

static initializer
^^^^^^^^^^^^^^^^^^

A :dt:`static initializer` is a :t:`construct` that provides the :t:`value` of
its related :t:`static`.

See :s:`StaticInitializer`.

static lifetime elision
^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Static lifetime elision` is a form of :t:`lifetime elision` that applies
to :t:`[constant]s` and :t:`[static]s`.

str
^^^

:dc:`str` is a :t:`sequence type` that represents a :t:`slice` of 8-bit
unsigned bytes.

strict keyword
^^^^^^^^^^^^^^

A :dt:`strict keyword` is a :t:`keyword` that always holds its special meaning.

See :s:`StrictKeyword`.

string literal
^^^^^^^^^^^^^^

A :dt:`string literal` is a :t:`literal` that consists of multiple characters.

See :s:`StringLiteral`.

struct
^^^^^^

A :dt:`struct` is an :t:`item` that declares a :t:`struct type`.

struct expression
^^^^^^^^^^^^^^^^^

A :dt:`struct expression` is an :t:`expression` that constructs an
:t:`enum value`, a :t:`struct value`, or a :t:`union value`.

See :s:`StructExpression`.

struct field
^^^^^^^^^^^^

A :dt:`struct field` is a :t:`field` of a :t:`struct type`.

struct pattern
^^^^^^^^^^^^^^

A :dt:`struct pattern` is a :t:`pattern` that matches an :t:`enum value`, a
:t:`struct value`, or a :t:`union value`.

See :s:`StructPattern`.

struct type
^^^^^^^^^^^

A :dt:`struct type` is an :t:`abstract data type` that is a product of other
:t:`[type]s`.

See :s:`StructDeclaration`.


struct value
^^^^^^^^^^^^

A :dt:`struct value` is a :t:`value` of a :t:`struct type`.

structurally equal
^^^^^^^^^^^^^^^^^^

A :t:`type` is :dt:`structurally equal` when its :t:`[value]s` can be compared
for equality by structure.

subexpression
^^^^^^^^^^^^^

A :dt:`subexpression` is an :t:`expression` nested within another
:t:`expression`.

subject expression
^^^^^^^^^^^^^^^^^^

A :dt:`subject expression` is an :t:`expression` that controls
:t:`[for loop]s`, :t:`[if expression]s`, and :t:`[match expression]s`.

See :s:`SubjectExpression`.

subject let expression
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`subject let expression` is an :t:`expression` that controls
:t:`[if let expression]s` and :t:`[while let loop]s`.

See :s:`SubjectLetExpression`.

subpattern
^^^^^^^^^^

A :dt:`subpattern` is a :t:`pattern` nested within another :t:`pattern`.

subtraction assignment
^^^^^^^^^^^^^^^^^^^^^^

For :dt:`subtraction assignment`, see :t:`subtraction assignment`.

subtraction assignment expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`subtraction assignment expression` is a
:t:`compound assignment expression` that uses subtraction.

See :s:`SubtractionAssignmentExpression`.

subtraction expression
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`subtraction expression` is an :t:`arithmetic expression` that uses
subtraction.

See :s:`SubtractionExpression`.

subtrait
^^^^^^^^

A :dt:`subtrait` is a :t:`trait` with a :t:`supertrait`.

subtype
^^^^^^^

A :dt:`subtype` is a :t:`type` with additional constraints.

subtyping
^^^^^^^^^

:dt:`Subtyping` is a property of :t:`[type]s`, allowing one :t:`type` to be
used where another :t:`type` is expected.

suffixed float
^^^^^^^^^^^^^^

A :dt:`suffixed float` is a :t:`float literal` with a :t:`float suffix`.

suffixed integer
^^^^^^^^^^^^^^^^

A :dt:`suffixed integer` is an :t:`integer literal` with an :t:`integer suffix`.

super public modifier
^^^^^^^^^^^^^^^^^^^^^

A :dt:`super public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility` within the parent :t:`module` only.

See :s:`SuperPublicModifier`.

supertrait
^^^^^^^^^^

A :dt:`supertrait` is a transitive :t:`trait` that a :t:`type` must
additionally implement.

sync type
^^^^^^^^^

A :dt:`sync type` is a :t:`type` that implements the :std:`core::marker::Sync`
:t:`trait`.

syntactic category
^^^^^^^^^^^^^^^^^^

A :dt:`syntactic category` is a nonterminal in the Backus-Naur Form grammar
definition of the Rust programming language.

tail expression
^^^^^^^^^^^^^^^

A :dt:`tail expression` is the last :t:`expression` within a
:t:`block expression`.

temporary
^^^^^^^^^

A :dt:`temporary` is an anonymous :t:`variable` produced by some intermediate
computation.

terminated
^^^^^^^^^^

A :t:`loop expression` is :dt:`terminated` when its :t:`block expression` is no
longer evaluated.

terminated macro invocation
^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`terminated macro invocation` is a :t:`macro invocation` that may be used
as a :t:`statement`.

See :s:`TerminatedMacroInvocation`.

textual macro scope
^^^^^^^^^^^^^^^^^^^

A :dt:`textual macro scope` is a :t:`scope` for :t:`[declarative macro]s`.

textual type
^^^^^^^^^^^^

A :dt:`textual type` is a :t:`type` class that includes type :c:`char` and type
:c:`str`.

thin pointer
^^^^^^^^^^^^

A :dt:`thin pointer` is a :t:`value` of a :t:`thin pointer type`.

thin pointer type
^^^^^^^^^^^^^^^^^

A :dt:`thin pointer type` is an :t:`indirection type` that refers to a
:t:`fixed sized type`.

token matching
^^^^^^^^^^^^^^

:dt:`Token matching` is the process of consuming a :s:`TokenTree` in an attempt
to fully satisfy a :t:`macro match` of a selected :t:`macro matcher` that
belongs to a resolved :t:`declarative macro`.

tokens
^^^^^^

:dt:`[Token]s` are a subset of :t:`[lexical element]s` consumed by
:t:`[macro]s`.

trait
^^^^^

A :dt:`trait` is an :t:`item` that describes an interface a :t:`type` can
implement.

See :s:`TraitDeclaration`.

trait body
^^^^^^^^^^

A :dt:`trait body` is a :t:`construct` that encapsulates the
:t:`[associated item]s`, :t:`[inner attribute]s`, and
:t:`[inner doc comment]s` of a :t:`trait`.

See :s:`TraitBody`.

trait bound
^^^^^^^^^^^

A :dt:`trait bound` is a :t:`bound` that imposes a constraint on the
:t:`[trait]s` of :t:`[generic parameter]s`.

See :s:`TraitBound`.

trait implementation
^^^^^^^^^^^^^^^^^^^^

A :dt:`trait implementation` is an :t:`implementation` that adds functionality
specified by a :t:`trait`.

See :s:`TraitImplementation`.

trait object lifetime elision
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Trait object lifetime elision` is a form of :t:`lifetime elision` that
applies to :t:`[trait object type]s`.

trait object type
^^^^^^^^^^^^^^^^^

A :dt:`trait object type` is a :t:`type` that implements a :t:`trait`, where
the :t:`type` is not known at compile time.

See :s:`TraitObjectTypeSpecification`,
:s:`TraitObjectTypeSpecificationOneBound`.

trait type
^^^^^^^^^^

A :dt:`trait type` is either an :t:`impl trait type` or a
:t:`trait object type`.

transparent representation
^^^^^^^^^^^^^^^^^^^^^^^^^^

:dt:`Transparent representation` is a :t:`type representation` that applies
only to an :t:`enum type` with a single :t:`enum variant` or a :t:`struct type`
where the :t:`struct type` or :t:`enum variant` has a single :t:`field` of
non-zero :t:`size` and any number of :t:`[field]s` of :t:`size` zero and
:t:`alignment` one.

trivial predicate
^^^^^^^^^^^^^^^^^

A :dt:`trivial predicate` is a :t:`where clause predicate` that does not use
the :t:`[generic parameter]s` or :t:`[higher-ranked trait bound]s` of the related
:t:`construct`.

tuple
^^^^^

A :dt:`tuple` is a :t:`value` of a :t:`tuple type`.

tuple enum variant
^^^^^^^^^^^^^^^^^^

A :dt:`tuple enum variant` is an :t:`enum variant` with a
:s:`TupleStructFieldList`.

tuple enum variant value
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`tuple enum variant value` is a :t:`value` of a :t:`tuple enum variant`.

tuple expression
^^^^^^^^^^^^^^^^

A :dt:`tuple expression` is an :t:`expression` that constructs a :t:`tuple`.

See :s:`TupleExpression`.

tuple field
^^^^^^^^^^^

A :dt:`tuple field` is a :t:`field` of a :t:`tuple type`.

tuple initializer
^^^^^^^^^^^^^^^^^

A :dt:`tuple initializer` is an :t:`operand` that provides the :t:`value` of a
:t:`tuple field` in a :t:`tuple expression`.

tuple pattern
^^^^^^^^^^^^^

A :dt:`tuple pattern` is a :t:`pattern` that matches a :t:`tuple` which
satisfies all criteria defined by its :t:`[subpattern]s`.

See :s:`TuplePattern`.

tuple struct
^^^^^^^^^^^^

A :dt:`tuple struct` is a :t:`struct` with a :s:`TupleStructFieldList`.

See :s:`TupleStructDeclaration`.

tuple struct call expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`tuple struct call expression` is a :t:`call expression` where the
:t:`call operand` resolves to a :t:`tuple struct` or a :t:`tuple enum variant`.

tuple struct field
^^^^^^^^^^^^^^^^^^

A :dt:`tuple struct field` is a :t:`field` of a :t:`tuple struct type`.

See :s:`TupleStructField`.

tuple struct pattern
^^^^^^^^^^^^^^^^^^^^

A :dt:`tuple struct pattern` is a :t:`pattern` that matches a
:t:`tuple enum variant value` or a :t:`tuple struct value`.

See :s:`TupleStructPattern`.

tuple struct type
^^^^^^^^^^^^^^^^^

A :dt:`tuple struct type` is the :t:`type` of a :t:`tuple struct`.

tuple struct value
^^^^^^^^^^^^^^^^^^

A :dt:`tuple struct value` is a :t:`value` of a :t:`tuple struct type`.

tuple type
^^^^^^^^^^

A :dt:`tuple type` is a :t:`sequence type` that represents a heterogeneous list
of other :t:`[type]s`.

See :s:`TupleTypeSpecification`.

type
^^^^

A :dt:`type` defines a set of :t:`[value]s` and a set of operations that act on
those :t:`[value]s`.

type alias
^^^^^^^^^^

A :dt:`type alias` is an :t:`item` that defines a :t:`name` for a :t:`type`.

See :s:`TypeAliasDeclaration`.

type argument
^^^^^^^^^^^^^

A :dt:`type argument` is a :t:`generic argument` that supplies the :t:`value`
of a :t:`type parameter`.

See :s:`TypeArgument`.

type ascription
^^^^^^^^^^^^^^^

A :dt:`type ascription` specifies the :t:`type` of a :t:`construct`.

See :s:`TypeAscription`.

type bound predicate
^^^^^^^^^^^^^^^^^^^^

A :dt:`type bound predicate` is a :t:`construct` that specifies
:t:`[lifetime bound]s` and :t:`[trait bound]s` on a :t:`type`.

See :s:`TypeBoundPredicate`.

type cast expression
^^^^^^^^^^^^^^^^^^^^

A :dt:`type cast expression` is an :t:`expression` that changes the :t:`type`
of an :t:`operand`.

See :s:`TypeCastExpression`.

type coercion
^^^^^^^^^^^^^

:dt:`Type coercion` is an implicit operation that changes the :t:`type` of
a :t:`value`.

type inference
^^^^^^^^^^^^^^

:dt:`Type inference` is the process of deducing the expected :t:`type` of an
arbitrary :t:`value`.

type inference root
^^^^^^^^^^^^^^^^^^^

A :dt:`type inference root` is a :t:`construct` whose inner :t:`[expression]s`
and :t:`[pattern]s` are subject to :t:`type inference` independently of other
:t:`[type inference root]s`.

type parameter
^^^^^^^^^^^^^^

A :dt:`type parameter` is a :t:`generic parameter` for a :t:`type`.

See :s:`TypeParameter`.

type parameter initializer
^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`type parameter initializer` is a :t:`construct` that provides the
default :t:`value` of its related :t:`type parameter`.

See :s:`TypeParameterInitializer`.

type parameter type
^^^^^^^^^^^^^^^^^^^

A :dt:`type parameter type` is a placeholder :t:`type` of a :t:`type parameter`
to be substituted by :t:`generic substitution`.

type path
^^^^^^^^^

A :dt:`type path` is a :t:`path` that acts as a :t:`type specification`.

See :s:`TypePath`.

type path resolution
^^^^^^^^^^^^^^^^^^^^

:dt:`Type path resolution` is a form of :t:`path resolution` that applies to
a :t:`type path`.

type representation
^^^^^^^^^^^^^^^^^^^

:dt:`Type representation` specifies the :t:`layout` of :t:`[field]s` of
:t:`[abstract data type]s`.

type specification
^^^^^^^^^^^^^^^^^^

A :dt:`type specification` describes the structure of a :t:`type`.

See :s:`TypeSpecification`.

type unification
^^^^^^^^^^^^^^^^

:dt:`Type unification` is the process by which :t:`type inference` propagates
known :t:`[type]s` across the :t:`type inference root` and assigns concrete
:t:`[type]s` to :t:`[type variable]s`, as well as a general mechanism to check
for compatibility between two :t:`[type]s` during :t:`method resolution`.

type variable
^^^^^^^^^^^^^

A :dt:`type variable` is a placeholder used during :t:`type inference` to stand
in for an undetermined :t:`type` of an :t:`expression` or a :t:`pattern`.

u8
^^

:dc:`u8` is an :t:`unsigned integer type` whose :t:`[value]s` range from 0 to
2\ :sup:`8` - 1, all inclusive.

u16
^^^

:dc:`u16` is an :t:`unsigned integer type` whose :t:`[value]s` range from 0 to
2\ :sup:`16` - 1, all inclusive.

u32
^^^

:dc:`u32` is an :t:`unsigned integer type` whose :t:`[value]s` range from 0 to
2\ :sup:`32` - 1, all inclusive.

u64
^^^

:dc:`u64` is an :t:`unsigned integer type` whose :t:`[value]s` range from 0 to
2\ :sup:`64` - 1, all inclusive.

u128
^^^^

:dc:`u128` is an :t:`unsigned integer type` whose :t:`[value]s` range from 0 to
2\ :sup:`128` - 1, all inclusive.

unary operator
^^^^^^^^^^^^^^

A :dt:`unary operator` operates on one :t:`operand`.

undefined behavior
^^^^^^^^^^^^^^^^^^

:dt:`Undefined behavior` is a situation that results in an unbounded error.

under resolution
^^^^^^^^^^^^^^^^

A :t:`construct` that is being resolved is said to be :dt:`under resolution`.

underscore expression
^^^^^^^^^^^^^^^^^^^^^

An :dt:`underscore expression` is an :t:`expression` that acts as a placeholder
in a :t:`destructuring assignment`.

See :s:`UnderscoreExpression`.

underscore pattern
^^^^^^^^^^^^^^^^^^

An :dt:`underscore pattern` is a :t:`pattern` that matches any single
:t:`value`.

See :s:`UnderscorePattern`.

unhygienic
^^^^^^^^^^

An :t:`identifier` is :dt:`unhygienic` when it has :t:`call site hygiene`.

Unicode
^^^^^^^

:dt:`Unicode` is the universal character encoding standard for written
characters and text described in the Unicode® Standard by the Unicode
Consortium.

unifiable
^^^^^^^^^

For :dt:`unifiable`, see :t:`unify`.

unifiable types
^^^^^^^^^^^^^^^

Two :t:`[type]s` that :t:`unify` are said to be :dt:`[unifiable type]s`.

unified type
^^^^^^^^^^^^

A :dt:`unified type` is a :t:`type` produced by :t:`type unification`.

unify
^^^^^

A :t:`type` is said to :dt:`unify` with another type when the domains, ranges,
and structures of both :t:`[type]s` are compatible.

union
^^^^^

A :dt:`union` is an :t:`item` that declares a :t:`union type`.

union field
^^^^^^^^^^^

A :dt:`union field` is a :t:`field` of a :t:`union type`.

union type
^^^^^^^^^^

A :dt:`union type` is an :t:`abstract data type` similar to a :t:`C`-like union.

See :s:`UnionDeclaration`.

union value
^^^^^^^^^^^

A :dt:`union value` is a :t:`value` of a :t:`union type`.

unique immutable reference
^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`unique immutable reference` is an :t:`immutable reference` produced by
:t:`capturing` what is asserted to be the only live :t:`reference` to a
:t:`value` while the :t:`reference` exists.

unit enum variant
^^^^^^^^^^^^^^^^^

A :dt:`unit enum variant` is an :t:`enum variant` without a :t:`field list`.

unit struct
^^^^^^^^^^^

A :dt:`unit struct` is a :t:`struct` without a :t:`field list`.

See :s:`UnitStructDeclaration`.

unit struct constant
^^^^^^^^^^^^^^^^^^^^

A :dt:`unit struct constant` is a :t:`constant` implicitly created by a
:t:`unit struct`.

unit struct type
^^^^^^^^^^^^^^^^

A :dt:`unit struct type` is the :t:`type` of a :t:`unit struct`.

unit struct value
^^^^^^^^^^^^^^^^^

A :dt:`unit struct value` is a :t:`value` of a :t:`unit struct type`.

unit tuple
^^^^^^^^^^

A :dt:`unit tuple` is a :t:`value` of the :t:`unit type`.

unit type
^^^^^^^^^

The :dt:`unit type` is a :t:`tuple type` of zero :t:`arity`.

unit value
^^^^^^^^^^

The :dt:`unit value` is the :t:`value` of a :t:`unit type`.

unnamed constant
^^^^^^^^^^^^^^^^

An :dt:`unnamed constant` is a :t:`constant` declared with character 0x5F (low
line).

unnamed lifetime
^^^^^^^^^^^^^^^^

An :dt:`unnamed lifetime` is a :t:`lifetime` declared with character 0x5F (low
line).

unqualified path expression
^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`unqualified path expression` is a :t:`path expression`  without a :t:`qualified type`.

unsafe block
^^^^^^^^^^^^

For :dt:`unsafe block`, see :t:`unsafe block expression`.

unsafe block expression
^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`unsafe block expression` is a :t:`block expression` that is specified
with :t:`keyword` ``unsafe``.

See :s:`UnsafeBlockExpression`.

unsafe context
^^^^^^^^^^^^^^

An :dt:`unsafe context` is either an :t:`unsafe block` or an
:t:`unsafe function`.

unsafe external block
^^^^^^^^^^^^^^^^^^^^^

An :dt:`unsafe external block` is an :t:`external block` subject to keyword ``unsafe``.

unsafe function
^^^^^^^^^^^^^^^

An :dt:`unsafe function` is a :t:`function` subject to :t:`keyword` ``unsafe``.

unsafe function item type
^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`unsafe function item type` is a :t:`function item type` where the
related :t:`function` is an :t:`unsafe function`.

unsafe function pointer type
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`unsafe function pointer type` is a function pointer type subject to
:t:`keyword` ``unsafe``.

unsafe operation
^^^^^^^^^^^^^^^^

An :dt:`unsafe operation` is an operation that may result in
:t:`undefined behavior` that is not diagnosed as a static error.
:t:`[Unsafe operation]s` are referred to as :t:`unsafe Rust`.

unsafe Rust
^^^^^^^^^^^

For :dt:`unsafe Rust`, see :t:`[unsafe operation]s`.

unsafe trait
^^^^^^^^^^^^

An :dt:`unsafe trait` is a :t:`trait` subject to :t:`keyword` ``unsafe``

unsafe trait implementation
^^^^^^^^^^^^^^^^^^^^^^^^^^^

An :dt:`unsafe trait implementation` is a :t:`trait implementation` subject to
:t:`keyword` ``unsafe``.

unsafety
^^^^^^^^

:dt:`Unsafety` is the presence of :t:`[unsafe operation]s` in program text.

unsigned integer type
^^^^^^^^^^^^^^^^^^^^^

An :dt:`unsigned integer type` is an :t:`integer type` whose :t:`[value]s`
denote zero and positive whole numbers.

unsized coercion
^^^^^^^^^^^^^^^^

An :dt:`unsized coercion` is a :t:`type coercion` that converts a
:t:`sized type` into an :t:`unsized type`.

unsized type
^^^^^^^^^^^^

An :dt:`unsized type` is a :t:`type` with statically unknown size.

unsuffixed float
^^^^^^^^^^^^^^^^

An :dt:`unsuffixed float` is a :t:`float literal` without a :t:`float suffix`.

unsuffixed integer
^^^^^^^^^^^^^^^^^^

An :dt:`unsuffixed integer` is an :t:`integer literal` without an
:t:`integer suffix`.

use capture
^^^^^^^^^^^

An :dt:`use capture` is a :t:`generic parameter` referenced via keyword $$use$$ within an :t:`anonymous return type`.

See :s:`UseCaptures`.

use import
^^^^^^^^^^

A :dt:`use import` brings :t:`entities <entity>` :t:`in scope` within the
:t:`block expression` of an :t:`expression-with-block` or :t:`module` where the
:t:`use import` resides.

See :s:`UseImport`.

usize
^^^^^

:dc:`usize` is an :t:`unsigned integer type` with the same number of bits as
the platform's :t:`pointer type`, and is at least 16-bits wide.

validity invariant
^^^^^^^^^^^^^^^^^^

A :dt:`validity invariant` is an invariant that when violated results in
immediate :t:`undefined behavior`.

value
^^^^^

A :dt:`value` is either a :t:`literal` or the result of a computation, that may
be stored in a memory location, and interpreted based on some :t:`type`.

value expression
^^^^^^^^^^^^^^^^

A :dt:`value expression` is an :t:`expression` that represents a :t:`value`.

value expression context
^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`value expression context` is an expression context that is not a
:t:`place expression context`.

value operand
^^^^^^^^^^^^^

A :dt:`value operand` is an :t:`operand` that supplies the :t:`value` that is
assigned to an :t:`assignee operand` by an :t:`assignment expression`.

See :s:`ValueOperand`.

variable
^^^^^^^^

A :dt:`variable` is a placeholder for a :t:`value` that is allocated on the
stack.

variadic part
^^^^^^^^^^^^^

A :dt:`variadic part` indicates the presence of :t:`C`-like optional
parameters.

See :s:`VariadicPart`.

variance
^^^^^^^^

:dt:`Variance` is a property of :t:`[lifetime parameter]s` and
:t:`[type parameter]s` that describes the circumstances under which a
:t:`generic type` is a :t:`subtype` of an instantiation of itself with
different :t:`[generic argument]s`.

visibility
^^^^^^^^^^

:dt:`Visibility` is a property of :t:`[field]s` and :t:`[item]s` that determines
which :t:`[module]s` can refer to the :t:`name` of the :t:`field` or :t:`item`.

visibility modifier
^^^^^^^^^^^^^^^^^^^

A :dt:`visibility modifier` sets the :t:`visibility` of the :t:`name` of an
:t:`item`.

visible emptiness
^^^^^^^^^^^^^^^^^

:dt:`Visible emptiness <visible emptiness>` is a property of :t:`[type]s` and :t:`[enum variant]s` that have no :t:`[value]s` that are fully observable.

visible empty enum variant
^^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`visible empty enum variant` is an :t:`enum variant` subject to :t:`visible emptiness`.

visible empty type
^^^^^^^^^^^^^^^^^^

A :dt:`visible empty type` is a :t:`type` subject to :t:`visible emptiness`.

weak keyword
^^^^^^^^^^^^

A :dt:`weak keyword` is a :t:`keyword` whose special meaning depends on the
context.

See :s:`WeakKeyword`.

where clause
^^^^^^^^^^^^

A :dt:`where clause` is a :t:`construct` that specifies :t:`[bound]s` on
:t:`[lifetime parameter]s` and :t:`[type parameter]s`.

See :s:`WhereClause`.

where clause predicate
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`where clause predicate` is either a :t:`lifetime bound predicate` or a
:t:`type bound predicate`.

See :s:`WhereClausePredicate`.

while let loop
^^^^^^^^^^^^^^

For :dt:`while let loop`, see :t:`while let loop expression`.

while let loop expression
^^^^^^^^^^^^^^^^^^^^^^^^^

A :dt:`while let loop expression` is a :t:`loop expression` that continues to
evaluate its :t:`loop body` as long as its :t:`subject let expression` yields a
:t:`value` that can be matched against its :t:`pattern`.

See :s:`WhileLetLoopExpression`.

while loop
^^^^^^^^^^

For :dt:`while loop`, see :t:`while loop expression`.

while loop expression
^^^^^^^^^^^^^^^^^^^^^

A :dt:`while loop expression` is a :t:`loop expression` that continues to
evaluate its :t:`loop body` as long as its :t:`iteration expression` holds
true.

See :s:`WhileLoopExpression`.

whitespace string
^^^^^^^^^^^^^^^^^

A :dt:`whitespace string` is a string that consists of one or more
:t:`[whitespace character]s`.

zero-sized type
^^^^^^^^^^^^^^^

A :dt:`zero-sized type` is a :t:`fixed sized type` with :t:`size` zero.

zero-variant enum type
^^^^^^^^^^^^^^^^^^^^^^

A :dt:`zero-variant enum type` is an :t:`enum type` without any
:t:`[enum variant]s`.
