---
theme: neversink
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: https://cover.sli.dev
# some information about your slides (markdown enabled)
title: Algebraic Enums
author: Shannon Cavanagh
date: 11/08/2025
info: |
  ## Slidev Starter Template
  Presentation slides for developers.

  Learn more at [Sli.dev](https://sli.dev)
# apply unocss classes to the current slide
class: text-center
# https://sli.dev/features/drawing
drawings:
  persist: false
# enable MDC Syntax: https://sli.dev/features/mdc
mdc: true
layout: cover 
neversink_slug: How I Learned To Stop Worrying And Love The Null | Rochester Developers 2025
transition: slide-up

---

# Algebraic Enums

<p style="font-size:1em; letter-spacing:3px; line-height: 1.8; margin-top:-0.6em;">
Or  <br/>
How  <br/>
I Learned  <br/>
To  <br/>
Stop  <br/>
Worrying  <br/>
And  <br/>
Love  <br/>
The  <br/>
Null
</p>
::note::
Shannon Cavanagh - 11/08/2025 - Rochester Developers

---

```yaml
layout: two-cols-title
layoutClass: gap-16
transition: slide-right
color: teal
```

::title::

# Outline

::left::

  
1. Author
1. Concept
1. Context
1. Non-Templating
1. Templating
1. Error Inverting

::right::

---

```yaml
layout: image-left
image: https://media1.tenor.com/m/enfY28nVoOMAAAAd/smiling-friends-pim.gif
transition: slide-down
color: blue
```

# About the Author

- 20 years programming experience
- 8 years robotics experience
- All your language are belong to us
- Written vulkan renderers >100 times
- Uneducated drop-out savant

<p style="margin-top:10em">
<span style="margin-right:2em; display: inline-flex; align-items: center;">
<span style="margin-right:0.2em;">
@jonf.blog
</span>
<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-bluesky" viewBox="0 0 16 16">
  <path d="M3.468 1.948C5.303 3.325 7.276 6.118 8 7.616c.725-1.498 2.698-4.29 4.532-5.668C13.855.955 16 .186 16 2.632c0 .489-.28 4.105-.444 4.692-.572 2.04-2.653 2.561-4.504 2.246 3.236.551 4.06 2.375 2.281 4.2-3.376 3.464-4.852-.87-5.23-1.98-.07-.204-.103-.3-.103-.218 0-.081-.033.014-.102.218-.379 1.11-1.855 5.444-5.231 1.98-1.778-1.825-.955-3.65 2.28-4.2-1.85.315-3.932-.205-4.503-2.246C.28 6.737 0 3.12 0 2.632 0 .186 2.145.955 3.468 1.948"/>
</svg>
</span>
<span style="display: inline-flex; align-items: center;">
<span style="margin-right:0.2em;">
jonf@jonf.blog
</span>
<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-envelope-at-fill" viewBox="0 0 16 16">
  <path d="M2 2A2 2 0 0 0 .05 3.555L8 8.414l7.95-4.859A2 2 0 0 0 14 2zm-2 9.8V4.698l5.803 3.546zm6.761-2.97-6.57 4.026A2 2 0 0 0 2 14h6.256A4.5 4.5 0 0 1 8 12.5a4.49 4.49 0 0 1 1.606-3.446l-.367-.225L8 9.586zM16 9.671V4.697l-5.803 3.546.338.208A4.5 4.5 0 0 1 12.5 8c1.414 0 2.675.652 3.5 1.671"/>
  <path d="M15.834 12.244c0 1.168-.577 2.025-1.587 2.025-.503 0-1.002-.228-1.12-.648h-.043c-.118.416-.543.643-1.015.643-.77 0-1.259-.542-1.259-1.434v-.529c0-.844.481-1.4 1.26-1.4.585 0 .87.333.953.63h.03v-.568h.905v2.19c0 .272.18.42.411.42.315 0 .639-.415.639-1.39v-.118c0-1.277-.95-2.326-2.484-2.326h-.04c-1.582 0-2.64 1.067-2.64 2.724v.157c0 1.867 1.237 2.654 2.57 2.654h.045c.507 0 .935-.07 1.18-.18v.731c-.219.1-.643.175-1.237.175h-.044C10.438 16 9 14.82 9 12.646v-.214C9 10.36 10.421 9 12.485 9h.035c2.12 0 3.314 1.43 3.314 3.034zm-4.04.21v.227c0 .586.227.8.581.8.31 0 .564-.17.564-.743v-.367c0-.516-.275-.708-.572-.708-.346 0-.573.245-.573.791"/>
</svg>
</span>
</p>

---

```yaml
layout: top-title-two-cols
columns: is-8
align: l-lt-lt
class: text-center
transition: slide-left
color: teal-light
```

$$
\text{Dog} = \text{Bark} \times \text{Bite} \times \text{Fur}\\
\text{Color} = \text{Red} + \text{Blue} + \text{Green}
$$

::title::

# Concept

::left::

1. A struct is a _composite_ of its parts.
   1. Promises its layout contains all these datums.
1. An enum is exclusively only _one_ of its parts.
   1. This is intuitive to most languages.
   1. An integer can only be one number at a time.
   1. Why limit ourselves to just integers?

::right::

<<< @/snippets/sum_vs_composite.rs rust {0-0|1-5|7-11}

<!--
Talk about structs, [CLICK] Code example
Mention enums [CLICK] Code example, =THEN TALK=. Last line is joke.
"SO...."
-->

---

```yaml
layout: quote
transition: slide-up
```

<img src="https://media1.tenor.com/m/BukqcvkwSqsAAAAC/glue-gorilla-glue.gif" class="mx-auto w-1/2" alt="Do not put gorilla glue in your hair" />

<!--
"Why limit ourselves *at all*?"
Who's going to stop you, the man??
-->

---

```yaml
layout: top-title
transition: slide-right
color: red
```

::title::
# Context
::content::

<v-clicks>

- Popular from ML, like all good things about Typing.
    - Staple in functional world
    - For solving theorems, compiler enforced all possibilities handled.
    - This became Pattern Matching,
    - Replace State with rich data
- Languages from 2010+ are finally maturing
    - Swift & Rust trying to improving writing code, not just 'code'.
- Exceptions are dead.
    - Long live exceptions.

</v-clicks>

<!--
ML stands for 'Meta Language'
  Standard ML standardized this in 1983.
  Other languages include Miranda, Haskell, OCaml
  A more complete type system allows stricter compiler promises
  and a more complete safety system.
  Parallel programming has made the stack frame more and more important.
  Panics & unwinding are a lot less acceptable.
  By "throwing more exceptions" ( complex sum types of errors), we can throw less exceptions.
-->

---

```yaml
layout: section
transition: slide-down
color: sky-light
```

# Code

<v-click>

![lets go already](https://media1.tenor.com/m/oVYyASiqzxwAAAAC/futurama-let%27s-go.gif)

</v-click>

<!--
Examples will be in the RUST programming language, which should be mostly similar to SWIFT / any ALGOL languages.
-->

---

```yaml
layout: top-title
transition: fade
color: red
```

::title::

# Non-templating Sum

::content::

<<< @/snippets/nontemplating_example.rs rust {1,5|1-3,5,7,14|1-3,5,7-10,13-14|1,4-8,11-14|}

<!--
We're going to go over making a basic enum, 
When I say 'templating' I mean this enum has only one purpose.
In most languages with SUM types, we don't specify the int values for these states, as for the most powerful pattern matching, the compiler needs to be able to promise non-overlapping behavior.
( if you need ints, just have a translation function it's fine )
[CLICK]
Here's that pattern matching in action. Many languages have this now.
[CLICK]
A SUM type can itself, contain a composite structure. This can be a struct, int, anything. The compiler implementation will decide how this is laid out in memory. Normally a deciding byte but might not be!
In the pattern matching we're allowed access into this payload. Unlike UNIONS or loose enums there's no room for programmer error here.
Don't forget u have a cute guy next slide
-->

---

```yaml
layout: top-title
transition: fade
color: red
```

::title::

# Non-templating Sum

::content::

<<< @/snippets/nontemplating_example.rs rust {}

<span style="display: inline-flex; align-items: center;margin-top:2em">
<Mug mood="excited" size="100"/>
<SpeechBubble  style="margin-left:1em;" position="l" color="sky" shape="round" animation="float" maxWidth="500px">
Here, we can tell the difference between CardPlays with no datums and something with a payload!
</SpeechBubble>

</span>

---

```yaml
layout: top-title-two-cols
columns: is-5
align: l-lt-lt
color: amber
```

::title::

# Templating

::left::
<<< @/snippets/simple_templating.rs rust {1-6|1-6,9,13|}
::right::
<<< @/snippets/simple_templating2.rs rust {0-0|1,10|1-2,9-10|1,3,7,9-10|1,8,9-10|}

<StickyNote style="margin-top:1em; margin-left:2em;" color="amber-light" textAlign="left" width="180px" title="Hmmm..">
  This looks a lot like Nullable T,<br/><br/>
  Which is a <i>composite</i> of data or no-data
</StickyNote>

<!-- 
Here it is. Ultimate power 
For pointer types or things with bit niches, this is ZERO COST.
[CLICK]
Here we have a more casual pattern match. We're targeting one specific pattern and extracting its payload
[CLICK][CLICK]
This can make error-handling explicit. In every language with SUM types,
it almost always starts with optionals and errors.
With, for instance, nullable pointers, you have to (aka, SHOULD) check ur pointers are not null on EVERY function signature. Because.
By representing a null reference as an option, when we have a function signature WITHOUT Option<>, no-one will be allowed to call it with None ( aka null )
[CLICK]
db.read() might be null. We're going to handle that, so that the functions that work on this data don't need to check it.
[CLICK] If not null, we can execute this branch confidently.
[Click etc]
-->

---

```yaml
layout: top-title-two-cols
columns: is-6
align: r-lt-lt
color: rose-light
```

::title::

# Complex Non-Templating

::left::

## Ideals

1. Anything we can lay out in a struct,
   we wish to lay out in an sum type.
   1. Generics, v-tables, references, heap data
1. Compiler will decide on a discriminant
   1. Sometimes this is thin!
   1. Sometimes the compiler can reason without it!
1. Eventually as natural as a struct
   <Mug style="float:right;margin-right:2em;margin-top:2em;" color="#FFA2AB" size="50" mood="shocked"/>

::right::
<<< @/snippets/complex_templating1.rs rust {1-6,14,16-21|1-4,7,14,16-21|1-4,8-11,14,16-21|1-4,12-14,16-99}

<!--
Read lines. What can we put in here? Anything you truly want for good reasons.
Compiler will reason about bit blits. In some cases on the stack, it won't 
Let's start with pazaak! Every card has a value from one-to-ten, and some way that value affects the board. Let's break it down. They all share these fields,
and they all have only exactly one of these!
[CLICK]
In some cases, we'll need more information.
[CLICK]
In rust at least, the line between tuples and structs is fuzzy. We can just go ahead and name complex fields.
[CLICK]
Here I've got an unknown function pointer we lookup at runtime in a vtable. ( probably would be an inherited class normally but.. )
-->

---

```yaml
layout: side-title
align: rm-lm
transition: fade
color: indigo
```

::title::

# Error Inverting

::content::

- The Triangle of doom!
- I don't care what you think, rightward drift is _real_.
- The most common SUM in programming is..

  <AdmonitionType v-click type="warning" title="" width="350px" v-drag>
  <i>User Input!</i><br/>
  includes.. Integrity, Reasoning, and State
  </AdmonitionType>

<!--
Who's heard of the triangle of doom?
It starts with one "if not null"... then inside that, check the next if not null..
SUM types can help combat this. In particular, the most common thing
SUM can resolve is... [CLICK]
-->

---

```yaml
layout: two-cols-title
columns: is-4
align: l-lt-lt
transition: fade
titlepos: t
color: green-light
```

::title::

# Error Inverting and Complex Templating

::left::
<<< @/snippets/error_inverting1.rs rust {6-10|1-4|}
::right::
<<< @/snippets/error_inverting2.rs rust {0-0|1,12-14|1-4,12-14|5-7,12-14|8-12,12-14|}
<Mug style="margin-bottom:4em;bottom:0px;position:absolute;" mood="happy" size="100"/>

<!--
Here's a common one. Nothing quite looking up fault codes in a giant int ENUM.
[CLICK]
Let's look carefully at this one. This is a common shape, if the result can be proven to be OK, then it can also be proven to be T.
If it can be proven to be Err, then it can be proven to be E. It's a real type safe union. The is a LOT that can be done, but let's just start with this example from the standard lib.
[CLICK]just a thing to return[CLICK]
We might return a Customer, or we might return an error.
Lots of functions involving IO or user input will do this. This way
we can force the caller to handle errors.
If things go good, we'll return an OK result with some kind of customer.
[CLICK]
Here we go. We do some work. It looks to me that source-read might have a result, or it might not find anything. We'll call that a pipe panic.
Aside - simple functions probably don't need tons of error codes. That's left up to good taste.
[CLICK]
After some more work, we can check more complex conditions 
-->

---

```yaml
layout: two-cols-title
columns: is-4
align: l-lt-lt
transition: fade
titlepos: t
color: green-light
```

::title::

# Error Inverting and Complex Templating

::left::
<<< @/snippets/error_inverting1.rs rust {1,4-6,10-12,14,16}
::right::
<<< @/snippets/error_inverting3.rs rust {2-12}

<Mug style="margin-bottom:4em;bottom:0px;position:absolute;" mood="happy" size="100"/>

---

```yaml
layout: two-cols-title
columns: is-4
align: l-lt-lt
transition: fade
titlepos: t
color: green-light
```

::title::

# Error Inverting and Complex Templating

::left::
<<< @/snippets/error_inverting1.rs rust {1,4-6,10-12,14,16}
::right::
<<< @/snippets/error_inverting4.rs rust {2-9}

<Mug style="margin-bottom:4em;bottom:0px;position:absolute;" mood="happy" size="100"/>

---

```yaml
layout: two-cols-title
columns: is-4
align: l-lt-lt
transition: slide-up
titlepos: t
color: green-light
```

::title::

# Error Inverting and Complex Templating

::left::
<<< @/snippets/error_inverting1.rs rust
::right::
<<< @/snippets/error_inverting4.rs rust

<Mug style="margin-bottom:4em;bottom:0px;position:absolute;" mood="happy" size="100"/>

---

```yaml
layout: full
class: text-center
color: orange-light
```

# End!

<img src="https://media1.tenor.com/m/ncyU2W-KyPYAAAAd/obi-wan-bar.gif" class="mx-auto w-1/2" style="margin-top:5em" alt="Obi-wan Bar" />
