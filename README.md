# Humus
## A composting database

### Introduction

> "Put everything on the rotchain, the very mutable ledger where things change constantly and kind of just disappear over time."
_[(Anna Prendergrast (@Aprndrgrst), 18th November 2021)](https://twitter.com/APndrgrst/status/1461239757246136321)_

This is a small experiment in deliberate data decay.  a simple in-memory database that slowly forgets the things that you don't revisit.

It's a side project from the sprawling mutual-aid skill exchange I'm doing with [@jcalpickard](https://github.com/jcalpickard) that we're calling [_Enxaneta_](https://github.com/timcowlishaw/enxaneta).

The idea is that it forms a part of the infrastructure for some broader experiments on the affordances of deliberate data loss: A [counter-factual speculation](https://dl.acm.org/doi/fullHtml/10.1145/3577212) on digital technnologies that [refuse the logic](https://arxiv.org/abs/2010.08850) of scale and accumulation, and is very much inspired by [permacomputing](http://permacomputing.net/).

It's also an experiment in writing software as both [Bogostian Carpentry](https://quote.ucsd.edu/sed/files/2016/04/Bogost.pdf) and [Critical Technical Practice](https://pages.gseis.ucla.edu/faculty/agre/critical.html) - simultaneously using a process of making to create objects that-speak-for-themselves, and reflexively incorporating the things i learn into my broader practice as a programmer.

### Installation

#### Building from source:

Clone this repository, ensure you have rust and cargo installed, and run `cargo run`.

#### Using a pre-built executable:

Download the executable for your platform from the [Releases page](https://github.com/timcowlishaw/humus/releases) and run it.

#### Using cargo:

```bash
cargo install humus
#Then to run:
humus
```

### Using docker:

```bash
docker pull timcowlishaw/humus:latest
# Then to run:
docker run -p 3030:3030 timcowlishaw/humus
```

### Usage

To use, you can `POST` a json blob to any path on the server, for instance:

```bash
curl --location --request POST localhost:3030/path/to/the/entity --header 'Content-Type: application/json' --data-raw '{"attribute": "value"}'
```

You can retrieve them again by GETing the entity url:

```bash
curl localhost:3030/path/to/the/entity
# => [{"attribute": "value"}]
```

You'll note that this comes back as an element in an array. This is because you can post multiple objects to the same path:

```bash
curl --location --request POST localhost:3030/path/to/the/entity --header 'Content-Type: application/json' --data-raw '{"attribute": "value2"}'
```

```bash
curl localhost:3030/path/to/the/entity
#=> [{"attribute": "value"}, {"attribute": "value2"}]
```

The paths are laid out like a tree structure, where GETing a path gets all the objects underneath it. For instance, if we also add one at a sibling (or, i guess, like "cousin") URL:

```bash
curl --location --request POST localhost:3030/path/to/another/different/entity --header 'Content-Type: application/json' --data-raw '{"attribute": "value3"}
```
...then we get one of the ancestor paths....

```bash
curl localhost:3030/path
#=> [{"attribute": "value"}, {"attribute": "value2"}, {"attribute": "value3"}]
```

... we get ALL the entities stored under that path.

### Configuration

The server listens on the port and interface given in `HUMUS_SOCKET_ADDRESS` - the default is `127.0.0.1:3030`.

Each entity attribute will persist for a number of seconds defined in the `HUMUS_LIFETIME` environment variable (default: 60), from the last moment it was either updated or accessed.

There's also an environment variable switch called `HUMUS_REFRESH_CHILD_ENTITIES`, which is, by default, unset. If this is set to any value, then accessing for instance `/path` will also refresh the "lifetime" of all the entities stored at paths underneath it (eg `/path/subpath`). If not, you need to access an object directly *at the url at which it was stored* to reset the decay timer - accessing a parent path will show it, but it will decay as if you had never accessed it. If this is confusing, don't worry about it. This option will probably go away very soon once i've worked out what works best.

If you want to see more of what's going on internally, set the `RUST_LOG` env var to `debug` or `trace`.

### Future plans

Soon:
- Write a python client library!
- Add a configurable amount of noise to the lifetime of objects
- Decay objects more weirdly - gradually and nonlinearly add an increasing amount of noise to attributes of stored objects, that sort of thing.
- Actually persist the data to disk, in a way that doesn't mess up our strong guarantees of transience.
- Make the "lifetime" configurable per object or path.
- Allow a single object to appear at multiple paths.

Sometime later:
- Give the database a configurable capacity limit and see what happens.
. Allow explicit deletion of objects with DELETE requests
- Allow updating of an entity with PUT requests


