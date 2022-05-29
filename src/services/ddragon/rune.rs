use std::collections::HashMap;

// TODO: write deserializers for this mess of a json object

struct RuneObject {
    data: RuneTree,
}

type RuneTreeList = HashMap<u32, RuneTree>;

type RuneTree = HashMap<String, Vec<RuneSlot>>;

struct RuneSlot {
    data: RuneList,
}

type RuneList = HashMap<u32, String>;
