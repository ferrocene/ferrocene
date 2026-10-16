//@ known-bug: #130411
//@ edition: 2015
trait Project {
    const SELF: Self;
}

fn take1(_: Project<SELF = {}>) {}
