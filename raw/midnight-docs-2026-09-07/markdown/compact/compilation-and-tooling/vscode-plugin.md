> For the complete documentation index, see [llms.txt](/llms.txt)

# Visual Studio Code extension for Compact

The \[Visual Studio Code extension] for Compact is a plugin that assists with writing and debugging smart contracts written in Midnight's Compact language.

DApp developers can create new smart contracts

* using a file template
* from scratch, optionally using code snippets.

## Installation[​](#installation "Direct link to Installation")

Install via the \[Releases page] (<https://raw.githubusercontent.com/midnight-ntwrk/releases/gh-pages/artifacts/vscode-extension/compact-0.2.13/compact-0.2.13.vsix>)

## Features[​](#features "Direct link to Features")

### Syntax highlighting[​](#syntax-highlighting "Direct link to Syntax highlighting")

Smart contracts are written in the Compact language. The following language elements will be recognized and formatted:

* Compact keywords, such as `enum`, `struct`, and `circuit`
* string, boolean, and numeric literals
* comments
* parentheses

![Syntax highlighting](/assets/images/compact-syntax-a76e9fa96b86a7476a71cb0a2eb51e26.png)

### Building Compact source files and error highlighting[​](#building-compact-source-files-and-error-highlighting "Direct link to Building Compact source files and error highlighting")

For building smart contracts, you will typically want to add a script definition in `package.json`, like so:

```
"scripts": {

  "compact": "compact compile --vscode ./src/myContract.compact ./src/managed/myContract"

}
```

The preceding assumes that the Compact compiler is on the shell's command search path (if it isn't, follow the instructions provided in [Running Midnight Compact compiler](/develop/tutorial/building/counter-build#compile-the-code)). It also uses the Compact compiler's --vscode option to omit newlines in error messages, so that they render properly inside the VS Code environment.

This allows you to compile your smart contract using:

```
yarn compact
```

When you edit complex contracts, and you want to have fast interactions, it is more convenient to add a [task file](https://code.visualstudio.com/docs/editor/tasks) `.vscode/tasks.json` configured like this:

```
{

  "version": "2.0.0",

  "tasks": [

    {

      "label": "Compile compact file to JS",

      "type": "shell",

      "command": "npx compact compile --vscode --skip-zk ${file} ${workspaceFolder}/src/managed",

      "group": "build",

      "presentation": {

        "echo": true,

        "reveal": "never",

        "focus": false,

        "panel": "shared",

        "showReuseMessage": false,

        "clear": true,

        "revealProblems": "onProblem"

      },

      "problemMatcher": [

        "$compactException",

        "$compactInternal",

        "$compactCommandNotFound"

      ]

    }

  ]

}
```

The preceding configuration uses the Compact compiler flag `--skip-zk` to skip the generation of circuits, which can take a long time and may be unnecessary when you only want to the compiler to check your syntax.

Moreover, you can configure a `problemMatcher` for VS Code:

```
 "problemMatcher": [

     "$compactException",

     "$compactInternal",

     "$compactCommandNotFound"

 ]
```

so that all errors reported by the compiler will be shown on the **Problems** tab.

### Code snippets[​](#code-snippets "Direct link to Code snippets")

The VS Code extension provides the following [code snippets](https://code.visualstudio.com/docs/editor/userdefinedsnippets) when editing Compact smart contracts:

* `ledger` (`state`)
* `constructor` in ledger
* potentially exported `circuit` (`function` / `transition`)
* `witness` (`private` function)
* constructor
* import Compact `standard library` (`init`, `stdlib`)
* `if` statement (`cond`)
* `map` (`for`)
* `fold`
* `enum`
* `struct`
* `module`
* `assert`
* `pragma`

There is also a `compact` template, which will generate a simple Compact skeleton for a contract.

![Code snippets and errors](/assets/images/code-snippets-errors-6017f43d451ce0ec358c4a8c1fdfa6ae.gif)

### New Compact smart contract[​](#new-compact-smart-contract "Direct link to New Compact smart contract")

If you are creating DApp, you can create a new smart contract with an empty ledger and a single circuit as follows:

1. Bring up the commands pallette (Cmd+Shift+P).
2. Select **Snippets: Fill File with Snippet**.
3. Select **Compact**. Be aware: if you perform these steps in an existing file, the contents of the file are overwritten. Other snippets are available from inside the file. Just start typing the name of the function.

![File template](/assets/images/file-template-948ccce3632ac1881582a420a797e703.gif)

## License[​](#license "Direct link to License")

The Visual Studio Code extension for Compact is distributed under the Apache 2.0 license.
