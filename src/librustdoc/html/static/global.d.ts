interface Window {
    initSearch: (rawSearchIndex: SearchIndex) => void;

    initSidebarItems: (items: SidebarItems) => void;
    sidebarCurrent: SidebarCurrent;

    rootPath: string;
    currentCrate: string;

    pending_implementors: Implementors;
    register_implementors: (implementors: Implementors) => void;
    inlined_types: Set<string>;
}

declare var rootPath: string;
declare var currentCrate: string;
declare var resourcesSuffix: string;

interface Implementor {
    text: string;
    synthetic: boolean;
    types: string[];
}

interface Implementors {
    [crate: string]: Implementor[];
}

interface SidebarItems {
    [item_type: string]: [string, string][];
}

interface SidebarCurrent {
    name: string;
    ty: string;
    relpath: string;
}

interface SearchIndex {
    [crate: string]: {
        doc: string;
        items: [number, string, string, string, number, SearchFunctionType][]
        paths: [number, string][];
    }
}
interface Alias {
    crate: string;
    ty: number;
    name: string;
    desc: string;
    p: string;
}

interface CrateAliases {
    [alias: string]: Alias[];
}

interface Aliases {
    [crate: string]: CrateAliases;
}

declare var ALIASES: Aliases;

interface SearchQuery {
    raw: string;
    query: string;
    type: string;
    id: string;
}

interface SearchResults {
    in_args: SearchItem[];
    returned: SearchItem[];
    others: SearchItem[];
}

interface SearchResultsMultiple {
    in_args: SearchItem[][];
    returned: SearchItem[][];
    others: SearchItem[][];
}

interface SearchItem {
    crate: string;
    ty: number;
    name: string;
    path: string;
    desc?: string;
    parent?: SearchParent;
    type?: SearchFunctionType;
    lev?: number;
    is_alias?: boolean;
    href?: string;
    displayPath?: string;
    alias?: string;
    fullPath?: string;
}

interface SearchFunctionArgument {
    n: string;
    g?: string[];
}

interface SearchFunctionType {
    i?: SearchFunctionArgument[];
    o?: SearchFunctionArgument;
}

interface SearchParent {
    ty: number;
    name: string;
}

interface SearchResultIndex {
    id: number;
    index: number;
    lev?: number;
    dontValidate?: boolean;
    word?: string;
    item?: SearchItem;
}

interface SearchResultIndecies {
    [fullId: string]: SearchResultIndex;
}

interface SearchGenerics {
    name: string;
    generics: string[];
}

interface QueryStringParams {
    [query: string]: string;
}
