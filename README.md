# Useless ECS in Rust

I just wanted to understand how downcasting can be implemented. So I created this.

## What

It has a World struct that contains all entities and storages. Each entity is simply `Entity(u32)`
and each storage is stored in a HashMap.

In the HashMap, `TypeId` is used as key and the value is `Box<dyn Storage>`. The `dyn Storage` can be
downcasted to a concrete storage `StorageImpl<C>`, where `C: Component + 'static`.

You can then get (imm)mutable access to storages and components.

Removing entities is not yet implemented.

## How can I use it in my project?

It's best if you don't.
