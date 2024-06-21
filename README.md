# Humus
## A composting database

> "Put everything on the rotchain, the very mutable ledger where things change constantly and kind of just disappear over time."
_[(Anna Prendergrast (@Aprndrgrst), 18th November 2021)](https://twitter.com/APndrgrst/status/1461239757246136321)_

This is a small experiment in deliberate data decay.  a simple (at present) in memory entity-attribute-value database that slowly forgets the things that you don't revisit.

It's a side project from the sprawling mutual-aid skill exchange I'm doing with [@jcalpickard](https://github.com/jcalpickard) that we're calling [_Enxaneta_](https://github.com/timcowlishaw/enxaneta).

The idea is that it forms a part of the infrastructure for some broader experiments on the affordances of deliberate data loss: A [counter-factual speculation](https://dl.acm.org/doi/fullHtml/10.1145/3577212) on digital technnologies that [refuse the logic](https://arxiv.org/abs/2010.08850) of scale and accumulation, and is very much inspired by [permacomputing](http://permacomputing.net/).

It's also an experiment in writing software as both [Bogostian Carpentry](https://quote.ucsd.edu/sed/files/2016/04/Bogost.pdf) and [Critical Technical Practice](https://pages.gseis.ucla.edu/faculty/agre/critical.html) - simultaneously using a process of making to create objects that-speak-for-themselves, and reflexively incorporating the results into my practice.

It's written in Rust, so you'll need cargo installed, and you run it simply by running (`cargo run`). It's an HTTP REST API that runs on localhost port 3030.

Entites are json blobs (whose values, at the moment, are strings), you can store them by PUTing any url like:

`curl --location --request PUT localhost:3030/entity --header 'Content-Type: application/json' --data-raw '{"attribute": "value"}'`

You can retrieve them again by GETing the entity url:

`curl localhost:3030/entity`.

Each entity attribute will persist for a number of seconds defined in the `LIFETIME` constant in `store.rs` from the last moment it was either updated or accessed.

My plans for further development:

- Make the lifetime and port configurable at runtime
- Allow any json object to be stored as an entity
. Allow explicit deletion of objects with DELETE requests
- Allow updating of part of an entity with PATCH requests
- Add a configurable amount of noise to the lifetime of objects
- Make objects decay in a manner that's non-linear the number of accesses or updates
- Actually persist the data to disk, in a way that doesn't mess up our guarantees of transience.
- Give the database a configurable capacity limit and see what happens.
- Better build / deploy instructions and options (I'll probably add a Dockerfile or something).

If you're inspired to build anything on top of this, please feel free - I'm very interested in the affordances a weird, forgetful database like this might have for development of other applications.

Also, this is my first Rust project, and i'd welcome any feedback or suggestions :-)

Thank you!

Tim

