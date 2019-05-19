CodeMirror.defineSimpleMode("rommap", {
    // The start state contains the rules that are intially used
    start: [
      // The regex matches the token, the token property contains the type
      {regex: /\${\w+}/, token: "string"},
      {regex: /x|X/, token: "atom"},
      {regex: /0|1/, token: "variable-2"},
      {regex: /\w+/, token: "keyword"},
      {regex: /;.*/, token: "comment"},
    ],
    // The multi-line comment state.
    comment: [],
    // The meta property contains global information about the mode. It
    // can contain properties like lineComment, which are supported by
    // all modes, and also directives like dontIndentStates, which are
    // specific to simple modes.
    meta: {
      dontIndentStates: ["comment"],
      lineComment: ";"
    }
  });